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

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let prog = args[0].clone();

    if prog.ends_with('[') || args.get(0).map_or(false, |s| s == "[") {
        if args.last().map_or(true, |s| s != "]") {
            eprintln!("[: missing ']'");
            process::exit(2);
        }
        args.pop();
    }

    let tokens: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    if tokens.is_empty() { process::exit(1); }
    if tokens.len() == 1 {
        if tokens[0] == "--help" {
            println!("Usage: test EXPRESSION\n  or:  [ EXPRESSION ]\nEvaluate EXPRESSION and return exit status 0 (true) or 1 (false).\n\n      --help     display this help and exit");
            process::exit(0);
        }
        process::exit(if tokens[0].is_empty() { 1 } else { 0 });
    }

    if tokens.len() == 2 {
        let (op, arg) = (tokens[0], tokens[1]);
        let path = Path::new(arg);
        let res = match op {
            "-e" => path.exists(),
            "-f" => path.is_file(),
            "-d" => path.is_dir(),
            "-s" => fs::metadata(path).map_or(false, |m| m.len() > 0),
            "-r" => fs::File::open(path).is_ok(),
            "-w" => fs::OpenOptions::new().write(true).open(path).is_ok(),
            "-x" => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::metadata(path).map_or(false, |m| m.permissions().mode() & 0o111 != 0)
                }
                #[cfg(not(unix))]
                { path.exists() }
            }
            "-z" => arg.is_empty(),
            "-n" => !arg.is_empty(),
            _ => { eprintln!("test: unrecognized unary operator '{}'", op); process::exit(2); }
        };
        process::exit(if res { 0 } else { 1 });
    }

    if tokens.len() == 3 {
        let (left, op, right) = (tokens[0], tokens[1], tokens[2]);
        let res = match op {
            "=" | "==" => left == right,
            "!=" => left != right,
            "-eq" | "-ne" | "-gt" | "-lt" | "-ge" | "-le" => {
                let l = left.parse::<i64>().unwrap_or_else(|_| { eprintln!("test: integer expression expected"); process::exit(2); });
                let r = right.parse::<i64>().unwrap_or_else(|_| { eprintln!("test: integer expression expected"); process::exit(2); });
                match op {
                    "-eq" => l == r,
                    "-ne" => l != r,
                    "-gt" => l > r,
                    "-lt" => l < r,
                    "-ge" => l >= r,
                    "-le" => l <= r,
                    _ => unreachable!(),
                }
            }
            _ => { eprintln!("test: unrecognized binary operator '{}'", op); process::exit(2); }
        };
        process::exit(if res { 0 } else { 1 });
    }

    eprintln!("test: too many arguments");
    process::exit(2);
}
