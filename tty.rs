// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::ffi::CStr;
use std::fs;
use std::io::{self, IsTerminal};
use std::process;

#[cfg(unix)]
extern "C" {
    fn ttyname(fd: i32) -> *const std::ffi::c_char;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut silent = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-s" | "--silent" | "--quiet" => silent = true,
            "--help" => {
                println!("Usage: tty [OPTION]...\nPrint the file name of the terminal connected to standard input.\n\n  -s, --silent, --quiet   print nothing, only return an exit status\n      --help              display this help and exit");
                return;
            }
            _ => {
                eprintln!("tty: unrecognized option '{}'\nTry 'tty --help' for more information.", arg);
                process::exit(2);
            }
        }
    }

    if !io::stdin().is_terminal() {
        if !silent {
            println!("not a tty");
        }
        process::exit(1);
    }

    if silent {
        process::exit(0);
    }

    if let Ok(target) = fs::read_link("/proc/self/fd/0") {
        println!("{}", target.display());
        return;
    }

    #[cfg(unix)]
    unsafe {
        let ptr = ttyname(0);
        if !ptr.is_null() {
            if let Ok(name) = CStr::from_ptr(ptr).to_str() {
                println!("{}", name);
                return;
            }
        }
    }

    println!("/dev/tty");
}
