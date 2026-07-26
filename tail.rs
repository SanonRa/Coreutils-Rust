use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lines_count: Option<usize> = Some(10);
    let mut bytes_count: Option<usize> = None;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            println!("Usage: tail [OPTION]... [FILE]...\nPrint the last 10 lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n\n  -c, --bytes=NUM   output the last NUM bytes\n  -n, --lines=NUM   output the last NUM lines, instead of the last 10\n      --help        display this help and exit");
            return;
        } else if arg == "-n" || arg == "-c" {
            if i + 1 >= args.len() {
                eprintln!("tail: option requires an argument -- '{}'", arg);
                process::exit(1);
            }
            let val = args[i + 1].parse::<usize>().unwrap_or(10);
            if arg == "-n" {
                lines_count = Some(val);
                bytes_count = None;
            } else {
                bytes_count = Some(val);
                lines_count = None;
            }
            i += 1;
        } else if let Some(val_str) = arg.strip_prefix("-n") {
            lines_count = Some(val_str.parse().unwrap_or(10));
            bytes_count = None;
        } else if let Some(val_str) = arg.strip_prefix("-c") {
            bytes_count = Some(val_str.parse().unwrap_or(0));
            lines_count = None;
        } else if arg.starts_with('-') && arg.len() > 1 && arg.chars().nth(1).unwrap().is_ascii_digit() {
            lines_count = Some(arg[1..].parse().unwrap_or(10));
            bytes_count = None;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("tail: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if files.is_empty() {
        files.push("-".to_string());
    }

    let print_headers = files.len() > 1;
    let mut stdout = io::stdout().lock();
    let mut first_file = true;

    for file in &files {
        if print_headers {
            if !first_file {
                let _ = writeln!(stdout);
            }
            let name = if file == "-" { "standard input" } else { file };
            let _ = writeln!(stdout, "==> {} <==", name);
            first_file = false;
        }

        let mut reader: Box<dyn Read> = if file == "-" {
            Box::new(io::stdin())
        } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => {
                    eprintln!("tail: cannot open '{}' for reading: {}", file, e);
                    continue;
                }
            }
        };

        if let Some(n) = lines_count {
            let buf_reader = BufReader::new(reader);
            let mut buffer = Vec::with_capacity(n + 1);
            for line in buf_reader.lines().map_while(Result::ok) {
                buffer.push(line);
                if buffer.len() > n {
                    buffer.remove(0);
                }
            }
            for line in buffer {
                let _ = writeln!(stdout, "{}", line);
            }
        } else if let Some(c) = bytes_count {
            let mut all_bytes = Vec::new();
            let _ = reader.read_to_end(&mut all_bytes);
            let start = all_bytes.len().saturating_sub(c);
            let _ = stdout.write_all(&all_bytes[start..]);
        }
    }
}
