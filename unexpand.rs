use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut all = false;
    let mut tab_size = 8;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-a" || arg == "--all" {
            all = true;
        } else if arg == "-t" || arg == "--tabs" {
            if i + 1 >= args.len() {
                eprintln!("unexpand: option requires an argument -- '{}'", arg);
                process::exit(1);
            }
            tab_size = args[i + 1].parse().unwrap_or(8);
            all = true;
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-t") {
            tab_size = val.parse().unwrap_or(8);
            all = true;
        } else if let Some(val) = arg.strip_prefix("--tabs=") {
            tab_size = val.parse().unwrap_or(8);
            all = true;
        } else if arg == "--help" {
            println!("Usage: unexpand [OPTION]... [FILE]...\nConvert blanks in each FILE to tabs, writing to standard output.\n\n  -a, --all           convert all blanks, instead of just initial blanks\n  -t, --tabs=NUMBER   have tabs NUMBER characters apart instead of 8 (enables -a)\n      --help          display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("unexpand: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    for file in &files {
        let reader: Box<dyn Read> = if file == "-" {
            Box::new(io::stdin())
        } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => { eprintln!("unexpand: {}: {}", file, e); continue; }
            }
        };

        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            let mut col = 0;
            let mut space_count = 0;
            let mut leading = true;

            for ch in line.chars() {
                if ch == ' ' && (leading || all) {
                    space_count += 1;
                    col += 1;
                    if col % tab_size == 0 && space_count > 1 {
                        print!("\t");
                        space_count = 0;
                    }
                } else if ch == '\t' && (leading || all) {
                    print!("\t");
                    col += tab_size - (col % tab_size);
                    space_count = 0;
                } else {
                    for _ in 0..space_count { print!(" "); }
                    space_count = 0;
                    print!("{}", ch);
                    col += 1;
                    if ch != ' ' && ch != '\t' { leading = false; }
                }
            }
            for _ in 0..space_count { print!(" "); }
            println!();
        }
    }
}
