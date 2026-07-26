// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parents = false;
    let mut verbose = false;
    let mut dirs = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-p" | "--parents" => parents = true,
            "-v" | "--verbose" => verbose = true,
            "-pv" | "-vp" => {
                parents = true;
                verbose = true;
            }
            "--help" => {
                println!("Usage: mkdir [OPTION]... DIRECTORY...\nCreate the DIRECTORY(ies), if they do not already exist.\n\n  -p, --parents   no error if existing, make parent directories as needed\n  -v, --verbose   print a message for each created directory\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("mkdir: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => dirs.push(arg.clone()),
        }
    }

    if dirs.is_empty() {
        eprintln!("mkdir: missing operand\nTry 'mkdir --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for dir in dirs {
        let result = if parents {
            fs::create_dir_all(&dir)
        } else {
            fs::create_dir(&dir)
        };

        match result {
            Ok(_) => {
                if verbose {
                    println!("mkdir: created directory '{}'", dir);
                }
            }
            Err(e) if parents && e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
                exit_code = 1;
            }
        }
    }
    process::exit(exit_code);
}
