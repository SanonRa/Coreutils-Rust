// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut physical = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-P" | "--physical" => physical = true,
            "-L" | "--logical" => physical = false,
            "--help" => {
                println!("Usage: pwd [OPTION]...\nPrint the full filename of the current working directory.\n\n  -L, --logical   use PWD from environment, even if it contains symlinks\n  -P, --physical  avoid all symlinks\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("pwd: invalid option -- '{}'\nTry 'pwd --help' for more information.", arg);
                process::exit(1);
            }
            _ => {}
        }
    }

    let path = if physical {
        env::current_dir().unwrap_or_else(|e| {
            eprintln!("pwd: error reading current directory: {}", e);
            process::exit(1);
        })
    } else {
        match env::var("PWD") {
            Ok(val) => {
                let pwd_path = PathBuf::from(&val);
                if pwd_path.is_absolute() && pwd_path.exists() {
                    if let Ok(curr) = env::current_dir() {
                        if let (Ok(p1), Ok(p2)) = (pwd_path.canonicalize(), curr.canonicalize()) {
                            if p1 == p2 {
                                pwd_path
                            } else {
                                curr
                            }
                        } else {
                            curr
                        }
                    } else {
                        pwd_path
                    }
                } else {
                    env::current_dir().unwrap_or_else(|e| {
                        eprintln!("pwd: error reading current directory: {}", e);
                        process::exit(1);
                    })
                }
            }
            Err(_) => env::current_dir().unwrap_or_else(|e| {
                eprintln!("pwd: error reading current directory: {}", e);
                process::exit(1);
            }),
        }
    };

    println!("{}", path.display());
}
