// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::io::{self, BufRead};
use std::process;

enum Mode { None, Si, Iec }

fn format_num(val: f64, mode: &Mode) -> String {
    match mode {
        Mode::None => format!("{}", val as i64),
        Mode::Si => {
            const UNITS: &[&str] = &["", "K", "M", "G", "T", "P", "E"];
            let mut size = val; let mut idx = 0;
            while size >= 1000.0 && idx < UNITS.len() - 1 { size /= 1000.0; idx += 1; }
            if idx == 0 { format!("{}", val as i64) }
            else if size >= 10.0 { format!("{:.0}{}", size, UNITS[idx]) }
            else { format!("{:.1}{}", size, UNITS[idx]) }
        }
        Mode::Iec => {
            const UNITS: &[&str] = &["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei"];
            let mut size = val; let mut idx = 0;
            while size >= 1024.0 && idx < UNITS.len() - 1 { size /= 1024.0; idx += 1; }
            if idx == 0 { format!("{}", val as i64) }
            else if size >= 10.0 { format!("{:.0}{}", size, UNITS[idx]) }
            else { format!("{:.1}{}", size, UNITS[idx]) }
        }
    }
}

fn parse_num(s: &str, mode: &Mode) -> Option<f64> {
    let mut num_str = s.trim().to_string();
    let mut mult = 1.0;
    let base = match mode { Mode::Iec => 1024.0, _ => 1000.0 };

    if num_str.ends_with('i') { num_str.pop(); }
    if let Some(c) = num_str.chars().last() {
        match c.to_ascii_uppercase() {
            'K' => { mult = base; num_str.pop(); }
            'M' => { mult = base.powi(2); num_str.pop(); }
            'G' => { mult = base.powi(3); num_str.pop(); }
            'T' => { mult = base.powi(4); num_str.pop(); }
            'P' => { mult = base.powi(5); num_str.pop(); }
            'E' => { mult = base.powi(6); num_str.pop(); }
            _ => {}
        }
    }
    num_str.parse::<f64>().ok().map(|n| n * mult)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut to_mode = Mode::None;
    let mut from_mode = Mode::None;
    let mut operands = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--to=si" => to_mode = Mode::Si,
            "--to=iec" => to_mode = Mode::Iec,
            "--from=si" => from_mode = Mode::Si,
            "--from=iec" => from_mode = Mode::Iec,
            "--help" => {
                println!("Usage: numfmt [OPTION]... [NUMBER]...\nReformat NUMBER(s), or the numbers from standard input.\n\n  --from=si|iec   auto-scale input numbers to SI (1000) or IEC (1024)\n  --to=si|iec     auto-scale output numbers to SI (1000) or IEC (1024)\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("numfmt: unrecognized option '{}'", arg); process::exit(1); }
            _ => operands.push(arg.clone()),
        }
    }

    let process_val = |word: &str| {
        match parse_num(word, &from_mode) {
            Some(val) => println!("{}", format_num(val, &to_mode)),
            None => { eprintln!("numfmt: invalid number: '{}'", word); process::exit(1); }
        }
    };

    if operands.is_empty() {
        let stdin = io::stdin().lock();
        for line in stdin.lines().map_while(Result::ok) {
            for word in line.split_whitespace() { process_val(word); }
        }
    } else {
        for word in operands { process_val(&word); }
    }
}
