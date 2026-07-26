// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::ffi::CString;
use std::process;

#[cfg(unix)]
extern "C" {
    fn mkfifo(path: *const std::ffi::c_char, mode: u32) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fifos = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: mkfifo [OPTION]... NAME...\nCreate named pipes (FIFOs) with the given NAMEs.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("mkfifo: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            fifos.push(arg.clone());
        }
    }

    if fifos.is_empty() {
        eprintln!("mkfifo: missing operand\nTry 'mkfifo --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    #[cfg(unix)]
    for fifo in fifos {
        if let Ok(c_path) = CString::new(fifo.clone()) {
            let res = unsafe { mkfifo(c_path.as_ptr(), 0o666) };
            if res != 0 {
                eprintln!("mkfifo: cannot create fifo '{}': {}", fifo, std::io::Error::last_os_error());
                exit_code = 1;
            }
        } else {
            eprintln!("mkfifo: invalid path '{}'", fifo);
            exit_code = 1;
        }
    }
    #[cfg(not(unix))]
    {
        eprintln!("mkfifo: named pipes are not supported on this platform");
        exit_code = 1;
    }
    process::exit(exit_code);
}
