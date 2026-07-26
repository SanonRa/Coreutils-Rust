// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => {
                println!("Usage: hostname [OPTION]...\nPrint or set the system's host name.\n\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("hostname: invalid option -- '{}'\nTry 'hostname --help' for more information.", arg);
                process::exit(1);
            }
            _ => {
                eprintln!("hostname: setting hostname is not supported in this lightweight utility");
                process::exit(1);
            }
        }
    }

    let host = fs::read_to_string("/proc/sys/kernel/hostname")
        .or_else(|_| fs::read_to_string("/etc/hostname"))
        .map(|s| s.trim().to_string())
        .or_else(|_| env::var("HOSTNAME"))
        .or_else(|_| env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| {
            eprintln!("hostname: cannot determine hostname");
            process::exit(1);
        });

    println!("{}", host);
}
