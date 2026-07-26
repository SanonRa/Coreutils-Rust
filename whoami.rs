use std::env;
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
    println!("Usage: whoami [OPTION]...");
    println!("Print the user name associated with the current effective user ID.");
    println!("Same as id -un.\n");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    print_help_epilogue("whoami");
}

#[cfg(unix)]
fn get_username() -> Result<String, String> {
    use std::ffi::CStr;
    unsafe {
        let uid = libc::geteuid();
        let pw = libc::getpwuid(uid);
        if pw.is_null() {
            let err = std::io::Error::last_os_error();
            return Err(format!("cannot find name for user ID {}: {}", uid, err));
        }
        let name = CStr::from_ptr((*pw).pw_name);
        Ok(name.to_string_lossy().into_owned())
    }
}

#[cfg(not(unix))]
fn get_username() -> Result<String, String> {
    env::var("USERNAME")
        .or_else(|_| env::var("USER"))
        .map_err(|_| "cannot find name for current user (USERNAME/USER env var not set)".to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check standard options
    if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
            process::exit(0);
        } else if args[1] == "--version" {
            print_version("whoami", &["Richard Mlynarik"]);
            process::exit(0);
        }
    }
    
    // Handle extra operands
    if args.len() > 1 {
        eprintln!("whoami: extra operand '{}'", args[1]);
        eprintln!("Try 'whoami --help' for more information.");
        process::exit(1);
    }
    
    match get_username() {
        Ok(username) => {
            println!("{}", username);
            process::exit(0);
        }
        Err(err_msg) => {
            eprintln!("whoami: {}", err_msg);
            process::exit(1);
        }
    }
}
