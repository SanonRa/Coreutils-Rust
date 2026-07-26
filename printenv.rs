// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut null_term = false;
    let mut vars = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-0" | "--null" => null_term = true,
            "--help" => {
                println!("Usage: printenv [OPTION]... [VARIABLE]...\nPrint the values of the specified environment VARIABLE(s).\nIf no VARIABLE is specified, print name and value pairs for them all.\n\n  -0, --null     end each output line with NUL, not newline\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("printenv: unrecognized option '{}'\nTry 'printenv --help' for more information.", arg);
                process::exit(1);
            }
            _ => vars.push(arg.clone()),
        }
    }

    let delimiter = if null_term { '\0' } else { '\n' };
    let mut stdout = io::stdout().lock();

    if vars.is_empty() {
        for (key, value) in env::vars() {
            let _ = write!(stdout, "{}={}{}", key, value, delimiter);
        }
    } else {
        let mut all_found = true;
        for var in vars {
            match env::var(&var) {
                Ok(val) => {
                    let _ = write!(stdout, "{}{}", val, delimiter);
                }
                Err(_) => {
                    all_found = false;
                }
            }
        }
        if !all_found {
            process::exit(1);
        }
    }
}
