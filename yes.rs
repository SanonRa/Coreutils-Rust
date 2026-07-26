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
    println!("Usage: yes [STRING]...");
    println!("  or:  yes OPTION");
    println!("Repeatedly output a line with all specified STRING(s), or 'y'.\n");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    print_help_epilogue("yes");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check standard options
    if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
            process::exit(0);
        } else if args[1] == "--version" {
            print_version("yes", &["David MacKenzie"]);
            process::exit(0);
        }
    }
    
    // Determine the string to output
    let output_str = if args.len() > 1 {
        let mut s = args[1..].join(" ");
        s.push('\n');
        s
    } else {
        "y\n".to_string()
    };
    
    let bytes = output_str.as_bytes();
    
    // Buffer the output for maximum throughput (matching GNU yes performance optimizations)
    const BUF_SIZE: usize = 16384;
    let mut buffer = Vec::with_capacity(BUF_SIZE + bytes.len());
    while buffer.len() < BUF_SIZE {
        buffer.extend_from_slice(bytes);
    }
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    loop {
        if let Err(e) = handle.write_all(&buffer) {
            // Ignore error reporting on SIGPIPE/BrokenPipe if stdout is closed, 
            // but print it to stderr for other write errors as does GNU yes.
            if e.kind() != io::ErrorKind::BrokenPipe {
                eprintln!("yes: standard output: {}", e);
            }
            process::exit(1);
        }
    }
}
