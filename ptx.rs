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

fn process_stream<R: Read>(reader: R, width: usize, source_name: &str) {
    let mut words = Vec::new();
    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        for word in line.split_whitespace() {
            let clean: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            if !clean.is_empty() {
                words.push(clean);
            }
        }
    }

    let side_len = (width.saturating_sub(30)) / 2;

    for i in 0..words.len() {
        let keyword = &words[i];
        if keyword.len() < 3 { continue; } // Skip brief stopwords

        let mut left = String::new();
        let mut idx = i;
        while idx > 0 && left.len() < side_len {
            idx -= 1;
            let cand = format!("{} {}", words[idx], left);
            if cand.len() <= side_len { left = cand; } else { break; }
        }

        let mut right = String::new();
        let mut idx = i + 1;
        while idx < words.len() && right.len() < side_len {
            let cand = format!("{} {}", right, words[idx]);
            if cand.len() <= side_len { right = cand; } else { break; }
            idx += 1;
        }

        println!("{:>width$}   {:<15}   {:<width$}   {}", left.trim_start(), keyword, right.trim_end(), source_name, width = side_len);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut width = 72usize;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-w" || arg == "--width" {
            if i + 1 >= args.len() { eprintln!("ptx: option requires an argument"); process::exit(1); }
            width = args[i + 1].parse().unwrap_or(72); i += 1;
        } else if let Some(val) = arg.strip_prefix("-w") {
            width = val.parse().unwrap_or(72);
        } else if arg == "--help" {
            println!("Usage: ptx [OPTION]... [INPUT]... [OUTPUT]\nOutput a permuted index, including context, of the words in the input files.\n\n  -w, --width=N   output width in columns, reference excluded (default 72)\n      --help      display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("ptx: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    for file in files {
        if file == "-" {
            process_stream(io::stdin(), width, "");
        } else {
            match File::open(&file) {
                Ok(f) => process_stream(f, width, &file),
                Err(e) => { eprintln!("ptx: {}: {}", file, e); process::exit(1); }
            }
        }
    }
}
