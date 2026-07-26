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

enum SumAlgorithm { Bsd, SysV }

fn calc_bsd(data: &[u8]) -> (u16, usize) {
    let mut checksum = 0u16;
    for &byte in data {
        checksum = (checksum >> 1) + ((checksum & 1) << 15);
        checksum = checksum.wrapping_add(byte as u16);
    }
    let blocks = (data.len() + 1023) / 1024;
    (checksum, blocks)
}

fn calc_sysv(data: &[u8]) -> (u32, usize) {
    let mut total = 0u32;
    for &byte in data { total += byte as u32; }
    let mut r = (total & 0xFFFF) + (total >> 16);
    r = (r & 0xFFFF) + (r >> 16);
    let blocks = (data.len() + 511) / 512;
    (r, blocks)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut algo = SumAlgorithm::Bsd;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-r" => algo = SumAlgorithm::Bsd,
            "-s" | "--sysv" => algo = SumAlgorithm::SysV,
            "--help" => {
                println!("Usage: sum [OPTION]... [FILE]...\nPrint checksum and block counts for each FILE.\n\n  -r          use default BSD checksum algorithm, use 1K blocks\n  -s, --sysv  use System V sum algorithm, use 512 bytes blocks\n      --help  display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("sum: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    let mut exit_code = 0;
    for file in files {
        let mut buffer = Vec::new();
        let res = if file == "-" { io::stdin().read_to_end(&mut buffer) } else { File::open(&file).and_then(|mut f| f.read_to_end(&mut buffer)) };
        match res {
            Ok(_) => match algo {
                SumAlgorithm::Bsd => {
                    let (sum, blocks) = calc_bsd(&buffer);
                    if file == "-" { println!("{:05} {:5}", sum, blocks); } else { println!("{:05} {:5} {}", sum, blocks, file); }
                }
                SumAlgorithm::SysV => {
                    let (sum, blocks) = calc_sysv(&buffer);
                    if file == "-" { println!("{} {}", sum, blocks); } else { println!("{} {} {}", sum, blocks, file); }
                }
            },
            Err(e) => { eprintln!("sum: {}: {}", file, e); exit_code = 1; }
        }
    }
    process::exit(exit_code);
}
