// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn process_stream<R: Read>(mut reader: R, separator: &str, before: bool) {
    let mut content = String::new();
    if let Err(e) = reader.read_to_string(&mut content) {
        eprintln!("tac: read error: {}", e);
        return;
    }

    if content.is_empty() { return; }

    let mut chunks: Vec<&str> = if separator == "\n" {
        let mut lines: Vec<&str> = content.split_inclusive('\n').collect();
        if !content.ends_with('\n') {
            // Ensure trailing non-newline chunk is preserved
        }
        lines
    } else {
        content.split_inclusive(separator).collect()
    };

    chunks.reverse();

    for chunk in chunks {
        if before && chunk.ends_with(separator) && chunk.len() > separator.len() {
            let core = &chunk[..chunk.len() - separator.len()];
            print!("{}{}", separator, core);
        } else {
            print!("{}", chunk);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut separator = "\n".to_string();
    let mut before = false;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-s" || arg == "--separator" {
            if i + 1 >= args.len() {
                eprintln!("tac: option requires an argument -- '{}'", arg);
                process::exit(1);
            }
            separator = args[i + 1].clone();
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-s") {
            separator = val.to_string();
        } else if arg == "-b" || arg == "--before" {
            before = true;
        } else if arg == "--help" {
            println!("Usage: tac [OPTION]... [FILE]...\nWrite each FILE to standard output, last line first.\n\n  -b, --before             attach the separator before instead of after\n  -s, --separator=STRING   use STRING as the separator instead of newline\n      --help               display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("tac: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    for file in &files {
        if file == "-" {
            process_stream(io::stdin(), &separator, before);
        } else {
            match File::open(file) {
                Ok(f) => process_stream(f, &separator, before),
                Err(e) => {
                    eprintln!("tac: failed to open '{}' for reading: {}", file, e);
                    process::exit(1);
                }
            }
        }
    }
}
