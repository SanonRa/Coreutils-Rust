// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tab_size = 8;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-t" || arg == "--tabs" {
            if i + 1 >= args.len() {
                eprintln!("expand: option requires an argument -- '{}'", arg);
                process::exit(1);
            }
            tab_size = args[i + 1].parse().unwrap_or(8);
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-t") {
            tab_size = val.parse().unwrap_or(8);
        } else if let Some(val) = arg.strip_prefix("--tabs=") {
            tab_size = val.parse().unwrap_or(8);
        } else if arg == "--help" {
            println!("Usage: expand [OPTION]... [FILE]...\nConvert tabs in each FILE to spaces, writing to standard output.\n\n  -t, --tabs=NUMBER   have tabs NUMBER characters apart, not 8\n      --help          display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("expand: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    for file in &files {
        let reader: Box<dyn Read> = if file == "-" {
            Box::new(io::stdin())
        } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => { eprintln!("expand: {}: {}", file, e); continue; }
            }
        };

        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            let mut col = 0;
            for ch in line.chars() {
                if ch == '\t' {
                    let spaces = tab_size - (col % tab_size);
                    for _ in 0..spaces { print!(" "); }
                    col += spaces;
                } else {
                    print!("{}", ch);
                    col += 1;
                }
            }
            println!();
        }
    }
}
