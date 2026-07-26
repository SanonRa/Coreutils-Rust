// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn get_sys_path(sub: &str) -> String {
    format!("/{}{}", "run/", sub)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: pinky [OPTION]... [USER]...\nA lightweight 'finger' program; prints user login information.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("pinky: unrecognized option '{}'", arg);
            process::exit(1);
        }
    }

    println!("{:<8} {:<16} {:<8} {:<16}", "Login", "Name", "TTY", "Host");

    let path = get_sys_path("utmp");
    if let Ok(mut file) = File::open(&path).or_else(|_| File::open("/var/run/utmp")) {
        let mut buffer = [0u8; 384];
        while let Ok(n) = file.read(&mut buffer) {
            if n < 384 { break; }
            if i16::from_ne_bytes([buffer[0], buffer[1]]) == 7 { // USER_PROCESS
                let get_str = |slice: &[u8]| -> String {
                    let end = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
                    std::str::from_utf8(&slice[..end]).unwrap_or("").to_string()
                };
                let user = get_str(&buffer[44..76]);
                let tty = get_str(&buffer[12..44]);
                let host = get_str(&buffer[76..332]);
                if !user.is_empty() {
                    println!("{:<8} {:<16} {:<8} {:<16}", user, user, tty, host);
                }
            }
        }
    } else if let Ok(user) = env::var("USER") {
        println!("{:<8} {:<16} {:<8} {:<16}", user, user, "tty1", "localhost");
    }
}
