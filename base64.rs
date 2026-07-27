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

const CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode(data: &[u8], wrap: usize) {
    let mut out = io::stdout().lock();
    let mut col = 0;
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let trip = (b0 << 16) | (b1 << 8) | b2;

        let mut enc = [b'='; 4];
        enc[0] = CHARS[((trip >> 18) & 0x3F) as usize];
        enc[1] = CHARS[((trip >> 12) & 0x3F) as usize];
        if chunk.len() > 1 { enc[2] = CHARS[((trip >> 6) & 0x3F) as usize]; }
        if chunk.len() > 2 { enc[3] = CHARS[(trip & 0x3F) as usize]; }

        for &c in &enc {
            let _ = out.write_all(&[c]);
            col += 1;
            if wrap > 0 && col >= wrap {
                let _ = out.write_all(b"\n");
                col = 0;
            }
        }
    }
    if wrap > 0 && col > 0 { let _ = out.write_all(b"\n"); }
}

fn decode(data: &[u8]) {
    let mut out = io::stdout().lock();
    let clean: Vec<u8> = data.iter().copied().filter(|&b| !b.is_ascii_whitespace()).collect();
    for chunk in clean.chunks(4) {
        if chunk.len() < 2 { continue; }
        let dec = |b: u8| -> u32 {
            match b {
                b'A'..=b'Z' => (b - b'A') as u32,
                b'a'..=b'z' => (b - b'a' + 26) as u32,
                b'0'..=b'9' => (b - b'0' + 52) as u32,
                b'+' => 62,
                b'/' => 63,
                _ => 0,
            }
        };
        let v0 = dec(chunk[0]);
        let v1 = dec(chunk[1]);
        let v2 = if chunk.len() > 2 && chunk[2] != b'=' { dec(chunk[2]) } else { 0 };
        let v3 = if chunk.len() > 3 && chunk[3] != b'=' { dec(chunk[3]) } else { 0 };
        let trip = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;

        let _ = out.write_all(&[(trip >> 16) as u8]);
        if chunk.len() > 2 && chunk[2] != b'=' { let _ = out.write_all(&[(trip >> 8) as u8]); }
        if chunk.len() > 3 && chunk[3] != b'=' { let _ = out.write_all(&[(trip) as u8]); }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dec_mode = false;
    let mut wrap = 76usize;
    let mut file_arg = "-".to_string();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-d" || arg == "--decode" {
            dec_mode = true;
        } else if arg == "-w" || arg == "--wrap" {
            if i + 1 >= args.len() { eprintln!("base64: option requires an argument"); process::exit(1); }
            wrap = args[i + 1].parse().unwrap_or(76);
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-w") {
            wrap = val.parse().unwrap_or(76);
        } else if arg == "--help" {
            println!("Usage: base64 [OPTION]... [FILE]\nBase64 encode or decode FILE, or standard input, to standard output.\n\n  -d, --decode          decode data\n  -w, --wrap=COLS       wrap encoded lines after COLS character (default 76, 0 to disable)\n      --help            display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("base64: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            file_arg = arg.clone();
        }
        i += 1;
    }

    let mut buffer = Vec::new();
    let reader: Box<dyn Read> = if file_arg == "-" {
        Box::new(io::stdin())
    } else {
        match File::open(&file_arg) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("base64: {}: {}", file_arg, e); process::exit(1); }
        }
    };

    let mut buf_reader = reader;
    if let Err(e) = buf_reader.read_to_end(&mut buffer) {
        eprintln!("base64: read error: {}", e);
        process::exit(1);
    }

    if dec_mode { decode(&buffer); } else { encode(&buffer, wrap); }
}
