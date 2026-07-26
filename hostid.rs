// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::process;

#[cfg(unix)]
extern "C" {
    fn gethostid() -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: hostid [OPTION]...\nPrint the numeric identifier (in hexadecimal) for the current host.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("hostid: unrecognized option '{}'\nTry 'hostid --help' for more information.", arg);
            process::exit(1);
        }
    }

    #[cfg(unix)]
    {
        let id = unsafe { gethostid() };
        println!("{:08x}", id as u32);
    }
    #[cfg(not(unix))]
    {
        eprintln!("hostid: unsupported platform");
        process::exit(1);
    }
}
