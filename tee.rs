// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut append = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" | "--append" => append = true,
            "-i" | "--ignore-interrupts" => {
                #[cfg(unix)]
                unsafe {
                    extern "C" { fn signal(sig: i32, cb: usize) -> usize; }
                    signal(2, 1); // SIGINT, SIG_IGN
                }
            }
            "--help" => {
                println!("Usage: tee [OPTION]... [FILE]...\nCopy standard input to each FILE, and also to standard output.\n\n  -a, --append              append to the given FILEs, do not overwrite\n  -i, --ignore-interrupts   ignore interrupt signals\n      --help                display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("tee: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
    }

    let mut handles = Vec::new();
    for path in files {
        match OpenOptions::new().write(true).create(true).truncate(!append).append(append).open(&path) {
            Ok(f) => handles.push(f),
            Err(e) => {
                eprintln!("tee: {}: {}", path, e);
            }
        }
    }

    let mut buffer = [0u8; 8192];
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    while let Ok(n) = stdin.read(&mut buffer) {
        if n == 0 { break; }
        let _ = stdout.write_all(&buffer[..n]);
        for handle in &mut handles {
            let _ = handle.write_all(&buffer[..n]);
        }
    }
}
