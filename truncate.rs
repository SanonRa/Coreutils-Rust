use std::env;
use std::fs::{self, OpenOptions};
use std::process;

fn parse_size(s: &str, current_size: u64) -> Option<u64> {
    let mut num_str = s.to_string();
    let mut mult = 1u64;
    if let Some(c) = s.chars().last() {
        match c.to_ascii_uppercase() {
            'K' => { mult = 1024; num_str.pop(); }
            'M' => { mult = 1024 * 1024; num_str.pop(); }
            'G' => { mult = 1024 * 1024 * 1024; num_str.pop(); }
            'T' => { mult = 1024 * 1024 * 1024 * 1024; num_str.pop(); }
            _ => {}
        }
    }

    let is_add = num_str.starts_with('+');
    let is_sub = num_str.starts_with('-');
    if is_add || is_sub { num_str.remove(0); }

    let val = num_str.parse::<u64>().ok()? * mult;
    if is_add { Some(current_size.saturating_add(val)) }
    else if is_sub { Some(current_size.saturating_sub(val)) }
    else { Some(val) }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut no_create = false;
    let mut size_spec: Option<String> = None;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-c" | "--no-create" => no_create = true,
            "-s" | "--size" => {
                if i + 1 >= args.len() { eprintln!("truncate: option requires an argument"); process::exit(1); }
                size_spec = Some(args[i + 1].clone()); i += 1;
            }
            "--help" => {
                println!("Usage: truncate OPTION... FILE...\nShrink or extend the size of each FILE to the specified size.\n\n  -c, --no-create   do not create any files\n  -s, --size=SIZE   set or adjust the file size by SIZE bytes\n      --help        display this help and exit");
                return;
            }
            _ if let Some(val) = arg.strip_prefix("-s") => size_spec = Some(val.to_string()),
            _ if arg.starts_with('-') => { eprintln!("truncate: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    let spec = match size_spec {
        Some(s) => s,
        None => { eprintln!("truncate: you must specify a size via '-s'"); process::exit(1); }
    };

    if files.is_empty() { eprintln!("truncate: missing file operand"); process::exit(1); }

    let mut exit_code = 0;
    for file in files {
        if !fs::metadata(&file).is_ok() && no_create { continue; }
        match OpenOptions::new().write(true).create(!no_create).open(&file) {
            Ok(f) => {
                let current_size = f.metadata().map(|m| m.len()).unwrap_or(0);
                if let Some(target) = parse_size(&spec, current_size) {
                    if let Err(e) = f.set_len(target) {
                        eprintln!("truncate: cannot resize '{}': {}", file, e);
                        exit_code = 1;
                    }
                } else {
                    eprintln!("truncate: invalid size '{}'", spec);
                    exit_code = 1;
                }
            }
            Err(e) => {
                eprintln!("truncate: cannot open '{}': {}", file, e);
                exit_code = 1;
            }
        }
    }
    process::exit(exit_code);
}
