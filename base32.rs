// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

const CHARS: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

fn encode_base32(data: &[u8], wrap: usize) {
    let mut out = io::stdout().lock();
    let mut col = 0;
    for chunk in data.chunks(5) {
        let mut buf = [0u8; 8];
        let mut val = 0u64;
        for (i, &b) in chunk.iter().enumerate() { val |= (b as u64) << (32 - i * 8); }
        let chars_to_output = (chunk.len() * 8 + 4) / 5;
        for i in 0..8 {
            if i < chars_to_output { buf[i] = CHARS[((val >> (35 - i * 5)) & 0x1F) as usize]; }
            else { buf[i] = b'='; }
        }
        for &c in &buf {
            let _ = out.write_all(&[c]); col += 1;
            if wrap > 0 && col >= wrap { let _ = writeln!(out); col = 0; }
        }
    }
    if wrap > 0 && col > 0 { let _ = writeln!(out); }
}

fn decode_base32(data: &[u8]) {
    let mut out = io::stdout().lock();
    let clean: Vec<u8> = data.iter().copied().filter(|&b| !b.is_ascii_whitespace()).collect();
    for chunk in clean.chunks(8) {
        if chunk.len() < 2 { continue; }
        let dec = |b: u8| -> u64 {
            match b {
                b'A'..=b'Z' => (b - b'A') as u64,
                b'a'..=b'z' => (b - b'a') as u64,
                b'2'..=b'7' => (b - b'2' + 26) as u64,
                _ => 0,
            }
        };
        let mut val = 0u64;
        let mut valid_chars = 0;
        for (i, &b) in chunk.iter().enumerate() {
            if b == b'=' { break; }
            val |= dec(b) << (35 - i * 5);
            valid_chars += 1;
        }
        let bytes_to_output = (valid_chars * 5) / 8;
        for i in 0..bytes_to_output {
            let _ = out.write_all(&[((val >> (32 - i * 8)) & 0xFF) as u8]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut decode = false;
    let mut wrap = 76usize;
    let mut file_arg = "-".to_string();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-d" || arg == "--decode" {
            decode = true;
        } else if arg == "-w" || arg == "--wrap" {
            if i + 1 >= args.len() { eprintln!("base32: option requires an argument"); process::exit(1); }
            wrap = args[i + 1].parse().unwrap_or(76); i += 1;
        } else if let Some(val) = arg.strip_prefix("-w") {
            wrap = val.parse().unwrap_or(76);
        } else if arg == "--help" {
            println!("Usage: base32 [OPTION]... [FILE]\nBase32 encode or decode FILE, or standard input, to standard output.\n\n  -d, --decode          decode data\n  -w, --wrap=COLS       wrap encoded lines after COLS character (default 76)\n      --help            display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("base32: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            file_arg = arg.clone();
        }
        i += 1;
    }

    let mut buffer = Vec::new();
    let mut reader: Box<dyn io::Read> = if file_arg == "-" { Box::new(io::stdin()) } else {
        match File::open(&file_arg) { Ok(f) => Box::new(f), Err(e) => { eprintln!("base32: {}: {}", file_arg, e); process::exit(1); } }
    };
    let _ = reader.read_to_end(&mut buffer);

    if decode { decode_base32(&buffer); } else { encode_base32(&buffer, wrap); }
}
