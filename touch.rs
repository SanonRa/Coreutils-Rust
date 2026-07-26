// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut no_create = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-c" | "--no-create" => no_create = true,
            "--help" => {
                println!("Usage: touch [OPTION]... FILE...\nUpdate the access and modification times of each FILE to the current time.\n\n  -c, --no-create   do not create any files\n      --help        display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("touch: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() {
        eprintln!("touch: missing file operand\nTry 'touch --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for file in files {
        let path = Path::new(&file);
        if !path.exists() {
            if no_create {
                continue;
            }
            if let Err(e) = File::create(path) {
                eprintln!("touch: cannot touch '{}': {}", file, e);
                exit_code = 1;
                continue;
            }
        }
        if let Ok(f) = File::options().write(true).open(path) {
            if let Err(e) = f.set_modified(SystemTime::now()) {
                eprintln!("touch: setting times of '{}': {}", file, e);
                exit_code = 1;
            }
        } else {
            eprintln!("touch: cannot open '{}' for writing", file);
            exit_code = 1;
        }
    }
    process::exit(exit_code);
}
