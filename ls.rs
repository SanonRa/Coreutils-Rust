// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

struct LsOptions {
    all: bool,
    long: bool,
    one_per_line: bool,
    human: bool,
    reverse: bool,
    sort_time: bool,
}

fn format_size(bytes: u64, human: bool) -> String {
    if !human { return bytes.to_string(); }
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 { size /= 1024.0; unit_idx += 1; }
    if unit_idx == 0 { format!("{}", bytes) } else if size >= 10.0 { format!("{:.0}{}", size, UNITS[unit_idx]) } else { format!("{:.1}{}", size, UNITS[unit_idx]) }
}

#[cfg(unix)]
fn format_mode(mode: u32) -> String {
    let chars = ['r', 'w', 'x'];
    let mut res = String::with_capacity(10);
    res.push(if mode & 0o170000 == 0o040000 { 'd' } else if mode & 0o170000 == 0o120000 { 'l' } else { '-' });
    for i in 0..9 {
        if mode & (1 << (8 - i)) != 0 { res.push(chars[i % 3]); } else { res.push('-'); }
    }
    res
}

fn list_dir(dir: &Path, opts: &LsOptions) {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !opts.all && name.starts_with('.') { continue; }
            entries.push(entry);
        }
    }

    if opts.sort_time {
        entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).unwrap_or(UNIX_EPOCH));
    } else {
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }
    if opts.reverse { entries.reverse(); }

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        if opts.long {
            if let Ok(meta) = entry.metadata() {
                #[cfg(unix)]
                {
                    let mode = format_mode(meta.mode());
                    let nlink = meta.nlink();
                    let size = format_size(meta.size(), opts.human);
                    println!("{} {:>2} {:>5} {:>5} {:>8} {}", mode, nlink, meta.uid(), meta.gid(), size, name);
                }
                #[cfg(not(unix))]
                {
                    let size = format_size(meta.len(), opts.human);
                    let kind = if meta.is_dir() { "d" } else { "-" };
                    println!("{} {:>8} {}", kind, size, name);
                }
            } else {
                println!("?????????? ? ? ? ? {}", name);
            }
        } else if opts.one_per_line {
            println!("{}", name);
        } else {
            print!("{}  ", name);
        }
    }
    if !opts.long && !opts.one_per_line { println!(); }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = LsOptions { all: false, long: false, one_per_line: false, human: false, reverse: false, sort_time: false };
    let mut paths = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" | "--all" => opts.all = true,
            "-l" => opts.long = true,
            "-1" => opts.one_per_line = true,
            "-h" | "--human-readable" => opts.human = true,
            "-r" | "--reverse" => opts.reverse = true,
            "-t" => opts.sort_time = true,
            "-la" | "-al" => { opts.long = true; opts.all = true; }
            "-lah" | "-lha" | "-ahl" => { opts.long = true; opts.all = true; opts.human = true; }
            "--help" => {
                println!("Usage: ls [OPTION]... [FILE]...\nList information about the FILEs (the current directory by default).\n\n  -a, --all              do not ignore entries starting with .\n  -h, --human-readable   with -l and/or -s, print human readable sizes\n  -l                     use a long listing format\n  -r, --reverse          reverse order while sorting\n  -t                     sort by modification time, newest first\n  -1                     list one file per line\n      --help             display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => { eprintln!("ls: unrecognized option '{}'", arg); process::exit(1); }
            _ => paths.push(arg.clone()),
        }
    }

    if paths.is_empty() { paths.push(".".to_string()); }
    let multiple = paths.len() > 1;

    for (idx, path) in paths.iter().enumerate() {
        if multiple {
            if idx > 0 { println!(); }
            println!("{}:", path);
        }
        let p = Path::new(path);
        if p.is_dir() { list_dir(p, &opts); }
        else if p.exists() { println!("{}", path); }
        else { eprintln!("ls: cannot access '{}': No such file or directory", path); }
    }
}
