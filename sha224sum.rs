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

struct Sha224 {
    state: [u32; 8],
    buffer: [u8; 64],
    buf_len: usize,
    total_len: u64,
}

impl Sha224 {
    fn new() -> Self {
        Self {
            state: [
                0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939,
                0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
            ],
            buffer: [0u8; 64],
            buf_len: 0,
            total_len: 0,
        }
    }

    fn transform(&mut self) {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];

        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([self.buffer[i * 4], self.buffer[i * 4 + 1], self.buffer[i * 4 + 2], self.buffer[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = self.state[0]; let mut b = self.state[1]; let mut c = self.state[2]; let mut d = self.state[3];
        let mut e = self.state[4]; let mut f = self.state[5]; let mut g = self.state[6]; let mut h = self.state[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g; g = f; f = e; e = d.wrapping_add(temp1);
            d = c; c = b; b = a; a = temp1.wrapping_add(temp2);
        }

        for (i, val) in [a, b, c, d, e, f, g, h].iter().enumerate() {
            self.state[i] = self.state[i].wrapping_add(*val);
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.total_len += data.len() as u64;
        for &byte in data {
            self.buffer[self.buf_len] = byte;
            self.buf_len += 1;
            if self.buf_len == 64 {
                self.transform();
                self.buf_len = 0;
            }
        }
    }

    fn finalize(mut self) -> [u8; 28] {
        let bits = self.total_len * 8;
        self.buffer[self.buf_len] = 0x80;
        self.buf_len += 1;
        if self.buf_len > 56 {
            while self.buf_len < 64 { self.buffer[self.buf_len] = 0; self.buf_len += 1; }
            self.transform();
            self.buf_len = 0;
        }
        while self.buf_len < 56 { self.buffer[self.buf_len] = 0; self.buf_len += 1; }
        self.buffer[56..64].copy_from_slice(&bits.to_be_bytes());
        self.transform();

        let mut out = [0u8; 28];
        for (i, &word) in self.state[..7].iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else { Box::new(File::open(path)?) };
    let mut hasher = Sha224::new();
    let mut buffer = [0u8; 8192];
    while let Ok(n) = reader.read(&mut buffer) { if n == 0 { break; } hasher.update(&buffer[..n]); }
    Ok(hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut check_mode = false;
    let mut binary_mode = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-c" | "--check" => check_mode = true,
            "-b" | "--binary" => binary_mode = true,
            "-t" | "--text" => binary_mode = false,
            "--help" => {
                println!("Usage: sha224sum [OPTION]... [FILE]...\nPrint or check SHA224 (224-bit) checksums.\n\n  -b, --binary   read in binary mode\n  -c, --check    read SHA224 sums from the FILEs and check them\n  -t, --text     read in text mode (default)\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("sha224sum: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    if check_mode {
        let mut fail_count = 0;
        for file in files {
            let reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
                match File::open(&file) { Ok(f) => Box::new(f), Err(e) => { eprintln!("sha224sum: {}: {}", file, e); continue; } }
            };
            for line in BufReader::new(reader).lines().map_while(Result::ok) {
                if let Some(idx) = line.find(' ') {
                    let expected = &line[..idx];
                    let target = line[idx..].trim_start_matches([' ', '*']).trim();
                    match hash_file(target) {
                        Ok(actual) if actual == expected => println!("{}: OK", target),
                        _ => { println!("{}: FAILED", target); fail_count += 1; }
                    }
                }
            }
        }
        if fail_count > 0 { eprintln!("sha224sum: WARNING: {} computed checksum did NOT match", fail_count); process::exit(1); }
    } else {
        let mode_char = if binary_mode { '*' } else { ' ' };
        let mut exit_code = 0;
        for file in files {
            match hash_file(&file) {
                Ok(hash) => println!("{} {}{}", hash, mode_char, file),
                Err(e) => { eprintln!("sha224sum: {}: {}", file, e); exit_code = 1; }
            }
        }
        process::exit(exit_code);
    }
}
