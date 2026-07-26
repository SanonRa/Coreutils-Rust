// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

struct FindOpts {
    name_pattern: Option<String>,
    file_type: Option<char>,
    delete: bool,
    exec_cmd: Option<Vec<String>>,
}

fn glob_match(pattern: &str, text: &str) -> bool {
    let mut p_chars = pattern.chars().peekable();
    let mut t_chars = text.chars().peekable();
    while let Some(p) = p_chars.next() {
        if p == '*' {
            if p_chars.peek().is_none() { return true; }
            let next_p = *p_chars.peek().unwrap();
            while let Some(t) = t_chars.peek() {
                if *t == next_p && glob_match(&pattern[pattern.find('*').unwrap() + 1..], &text[text.len() - t_chars.clone().count()..]) {
                    return true;
                }
                t_chars.next();
            }
            return false;
        } else if p == '?' {
            if t_chars.next().is_none() { return false; }
        } else if Some(p) != t_chars.next() {
            return false;
        }
    }
    t_chars.next().is_none()
}

fn matches(path: &Path, opts: &FindOpts) -> bool {
    if let Some(ref pat) = opts.name_pattern {
        let name = path.file_name().to_string_lossy();
        if !glob_match(pat, &name) { return false; }
    }
    if let Some(ft) = opts.file_type {
        let meta = match fs::symlink_metadata(path) { Ok(m) => m, Err(_) => return false };
        let is_match = match ft {
            'f' => meta.is_file(),
            'd' => meta.is_dir(),
            'l' => meta.file_type().is_symlink(),
            _ => true,
        };
        if !is_match { return false; }
    }
    true
}

fn walk_dir(path: &Path, opts: &FindOpts) {
    if matches(path, opts) {
        println!("{}", path.display());
        if let Some(ref cmd) = opts.exec_cmd {
            let args: Vec<String> = cmd.iter().map(|s| if s == "{}" { path.to_string_lossy().to_string() } else { s.clone() }).collect();
            if !args.is_empty() {
                let _ = Command::new(&args[0]).args(&args[1..]).status();
            }
        }
        if opts.delete {
            if path.is_dir() { let _ = fs::remove_dir(path); }
            else { let _ = fs::remove_file(path); }
        }
    }

    if path.is_dir() && (!opts.delete || !matches(path, opts)) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                walk_dir(&entry.path(), opts);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut roots = Vec::new();
    let mut opts = FindOpts { name_pattern: None, file_type: None, delete: false, exec_cmd: None };
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-name" => {
                if i + 1 >= args.len() { eprintln!("find: missing argument to '-name'"); process::exit(1); }
                opts.name_pattern = Some(args[i + 1].clone()); i += 1;
            }
            "-type" => {
                if i + 1 >= args.len() { eprintln!("find: missing argument to '-type'"); process::exit(1); }
                opts.file_type = args[i + 1].chars().next(); i += 1;
            }
            "-delete" => opts.delete = true,
            "-print" => {} // Default behavior
            "-exec" => {
                let mut cmd = Vec::new();
                i += 1;
                while i < args.len() && args[i] != ";" && args[i] != "\\;" {
                    cmd.push(args[i].clone()); i += 1;
                }
                opts.exec_cmd = Some(cmd);
            }
            "--help" => {
                println!("Usage: find [PATH]... [EXPRESSION]\nSearch for files in a directory hierarchy.\n\n  -delete         delete matched empty directories or files\n  -exec CMD ;     execute CMD (replace {{}} with path)\n  -name PATTERN   match base of file name against PATTERN\n  -print          print full file name on standard output\n  -type [f|d|l]   filter by file type\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => { eprintln!("find: unknown predicate '{}'", arg); process::exit(1); }
            _ => roots.push(PathBuf::from(arg)),
        }
        i += 1;
    }

    if roots.is_empty() { roots.push(PathBuf::from(".")); }
    for root in roots { walk_dir(&root, &opts); }
}
