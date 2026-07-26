// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::OpenOptions;
use std::io::{self, IsTerminal};
use std::process::{self, Command, Stdio};

#[cfg(unix)]
extern "C" {
    fn signal(sig: i32, cb: usize) -> usize;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" {
        println!("Usage: nohup COMMAND [ARG]...\nRun COMMAND, ignoring hangup signals.\n\n      --help     display this help and exit");
        process::exit(if args.len() < 2 { 125 } else { 0 });
    }

    #[cfg(unix)]
    unsafe {
        signal(1, 1); // SIGHUP, SIG_IGN
    }

    let cmd_name = &args[1];
    let cmd_args = &args[2..];
    let mut command = Command::new(cmd_name);
    command.args(cmd_args);

    if io::stdout().is_terminal() {
        let out_path = "nohup.out";
        match OpenOptions::new().write(true).create(true).append(true).open(out_path) {
            Ok(file) => {
                eprintln!("nohup: ignoring input and appending output to '{}'", out_path);
                command.stdout(Stdio::from(file.try_clone().unwrap()));
                if io::stderr().is_terminal() {
                    command.stderr(Stdio::from(file));
                }
            }
            Err(_) => {
                let home_out = env::var("HOME").map(|h| format!("{}/nohup.out", h)).unwrap_or_else(|_| "nohup.out".to_string());
                if let Ok(file) = OpenOptions::new().write(true).create(true).append(true).open(&home_out) {
                    eprintln!("nohup: ignoring input and appending output to '{}'", home_out);
                    command.stdout(Stdio::from(file.try_clone().unwrap()));
                    if io::stderr().is_terminal() {
                        command.stderr(Stdio::from(file));
                    }
                } else {
                    eprintln!("nohup: failed to open nohup.out for writing");
                    process::exit(125);
                }
            }
        }
    }

    match command.status() {
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("nohup: failed to run command '{}': {}", cmd_name, e);
            if e.kind() == std::io::ErrorKind::NotFound { process::exit(127); }
            else { process::exit(126); }
        }
    }
}
