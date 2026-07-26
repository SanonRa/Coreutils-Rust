// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

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
    println!("Usage: basename NAME [SUFFIX]");
    println!("  or:  basename OPTION... NAME...");
    println!("Print NAME with any leading directory components removed.");
    println!("If specified, also remove a trailing SUFFIX.\n");
    println!("  -a, --multiple     support multiple arguments and treat each as a NAME");
    println!("  -s, --suffix=SUFFIX remove a trailing SUFFIX; implies -a");
    println!("  -z, --zero         end each output line with NUL, not newline");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    println!("\nExamples:");
    println!("  basename /usr/bin/sort          -> \"sort\"");
    println!("  basename include/stdio.h .h     -> \"stdio\"");
    println!("  basename -s .h include/stdio.h  -> \"stdio\"");
    println!("  basename -a any/str1 any/str2   -> \"str1\" followed by \"str2\"");
    print_help_epilogue("basename");
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

fn get_base_name(path: &str) -> String {
    let bytes = path.as_bytes();
    if bytes.is_empty() {
        return String::new();
    }
    
    let prefix_len = file_system_prefix_len(path);
    let mut end = bytes.len();
    
    // Skip trailing slashes, but not past the drive prefix
    while end > prefix_len && (bytes[end - 1] == b'/' || bytes[end - 1] == b'\\') {
        end -= 1;
    }
    
    // If the path was all slashes after the prefix
    if end == prefix_len {
        let rest_len = bytes.len() - prefix_len;
        if rest_len == 2 && (bytes[prefix_len] == b'/' || bytes[prefix_len] == b'\\') && (bytes[prefix_len + 1] == b'/' || bytes[prefix_len + 1] == b'\\') {
            return path[..prefix_len + 2].to_string();
        }
        if rest_len > 0 {
            return format!("{}{}", &path[..prefix_len], if bytes[prefix_len] == b'\\' { "\\" } else { "/" });
        }
        return path[..prefix_len].to_string();
    }
    
    // Find the last component separator
    let mut i = end;
    while i > prefix_len {
        i -= 1;
        if bytes[i] == b'/' || bytes[i] == b'\\' {
            return path[i + 1..end].to_string();
        }
    }
    
    path[prefix_len..end].to_string()
}

fn is_root(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if bytes.iter().all(|&b| b == b'/' || b == b'\\') {
        return true;
    }
    let prefix_len = file_system_prefix_len(name);
    if prefix_len > 0 && bytes[prefix_len..].iter().all(|&b| b == b'/' || b == b'\\') {
        return true;
    }
    false
}

fn remove_suffix(name: &mut String, suffix: &str) {
    if !suffix.is_empty() && name.ends_with(suffix) && name != suffix {
        name.truncate(name.len() - suffix.len());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut multiple_names = false;
    let mut use_nuls = false;
    let mut suffix = String::new();
    let mut names = Vec::new();
    
    let mut print_h = false;
    let mut print_v = false;
    
    // Custom option parsing
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            print_h = true;
            break;
        } else if arg == "--version" {
            print_v = true;
            break;
        } else if arg == "-a" || arg == "--multiple" {
            multiple_names = true;
        } else if arg == "-z" || arg == "--zero" {
            use_nuls = true;
        } else if arg == "-s" || arg == "--suffix" {
            if i + 1 < args.len() {
                suffix = args[i + 1].clone();
                multiple_names = true;
                i += 1;
            } else {
                eprintln!("basename: option requires an argument -- 's'");
                eprintln!("Try 'basename --help' for more information.");
                process::exit(1);
            }
        } else if arg.starts_with("-s") {
            suffix = arg[2..].to_string();
            multiple_names = true;
        } else if arg.starts_with('-') && arg != "-" && !arg.starts_with("--") {
            let mut invalid = None;
            for c in arg.chars().skip(1) {
                match c {
                    'a' => multiple_names = true,
                    'z' => use_nuls = true,
                    _ => {
                        invalid = Some(c);
                        break;
                    }
                }
            }
            if let Some(c) = invalid {
                eprintln!("basename: invalid option -- '{}'", c);
                eprintln!("Try 'basename --help' for more information.");
                process::exit(1);
            }
        } else if arg == "--" {
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
        print_version("basename", &["David MacKenzie"]);
        process::exit(0);
    }
    
    if names.is_empty() {
        eprintln!("basename: missing operand");
        eprintln!("Try 'basename --help' for more information.");
        process::exit(1);
    }
    
    if !multiple_names {
        if names.len() == 2 {
            suffix = names[1].clone();
            names.truncate(1);
        } else if names.len() > 2 {
            eprintln!("basename: extra operand '{}'", names[2]);
            eprintln!("Try 'basename --help' for more information.");
            process::exit(1);
        }
    }
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let separator = if use_nuls { b'\0' } else { b'\n' };
    
    for name_arg in names {
        let mut name = get_base_name(&name_arg);
        if !is_root(&name) {
            remove_suffix(&mut name, &suffix);
        }
        if let Err(e) = handle.write_all(name.as_bytes()) {
            eprintln!("basename: {}", e);
            process::exit(1);
        }
        if let Err(e) = handle.write_all(&[separator]) {
            eprintln!("basename: {}", e);
            process::exit(1);
        }
    }
}
