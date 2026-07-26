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

#[derive(Clone, Copy, PartialEq)]
enum Mode { Bytes, Chars, Fields }

fn parse_ranges(spec: &str) -> (Vec<usize>, Option<usize>) {
    let mut indices = Vec::new();
    let mut unbounded_from = None;

    for part in spec.split(',') {
        let part = part.trim();
        if let Some(idx) = part.find('-') {
            let (start_str, end_str) = (&part[..idx], &part[idx + 1..]);
            let start = if start_str.is_empty() { 1 } else { start_str.parse().unwrap_or(1) };
            if end_str.is_empty() {
                if unbounded_from.is_none() || Some(start) < unbounded_from {
                    unbounded_from = Some(start);
                }
            } else {
                let end = end_str.parse().unwrap_or(start);
                for i in start..=end {
                    if i > 0 && !indices.contains(&i) { indices.push(i); }
                }
            }
        } else if let Ok(val) = part.parse::<usize>() {
            if val > 0 && !indices.contains(&val) { indices.push(val); }
        }
    }
    indices.sort_unstable();
    (indices, unbounded_from)
}

fn is_selected(idx: usize, indices: &[usize], unbounded_from: Option<usize>) -> bool {
    if let Some(start) = unbounded_from {
        if idx >= start { return true; }
    }
    indices.binary_search(&idx).is_ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = None;
    let mut range_spec = String::new();
    let mut delimiter = '\t';
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-b" | "-c" | "-f" => {
                mode = match arg.as_str() {
                    "-b" => Some(Mode::Bytes),
                    "-c" => Some(Mode::Chars),
                    _ => Some(Mode::Fields),
                };
                if i + 1 >= args.len() {
                    eprintln!("cut: option requires an argument -- '{}'", arg);
                    process::exit(1);
                }
                range_spec = args[i + 1].clone();
                i += 1;
            }
            "-d" => {
                if i + 1 >= args.len() {
                    eprintln!("cut: option requires an argument -- '-d'");
                    process::exit(1);
                }
                let chars: Vec<char> = args[i + 1].chars().collect();
                if chars.len() != 1 {
                    eprintln!("cut: the delimiter must be a single character");
                    process::exit(1);
                }
                delimiter = chars[0];
                i += 1;
            }
            "--help" => {
                println!("Usage: cut OPTION... [FILE]...\nPrint selected parts of lines from each FILE to standard output.\n\n  -b, --bytes=LIST    select only these bytes\n  -c, --characters=LIST  select only these characters\n  -d, --delimiter=DELIM  use DELIM instead of TAB for field delimiter\n  -f, --fields=LIST   select only these fields\n      --help          display this help and exit");
                return;
            }
            _ if arg.starts_with("-b") => { mode = Some(Mode::Bytes); range_spec = arg[2..].to_string(); }
            _ if arg.starts_with("-c") => { mode = Some(Mode::Chars); range_spec = arg[2..].to_string(); }
            _ if arg.starts_with("-f") => { mode = Some(Mode::Fields); range_spec = arg[2..].to_string(); }
            _ if arg.starts_with("-d") => {
                let chars: Vec<char> = arg[2..].chars().collect();
                if chars.len() != 1 {
                    eprintln!("cut: the delimiter must be a single character");
                    process::exit(1);
                }
                delimiter = chars[0];
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("cut: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    let mode = match mode {
        Some(m) => m,
        None => {
            eprintln!("cut: you must specify a list of bytes, characters, or fields\nTry 'cut --help' for more information.");
            process::exit(1);
        }
    };

    if files.is_empty() { files.push("-".to_string()); }
    let (indices, unbounded) = parse_ranges(&range_spec);

    for file in &files {
        let reader: Box<dyn Read> = if file == "-" {
            Box::new(io::stdin())
        } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => { eprintln!("cut: {}: {}", file, e); continue; }
            }
        };

        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            match mode {
                Mode::Bytes => {
                    let bytes = line.as_bytes();
                    for (idx, &b) in bytes.iter().enumerate() {
                        if is_selected(idx + 1, &indices, unbounded) {
                            print!("{}", b as char);
                        }
                    }
                    println!();
                }
                Mode::Chars => {
                    for (idx, ch) in line.chars().enumerate() {
                        if is_selected(idx + 1, &indices, unbounded) {
                            print!("{}", ch);
                        }
                    }
                    println!();
                }
                Mode::Fields => {
                    if !line.contains(delimiter) {
                        println!("{}", line);
                        continue;
                    }
                    let fields: Vec<&str> = line.split(delimiter).collect();
                    let mut selected = Vec::new();
                    for (idx, &field) in fields.iter().enumerate() {
                        if is_selected(idx + 1, &indices, unbounded) {
                            selected.push(field);
                        }
                    }
                    println!("{}", selected.join(&delimiter.to_string()));
                }
            }
        }
    }
}
