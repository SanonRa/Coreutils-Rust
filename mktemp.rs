// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn random_string(len: usize, seed: u64) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut state = seed;
    let mut result = String::with_capacity(len);
    for _ in 0..len {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let idx = ((state >> 33) as usize) % CHARS.len();
        result.push(CHARS[idx] as char);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut directory = false;
    let mut quiet = false;
    let mut template: Option<String> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-d" | "--directory" => directory = true,
            "-q" | "--quiet" => quiet = true,
            "--help" => {
                println!("Usage: mktemp [OPTION]... [TEMPLATE]\nCreate a temporary file or directory, safely, and print its name.\nTEMPLATE must contain at least 3 consecutive 'X's in last component.\nIf TEMPLATE is not specified, use tmp.XXXXXXXXXX.\n\n  -d, --directory   create a directory, not a file\n  -q, --quiet       suppress diagnostics about file/dir-creation failure\n      --help        display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                if !quiet { eprintln!("mktemp: unrecognized option '{}'", arg); }
                process::exit(1);
            }
            _ => template = Some(arg.clone()),
        }
    }

    let tmpl = template.unwrap_or_else(|| {
        let mut p = env::temp_dir();
        p.push("tmp.XXXXXXXXXX");
        p.to_string_lossy().to_string()
    });

    if !tmpl.contains("XXX") {
        if !quiet { eprintln!("mktemp: too few X's in template '{}'", tmpl); }
        process::exit(1);
    }

    let seed_base = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(12345) ^ (process::id() as u64);

    for attempt in 0..100 {
        let rand_part = random_string(10, seed_base.wrapping_add(attempt));
        let mut path_str = tmpl.clone();
        while let Some(idx) = path_str.rfind('X') {
            let end = idx + 1;
            let start = path_str[..end].rfind(|c| c != 'X').map(|i| i + 1).unwrap_or(0);
            let x_count = end - start;
            let replacement = &rand_part[..x_count.min(rand_part.len())];
            path_str.replace_range(start..end, replacement);
            break;
        }

        let path = PathBuf::from(&path_str);
        if directory {
            if fs::create_dir(&path).is_ok() {
                println!("{}", path.display());
                return;
            }
        } else if File::options().write(true).create_new(true).open(&path).is_ok() {
            println!("{}", path.display());
            return;
        }
    }

    if !quiet { eprintln!("mktemp: failed to create temporary file/directory via template '{}'", tmpl); }
    process::exit(1);
}
