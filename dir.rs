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

fn escape_name(name: &str) -> String {
    let mut res = String::new();
    for c in name.chars() {
        match c {
            ' ' => res.push_str("\\ "),
            '\t' => res.push_str("\\t"),
            '\n' => res.push_str("\\n"),
            '\\' => res.push_str("\\\\"),
            _ => res.push(c),
        }
    }
    res
}

fn list_dir(dir: &Path, all: bool, long: bool, reverse: bool) {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !all && name.starts_with('.') { continue; }
            entries.push(entry);
        }
    }
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    if reverse { entries.reverse(); }

    for entry in entries {
        let name = escape_name(&entry.file_name().to_string_lossy());
        if long {
            if let Ok(meta) = entry.metadata() {
                println!("{:>8}  {}", meta.len(), name);
            } else {
                println!("????????  {}", name);
            }
        } else {
            print!("{}  ", name);
        }
    }
    if !long { println!(); }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut all = false;
    let mut long = false;
    let mut reverse = false;
    let mut paths = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" | "--all" => all = true,
            "-l" => long = true,
            "-r" | "--reverse" => reverse = true,
            "--help" => {
                println!("Usage: dir [OPTION]... [FILE]...\nList directory contents in columns.\n\n  -a, --all       do not ignore entries starting with .\n  -l              use a long listing format\n  -r, --reverse   reverse order while sorting\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => { eprintln!("dir: unrecognized option '{}'", arg); process::exit(1); }
            _ => paths.push(arg.clone()),
        }
    }

    if paths.is_empty() { paths.push(".".to_string()); }
    for path in paths {
        let p = Path::new(&path);
        if p.is_dir() { list_dir(p, all, long, reverse); }
        else if p.exists() { println!("{}", escape_name(&path)); }
        else { eprintln!("dir: cannot access '{}': No such file or directory", path); }
    }
}
