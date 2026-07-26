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
    println!("Usage: echo [SHORT-OPTION]... [STRING]...");
    println!("  or:  echo LONG-OPTION");
    println!("Echo the STRING(s) to standard output.\n");
    println!("  -n             do not output the trailing newline");
    println!("  -e             enable interpretation of backslash escapes");
    println!("  -E             disable interpretation of backslash escapes (default)");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    println!("\nIf -e is in effect, the following sequences are recognized:");
    println!("  \\\\      backslash");
    println!("  \\a      alert (bell)");
    println!("  \\b      backspace");
    println!("  \\c      produce no further output");
    println!("  \\e      escape");
    println!("  \\f      form feed");
    println!("  \\n      new line");
    println!("  \\r      carriage return");
    println!("  \\t      horizontal tab");
    println!("  \\v      vertical tab");
    println!("  \\0NNN   byte with octal value NNN (1 to 3 digits)");
    println!("  \\xHH    byte with hexadecimal value HH (1 to 2 digits)");
    println!("\nNOTE: your shell may have its own version of echo, which usually supersedes");
    println!("the version described here. Please refer to your shell's documentation");
    println!("for details about the options it supports.");
    print_help_epilogue("echo");
}

fn hextobin(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}

fn interpret_escapes(s: &str, output: &mut Vec<u8>) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 1;
            match bytes[i] {
                b'a' => output.push(7),
                b'b' => output.push(8),
                b'c' => return false, // \c produce no further output
                b'e' => output.push(27),
                b'f' => output.push(12),
                b'n' => output.push(10),
                b'r' => output.push(13),
                b't' => output.push(9),
                b'v' => output.push(11),
                b'\\' => output.push(b'\\'),
                b'x' => {
                    i += 1;
                    let mut val = 0u8;
                    let mut digits = 0;
                    while i < bytes.len() && digits < 2 {
                        let c = bytes[i];
                        if c.is_ascii_hexdigit() {
                            val = val * 16 + hextobin(c);
                            digits += 1;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    if digits > 0 {
                        output.push(val);
                        i -= 1;
                    } else {
                        output.push(b'\\');
                        output.push(b'x');
                        i -= 1;
                    }
                }
                c @ b'0'..=b'7' => {
                    let mut val = 0u32;
                    if c == b'0' {
                        val = 0;
                        i += 1;
                        let mut octal_count = 0;
                        while i < bytes.len() && octal_count < 3 {
                            let oc = bytes[i];
                            if oc >= b'0' && oc <= b'7' {
                                val = val * 8 + (oc - b'0') as u32;
                                octal_count += 1;
                                i += 1;
                            } else {
                                break;
                            }
                        }
                        i -= 1;
                    } else {
                        val = (c - b'0') as u32;
                        let mut digits = 1;
                        i += 1;
                        while i < bytes.len() && digits < 3 {
                            let oc = bytes[i];
                            if oc >= b'0' && oc <= b'7' {
                                val = val * 8 + (oc - b'0') as u32;
                                digits += 1;
                                i += 1;
                            } else {
                                break;
                            }
                        }
                        i -= 1;
                    }
                    output.push(val as u8);
                }
                other => {
                    output.push(b'\\');
                    output.push(other);
                }
            }
        } else {
            output.push(bytes[i]);
        }
        i += 1;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut display_return = true;
    let mut do_v9 = false;
    
    // Recognizing --help or --version only if it is the ONLY argument
    if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
            process::exit(0);
        } else if args[1] == "--version" {
            print_version("echo", &["Brian Fox", "Chet Ramey"]);
            process::exit(0);
        }
    }
    
    let mut idx = 1;
    // Manual option parsing
    while idx < args.len() && args[idx].starts_with('-') && args[idx] != "-" {
        let arg = &args[idx];
        let mut valid_opts = true;
        
        for c in arg.chars().skip(1) {
            match c {
                'e' | 'E' | 'n' => {}
                _ => {
                    valid_opts = false;
                    break;
                }
            }
        }
        
        if !valid_opts || arg.len() == 1 {
            break;
        }
        
        // Apply the options
        for c in arg.chars().skip(1) {
            match c {
                'e' => do_v9 = true,
                'E' => do_v9 = false,
                'n' => display_return = false,
                _ => {}
            }
        }
        idx += 1;
    }
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    let mut output_bytes = Vec::new();
    let mut stop_output = false;
    
    for (k, arg) in args[idx..].iter().enumerate() {
        if k > 0 {
            output_bytes.push(b' ');
        }
        if do_v9 {
            if !interpret_escapes(arg, &mut output_bytes) {
                stop_output = true;
                break;
            }
        } else {
            output_bytes.extend_from_slice(arg.as_bytes());
        }
    }
    
    if display_return && !stop_output {
        output_bytes.push(b'\n');
    }
    
    if let Err(e) = handle.write_all(&output_bytes) {
        eprintln!("echo: {}", e);
        process::exit(1);
    }
    
    process::exit(0);
}
