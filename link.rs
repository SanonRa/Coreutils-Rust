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

    if args.len() == 2 && args[1] == "--help" {
        println!("Usage: link FILE1 FILE2\n  or:  link OPTION\nCall the link function to create a link named FILE2 to an existing FILE1.\n\n      --help     display this help and exit");
        return;
    }

    if args.len() != 3 {
        eprintln!("link: missing operand\nTry 'link --help' for more information.");
        process::exit(1);
    }

    if let Err(e) = fs::hard_link(&args[1], &args[2]) {
        eprintln!("link: cannot create link '{}' to '{}': {}", args[2], args[1], e);
        process::exit(1);
    }
}
