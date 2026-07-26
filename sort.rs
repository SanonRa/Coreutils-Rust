// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

struct SortOptions {
    numeric: bool,
    reverse: bool,
    unique: bool,
    ignore_case: bool,
    key_field: Option<usize>,
}

fn extract_key<'a>(line: &'a str, field: Option<usize>) -> &'a str {
    match field {
        Some(f) if f > 0 => line.split_whitespace().nth(f - 1).unwrap_or(""),
        _ => line,
    }
}

fn compare_lines(a: &str, b: &str, opts: &SortOptions) -> Ordering {
    let key_a = extract_key(a, opts.key_field);
    let key_b = extract_key(b, opts.key_field);

    let ord = if opts.numeric {
        let num_a = key_a.trim().parse::<f64>().unwrap_or(f64::MIN);
        let num_b = key_b.trim().parse::<f64>().unwrap_or(f64::MIN);
        num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
    } else if opts.ignore_case {
        key_a.to_lowercase().cmp(&key_b.to_lowercase())
    } else {
        key_a.cmp(key_b)
    };

    if opts.reverse { ord.reverse() } else { ord }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = SortOptions { numeric: false, reverse: false, unique: false, ignore_case: false, key_field: None };
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-n" | "--numeric-sort" => opts.numeric = true,
            "-r" | "--reverse" => opts.reverse = true,
            "-u" | "--unique" => opts.unique = true,
            "-f" | "--ignore-case" => opts.ignore_case = true,
            "-k" => {
                if i + 1 >= args.len() { eprintln!("sort: option requires an argument -- '-k'"); process::exit(1); }
                opts.key_field = args[i + 1].parse().ok();
                i += 1;
            }
            "--help" => {
                println!("Usage: sort [OPTION]... [FILE]...\nWrite sorted concatenation of all FILE(s) to standard output.\n\n  -f, --ignore-case               fold lower case to upper case characters\n  -k, --key=KEYDEF                sort via a key; KEYDEF gives field number (1-based)\n  -n, --numeric-sort              compare according to string numerical value\n  -r, --reverse                   reverse the result of comparisons\n  -u, --unique                    output only the first of an equal run\n      --help                      display this help and exit");
                return;
            }
            _ if arg.starts_with("-k") => {
                let val = arg.strip_prefix("-k").unwrap();
                opts.key_field = val.parse().ok();
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("sort: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }
    let mut lines = Vec::new();

    for file in &files {
        let reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => { eprintln!("sort: {}: {}", file, e); continue; }
            }
        };
        for line in BufReader::new(reader).lines().map_while(Result::ok) { lines.push(line); }
    }

    lines.sort_by(|a, b| compare_lines(a, b, &opts));
    if opts.unique { lines.dedup_by(|a, b| compare_lines(a, b, &opts) == Ordering::Equal); }

    for line in lines { println!("{}", line); }
}
