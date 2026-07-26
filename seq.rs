// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut separator = "\n".to_string();
    let mut nums = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            println!("Usage: seq [OPTION]... LAST\n  or:  seq [OPTION]... FIRST LAST\n  or:  seq [OPTION]... FIRST INCREMENT LAST\nPrint numbers from FIRST to LAST, in steps of INCREMENT.\n\n  -s, --separator=STRING   use STRING to separate numbers (default: \\n)\n      --help               display this help and exit");
            return;
        } else if arg == "-s" {
            if i + 1 >= args.len() {
                eprintln!("seq: option requires an argument -- '-s'");
                process::exit(1);
            }
            separator = args[i + 1].clone();
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-s") {
            separator = val.to_string();
        } else if let Some(val) = arg.strip_prefix("--separator=") {
            separator = val.to_string();
        } else if arg.starts_with('-') && arg.len() > 1 && !arg.chars().nth(1).unwrap().is_ascii_digit() {
            eprintln!("seq: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            nums.push(arg.clone());
        }
        i += 1;
    }

    if nums.is_empty() || nums.len() > 3 {
        eprintln!("seq: invalid number of operands\nTry 'seq --help' for more information.");
        process::exit(1);
    }

    let is_float = nums.iter().any(|s| s.contains('.'));

    if !is_float {
        let parsed: Result<Vec<i64>, _> = nums.iter().map(|s| s.parse::<i64>()).collect();
        if let Ok(vals) = parsed {
            let (first, step, last) = match vals.as_slice() {
                [l] => (1, 1, *l),
                [f, l] => (*f, 1, *l),
                [f, s, l] => (*f, *s, *l),
                _ => unreachable!(),
            };

            if step == 0 {
                eprintln!("seq: zero increment");
                process::exit(1);
            }

            let mut current = first;
            let mut first_print = true;
            while (step > 0 && current <= last) || (step < 0 && current >= last) {
                if !first_print { print!("{}", separator); }
                print!("{}", current);
                first_print = false;
                match current.checked_add(step) {
                    Some(next) => current = next,
                    None => break,
                }
            }
            if !first_print { println!(); }
            return;
        }
    }

    let parsed: Result<Vec<f64>, _> = nums.iter().map(|s| s.parse::<f64>()).collect();
    match parsed {
        Ok(vals) => {
            let (first, step, last) = match vals.as_slice() {
                [l] => (1.0, 1.0, *l),
                [f, l] => (*f, 1.0, *l),
                [f, s, l] => (*f, *s, *l),
                _ => unreachable!(),
            };

            if step == 0.0 {
                eprintln!("seq: zero increment");
                process::exit(1);
            }

            let mut current = first;
            let mut first_print = true;
            let epsilon = 1e-10;
            while (step > 0.0 && current <= last + epsilon) || (step < 0.0 && current >= last - epsilon) {
                if !first_print { print!("{}", separator); }
                if current.fract().abs() < epsilon {
                    print!("{:.0}", current);
                } else {
                    print!("{}", current);
                }
                first_print = false;
                current += step;
            }
            if !first_print { println!(); }
        }
        Err(_) => {
            eprintln!("seq: invalid floating point argument");
            process::exit(1);
        }
    }
}
