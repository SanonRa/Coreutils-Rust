use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut width = 80;
    let mut spaces = false;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-s" || arg == "--spaces" {
            spaces = true;
        } else if arg == "-w" || arg == "--width" {
            if i + 1 >= args.len() {
                eprintln!("fold: option requires an argument -- '{}'", arg);
                process::exit(1);
            }
            width = args[i + 1].parse().unwrap_or(80);
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-w") {
            width = val.parse().unwrap_or(80);
        } else if let Some(val) = arg.strip_prefix("--width=") {
            width = val.parse().unwrap_or(80);
        } else if arg == "--help" {
            println!("Usage: fold [OPTION]... [FILE]...\nWrap input lines in each FILE, writing to standard output.\n\n  -s, --spaces        break at spaces\n  -w, --width=WIDTH   use WIDTH columns instead of 80\n      --help          display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("fold: unrecognized option '{}'", arg);
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
                Err(e) => { eprintln!("fold: {}: {}", file, e); continue; }
            }
        };

        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            let chars: Vec<char> = line.chars().collect();
            let mut start = 0;
            while start < chars.len() {
                let remaining = chars.len() - start;
                if remaining <= width {
                    let chunk: String = chars[start..].iter().collect();
                    println!("{}", chunk);
                    break;
                }

                let mut break_idx = width;
                if spaces {
                    if let Some(pos) = chars[start..start + width].iter().rposition(|&c| c.is_whitespace()) {
                        if pos > 0 { break_idx = pos + 1; }
                    }
                }

                let chunk: String = chars[start..start + break_idx].iter().collect();
                println!("{}", chunk);
                start += break_idx;
            }
        }
    }
}
