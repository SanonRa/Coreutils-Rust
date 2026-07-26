// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] == "--help" {
        println!("Usage: csplit [OPTION]... FILE PATTERN...\nOutput pieces of FILE separated by PATTERN(s) to files 'xx00', 'xx01', ...,\nand output byte counts of each piece to standard output.\n\n      --help     display this help and exit");
        return;
    }

    let file_arg = &args[1];
    let patterns: Vec<&String> = args[2..].iter().collect();

    let reader: Box<dyn Read> = if file_arg == "-" { Box::new(io::stdin()) } else {
        match File::open(file_arg) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("csplit: {}: {}", file_arg, e); process::exit(1); }
        }
    };

    let lines: Vec<String> = BufReader::new(reader).lines().map_while(Result::ok).collect();
    let mut split_indices = Vec::new();

    for pat in patterns {
        if let Ok(line_num) = pat.parse::<usize>() {
            if line_num > 0 && line_num <= lines.len() && !split_indices.contains(&(line_num - 1)) {
                split_indices.push(line_num - 1);
            }
        } else if pat.starts_with('/') && pat.ends_with('/') && pat.len() > 2 {
            let target = &pat[1..pat.len() - 1];
            if let Some(idx) = lines.iter().position(|l| l.contains(target)) {
                if !split_indices.contains(&idx) { split_indices.push(idx); }
            }
        }
    }
    split_indices.sort_unstable();

    let mut start = 0;
    let mut file_idx = 0;

    let mut write_piece = |from: usize, to: usize| {
        let fname = format!("xx{:02}", file_idx);
        file_idx += 1;
        if let Ok(mut f) = File::create(&fname) {
            let mut bytes_written = 0;
            for line in &lines[from..to] {
                let formatted = format!("{}\n", line);
                let _ = f.write_all(formatted.as_bytes());
                bytes_written += formatted.len();
            }
            println!("{}", bytes_written);
        }
    };

    for &idx in &split_indices {
        if idx > start {
            write_piece(start, idx);
            start = idx;
        }
    }
    if start < lines.len() { write_piece(start, lines.len()); }
}
