// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::process;

fn check_path(path: &str, posix: bool, check_hyphen: bool) -> bool {
    if check_hyphen && (path.is_empty() || path.starts_with('-') || path.contains("/-")) {
        eprintln!("pathchk: leading hyphen or empty string in '{}'", path);
        return false;
    }

    if posix {
        if path.len() > 255 {
            eprintln!("pathchk: path too long (limit 255): '{}'", path);
            return false;
        }
        for comp in path.split('/') {
            if comp.len() > 14 {
                eprintln!("pathchk: component too long (limit 14): '{}'", comp);
                return false;
            }
            for c in comp.chars() {
                if !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '-' {
                    eprintln!("pathchk: non-portable character '{}' in '{}'", c, path);
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut posix = false;
    let mut check_hyphen = false;
    let mut paths = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-p" => posix = true,
            "-P" => check_hyphen = true,
            "-pP" | "-Pp" => { posix = true; check_hyphen = true; }
            "--help" => {
                println!("Usage: pathchk [OPTION]... NAME...\nDiagnose invalid or unportable file names.\n\n  -p          check for most POSIX systems\n  -P          check for empty names and leading hyphens\n      --help  display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("pathchk: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => paths.push(arg.clone()),
        }
    }

    if paths.is_empty() {
        eprintln!("pathchk: missing operand\nTry 'pathchk --help' for more information.");
        process::exit(1);
    }

    let mut ok = true;
    for path in paths {
        if !check_path(&path, posix, check_hyphen) {
            ok = false;
        }
    }
    process::exit(if ok { 0 } else { 1 });
}
