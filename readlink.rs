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
    let mut canonicalize = false;
    let mut no_newline = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-f" | "--canonicalize" => canonicalize = true,
            "-n" | "--no-newline" => no_newline = true,
            "--help" => {
                println!("Usage: readlink [OPTION]... FILE...\nPrint value of a symbolic link or canonical file name.\n\n  -f, --canonicalize   canonicalize by following every symlink in every component\n  -n, --no-newline     do not output the trailing newline\n      --help           display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("readlink: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() {
        eprintln!("readlink: missing operand\nTry 'readlink --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for file in files {
        let result = if canonicalize {
            fs::canonicalize(&file)
        } else {
            fs::read_link(&file)
        };

        match result {
            Ok(path) => {
                let output = path.display().to_string();
                let cleaned = output.strip_prefix(r"\\?\").unwrap_or(&output);
                if no_newline {
                    print!("{}", cleaned);
                } else {
                    println!("{}", cleaned);
                }
            }
            Err(_) => exit_code = 1,
        }
    }
    process::exit(exit_code);
}
