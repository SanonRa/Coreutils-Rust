use std::env;
use std::io::{self, Write};
use std::process;

const VERSION: &str = "9.11";

fn print_version(program_name: &str, authors: &[&str]) {
    println!("{} (GNU coreutils) {}", program_name, VERSION);
    println!("Copyright (C) 2026 Free Software Foundation, Inc.");
    println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.");
    println!("This is free software: you are free to change and redistribute.");
    println!("There is NO WARRANTY, to the extent permitted by law.\n");
    if !authors.is_empty() {
        if authors.len() == 1 {
            println!("Written by {}.", authors[0]);
        } else {
            print!("Written by ");
            for (i, author) in authors.iter().enumerate() {
                if i > 0 {
                    if i == authors.len() - 1 {
                        print!(" and ");
                    } else {
                        print!(", ");
                    }
                }
                print!("{}", author);
            }
            println!(".");
        }
    }
}

fn print_help_epilogue(program_name: &str) {
    println!("\nGNU coreutils online help: <https://www.gnu.org/software/coreutils/>");
    println!("Full documentation <https://www.gnu.org/software/coreutils/{}>", program_name);
    println!("or available locally via: info '(coreutils) {} invocation'", program_name);
}

fn print_help() {
    println!("Usage: dirname [OPTION] NAME...");
    println!("Output each NAME with its last non-slash component and trailing slashes");
    println!("removed; if NAME contains no /'s, output '.' (meaning the current directory).\n");
    println!("  -z, --zero        end each output line with NUL, not newline");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    println!("\nExamples:");
    println!("  dirname /usr/bin/          -> \"/usr\"");
    println!("  dirname dir1/str dir2/str  -> \"dir1\" followed by \"dir2\"");
    println!("  dirname stdio.h            -> \".\"");
    print_help_epilogue("dirname");
}

fn file_system_prefix_len(path: &str) -> usize {
    let bytes = path.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' {
        let drive = bytes[0];
        if drive.is_ascii_alphabetic() {
            return 2;
        }
    }
    0
}

fn dir_len(path: &str) -> usize {
    let bytes = path.as_bytes();
    if bytes.is_empty() {
        return 0;
    }
    
    let prefix_len = file_system_prefix_len(path);
    let mut end = bytes.len();
    
    // Skip trailing slashes, but not past the drive prefix
    while end > prefix_len && (bytes[end - 1] == b'/' || bytes[end - 1] == b'\\') {
        end -= 1;
    }
    
    // If the path was all slashes after the prefix (e.g. "C:\")
    if end == prefix_len {
        let rest_len = bytes.len() - prefix_len;
        if rest_len == 2 && (bytes[prefix_len] == b'/' || bytes[prefix_len] == b'\\') && (bytes[prefix_len + 1] == b'/' || bytes[prefix_len + 1] == b'\\') {
            return prefix_len + 2;
        }
        if rest_len > 0 {
            return prefix_len + 1;
        }
        return prefix_len;
    }
    
    // Find the last component separator
    let mut i = end;
    while i > prefix_len {
        i -= 1;
        if bytes[i] == b'/' || bytes[i] == b'\\' {
            let mut dir_end = i;
            while dir_end > prefix_len && (bytes[dir_end - 1] == b'/' || bytes[dir_end - 1] == b'\\') {
                dir_end -= 1;
            }
            if dir_end == prefix_len {
                // The directory part is all slashes (e.g. "C:\usr" or "/usr")
                let mut leading_slashes = 0;
                while prefix_len + leading_slashes < bytes.len() && (bytes[prefix_len + leading_slashes] == b'/' || bytes[prefix_len + leading_slashes] == b'\\') {
                    leading_slashes += 1;
                }
                if leading_slashes == 2 {
                    return prefix_len + 2;
                }
                return prefix_len + 1;
            }
            return dir_end;
        }
    }
    
    prefix_len
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut use_nuls = false;
    let mut names = Vec::new();
    let mut print_h = false;
    let mut print_v = false;
    let mut invalid_opt = None;
    
    // Simple custom option parsing to match GNU behavior and avoid complex dependencies
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            print_h = true;
            break;
        } else if arg == "--version" {
            print_v = true;
            break;
        } else if arg == "--zero" {
            use_nuls = true;
        } else if arg.starts_with('-') && arg != "-" && !arg.starts_with("--") {
            // Short options
            for c in arg.chars().skip(1) {
                if c == 'z' {
                    use_nuls = true;
                } else {
                    invalid_opt = Some(c);
                    break;
                }
            }
            if invalid_opt.is_some() {
                break;
            }
        } else if arg == "--" {
            // End of options
            names.extend(args[i+1..].iter().cloned());
            break;
        } else {
            names.extend(args[i..].iter().cloned());
            break;
        }
        i += 1;
    }
    
    if print_h {
        print_help();
        process::exit(0);
    }
    if print_v {
        print_version("dirname", &["David MacKenzie", "Jim Meyering"]);
        process::exit(0);
    }
    
    if let Some(c) = invalid_opt {
        eprintln!("dirname: invalid option -- '{}'", c);
        eprintln!("Try 'dirname --help' for more information.");
        process::exit(1);
    }
    
    if names.is_empty() {
        eprintln!("dirname: missing operand");
        eprintln!("Try 'dirname --help' for more information.");
        process::exit(1);
    }
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let separator = if use_nuls { b'\0' } else { b'\n' };
    
    for name in names {
        let mut len = dir_len(&name);
        let mut result = name.as_str();
        
        if len == 0 {
            result = ".";
            len = 1;
        }
        
        if let Err(e) = handle.write_all(&result.as_bytes()[..len]) {
            eprintln!("dirname: {}", e);
            process::exit(1);
        }
        if let Err(e) = handle.write_all(&[separator]) {
            eprintln!("dirname: {}", e);
            process::exit(1);
        }
    }
}
