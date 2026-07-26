use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

struct Counts {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

fn count_stream<R: Read>(mut reader: R) -> io::Result<Counts> {
    let mut buffer = [0u8; 16384];
    let mut counts = Counts { lines: 0, words: 0, chars: 0, bytes: 0 };
    let mut in_word = false;

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        counts.bytes += n;
        for &byte in &buffer[..n] {
            if byte == b'\n' {
                counts.lines += 1;
            }
            if byte.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word {
                in_word = true;
                counts.words += 1;
            }
        }
        if let Ok(s) = std::str::from_utf8(&buffer[..n]) {
            counts.chars += s.chars().count();
        } else {
            counts.chars += n;
        }
    }
    Ok(counts)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut show_lines = false;
    let mut show_words = false;
    let mut show_chars = false;
    let mut show_bytes = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-l" | "--lines" => show_lines = true,
            "-w" | "--words" => show_words = true,
            "-m" | "--chars" => show_chars = true,
            "-c" | "--bytes" => show_bytes = true,
            "--help" => {
                println!("Usage: wc [OPTION]... [FILE]...\nPrint newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.\n\n  -c, --bytes   print the byte counts\n  -m, --chars   print the character counts\n  -l, --lines   print the newline counts\n  -w, --words   print the word counts\n      --help    display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("wc: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
    }

    if !show_lines && !show_words && !show_chars && !show_bytes {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }

    if files.is_empty() {
        files.push("-".to_string());
    }

    let mut total = Counts { lines: 0, words: 0, chars: 0, bytes: 0 };
    let multiple = files.len() > 1;

    for file in &files {
        let result = if file == "-" {
            count_stream(io::stdin())
        } else {
            File::open(file).and_then(count_stream)
        };

        match result {
            Ok(c) => {
                total.lines += c.lines;
                total.words += c.words;
                total.chars += c.chars;
                total.bytes += c.bytes;

                if show_lines { print!("{:8} ", c.lines); }
                if show_words { print!("{:8} ", c.words); }
                if show_chars { print!("{:8} ", c.chars); }
                if show_bytes { print!("{:8} ", c.bytes); }
                if file != "-" { println!("{}", file); } else { println!(); }
            }
            Err(e) => eprintln!("wc: {}: {}", file, e),
        }
    }

    if multiple {
        if show_lines { print!("{:8} ", total.lines); }
        if show_words { print!("{:8} ", total.words); }
        if show_chars { print!("{:8} ", total.chars); }
        if show_bytes { print!("{:8} ", total.bytes); }
        println!("total");
    }
}
