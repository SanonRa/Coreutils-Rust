// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

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

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
        if args[1] == "--help" {
            println!("Usage: true [ignored command line arguments]");
            println!("  or:  true OPTION");
            println!("Exit with a status code indicating success.\n");
            println!("      --help        display this help and exit");
            println!("      --version     output version information and exit");
            println!("\nNOTE: your shell may have its own version of true, which usually supersedes");
            println!("the version described here. Please refer to your shell's documentation");
            println!("for details about the options it supports.");
            print_help_epilogue("true");
            process::exit(0);
        } else if args[1] == "--version" {
            print_version("true", &["Jim Meyering"]);
            process::exit(0);
        }
    }
    
    process::exit(0);
}
