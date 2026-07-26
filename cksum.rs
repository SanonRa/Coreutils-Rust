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

fn calc_crc32(data: &[u8], mut total_len: usize) -> (u32, usize) {
    let mut crc = 0u32;
    for &byte in data {
        crc ^= (byte as u32) << 24;
        for _ in 0..8 {
            if (crc & 0x80000000) != 0 { crc = (crc << 1) ^ 0x04C11DB7; }
            else { crc <<= 1; }
        }
    }

    let len_copy = total_len;
    let mut len_bytes = Vec::new();
    while total_len > 0 {
        len_bytes.push((total_len & 0xFF) as u8);
        total_len >>= 8;
    }
    for &byte in len_bytes.iter().rev() {
        crc ^= (byte as u32) << 24;
        for _ in 0..8 {
            if (crc & 0x80000000) != 0 { crc = (crc << 1) ^ 0x04C11DB7; }
            else { crc <<= 1; }
        }
    }
    (!crc, len_copy)
}

fn process_stream<R: Read>(mut reader: R) -> io::Result<(u32, usize)> {
    let mut all_bytes = Vec::new();
    reader.read_to_end(&mut all_bytes)?;
    let len = all_bytes.len();
    Ok(calc_crc32(&all_bytes, len))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: cksum [FILE]...\nPrint CRC checksum and byte counts of each FILE.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("cksum: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    let mut exit_code = 0;
    for file in files {
        let res = if file == "-" { process_stream(io::stdin()) } else { File::open(&file).and_then(process_stream) };
        match res {
            Ok((crc, len)) => {
                if file == "-" { println!("{} {}", crc, len); }
                else { println!("{} {} {}", crc, len, file); }
            }
            Err(e) => { eprintln!("cksum: {}: {}", file, e); exit_code = 1; }
        }
    }
    process::exit(exit_code);
}
