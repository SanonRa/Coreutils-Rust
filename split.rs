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

fn next_filename(prefix: &str, counter: usize) -> String {
    let c1 = (b'a' + ((counter / 26) % 26) as u8) as char;
    let c2 = (b'a' + (counter % 26) as u8) as char;
    format!("{}{}{}", prefix, c1, c2)
}

fn parse_size(s: &str) -> Option<usize> {
    let mut num_str = s.to_string();
    let mut mult = 1;
    if let Some(c) = s.chars().last() {
        match c.to_ascii_uppercase() {
            'K' => { mult = 1024; num_str.pop(); }
            'M' => { mult = 1024 * 1024; num_str.pop(); }
            'G' => { mult = 1024 * 1024 * 1024; num_str.pop(); }
            _ => {}
        }
    }
    num_str.parse::<usize>().ok().map(|n| n * mult)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lines_chunk: Option<usize> = Some(1000);
    let mut bytes_chunk: Option<usize> = None;
    let mut file_arg = "-".to_string();
    let mut prefix = "x".to_string();
    let mut i = 1;
    let mut pos_args = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-l" | "--lines" => {
                if i + 1 >= args.len() { eprintln!("split: option requires an argument"); process::exit(1); }
                lines_chunk = args[i + 1].parse().ok(); bytes_chunk = None; i += 1;
            }
            "-b" | "--bytes" => {
                if i + 1 >= args.len() { eprintln!("split: option requires an argument"); process::exit(1); }
                bytes_chunk = parse_size(&args[i + 1]); lines_chunk = None; i += 1;
            }
            "--help" => {
                println!("Usage: split [OPTION]... [FILE [PREFIX]]\nOutput pieces of FILE to PREFIXaa, PREFIXab, ...;\ndefault size is 1000 lines, and default PREFIX is 'x'.\n\n  -b, --bytes=SIZE    put SIZE bytes per output file\n  -l, --lines=NUMBER  put NUMBER lines/records per output file\n      --help          display this help and exit");
                return;
            }
            _ if arg.starts_with("-l") => {
                let val = arg.strip_prefix("-l").unwrap();
                lines_chunk = val.parse().ok();
                bytes_chunk = None;
            }
            _ if arg.starts_with("-b") => {
                let val = arg.strip_prefix("-b").unwrap();
                bytes_chunk = parse_size(val);
                lines_chunk = None;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("split: unrecognized option '{}'", arg); process::exit(1); }
            _ => {
                if pos_args == 0 { file_arg = arg.clone(); } else if pos_args == 1 { prefix = arg.clone(); }
                pos_args += 1;
            }
        }
        i += 1;
    }

    let reader: Box<dyn Read> = if file_arg == "-" { Box::new(io::stdin()) } else {
        match File::open(&file_arg) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("split: {}: {}", file_arg, e); process::exit(1); }
        }
    };

    let mut counter = 0;
    if let Some(max_lines) = lines_chunk {
        let mut buf_reader = BufReader::new(reader);
        let mut current_lines = 0;
        let mut out_file: Option<File> = None;
        let mut line = String::new();

        while let Ok(n) = buf_reader.read_line(&mut line) {
            if n == 0 { break; }
            if current_lines == 0 || current_lines >= max_lines {
                let fname = next_filename(&prefix, counter);
                out_file = File::create(&fname).ok();
                counter += 1;
                current_lines = 0;
            }
            if let Some(ref mut f) = out_file { let _ = f.write_all(line.as_bytes()); }
            current_lines += 1;
            line.clear();
        }
    } else if let Some(max_bytes) = bytes_chunk {
        let mut handle = reader;
        let mut buffer = vec![0u8; 8192];
        let mut current_bytes = 0;
        let mut out_file: Option<File> = None;

        while let Ok(read_bytes) = handle.read(&mut buffer) {
            if read_bytes == 0 { break; }
            let mut offset = 0;
            while offset < read_bytes {
                if current_bytes == 0 || current_bytes >= max_bytes {
                    let fname = next_filename(&prefix, counter);
                    out_file = File::create(&fname).ok();
                    counter += 1;
                    current_bytes = 0;
                }
                let to_write = (read_bytes - offset).min(max_bytes - current_bytes);
                if let Some(ref mut f) = out_file { let _ = f.write_all(&buffer[offset..offset + to_write]); }
                offset += to_write;
                current_bytes += to_write;
            }
        }
    }
}
