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

fn format_paragraph(lines: &[String], width: usize) {
    let mut words = Vec::new();
    for line in lines {
        for word in line.split_whitespace() {
            words.push(word);
        }
    }

    let mut current_len = 0;
    for (idx, word) in words.iter().enumerate() {
        let word_len = word.chars().count();
        if current_len == 0 {
            print!("{}", word);
            current_len = word_len;
        } else if current_len + 1 + word_len <= width {
            print!(" {}", word);
            current_len += 1 + word_len;
        } else {
            println!();
            print!("{}", word);
            current_len = word_len;
        }
        if idx == words.len() - 1 { println!(); }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut width = 75;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-w" || arg == "--width" {
            if i + 1 >= args.len() {
                eprintln!("fmt: option requires an argument -- '{}'", arg);
                process::exit(1);
            }
            width = args[i + 1].parse().unwrap_or(75);
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-w") {
            width = val.parse().unwrap_or(75);
        } else if let Some(val) = arg.strip_prefix("--width=") {
            width = val.parse().unwrap_or(75);
        } else if arg == "--help" {
            println!("Usage: fmt [OPTION]... [FILE]...\nReformat each paragraph in the FILE(s), writing to standard output.\n\n  -w, --width=WIDTH   maximum line width (default of 75 columns)\n      --help          display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("fmt: unrecognized option '{}'", arg);
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
                Err(e) => { eprintln!("fmt: {}: {}", file, e); continue; }
            }
        };

        let mut paragraph = Vec::new();
        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                if !paragraph.is_empty() {
                    format_paragraph(&paragraph, width);
                    paragraph.clear();
                }
                println!();
            } else {
                paragraph.push(line);
            }
        }
        if !paragraph.is_empty() {
            format_paragraph(&paragraph, width);
        }
    }
}
