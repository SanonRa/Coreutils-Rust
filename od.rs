use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

#[derive(Clone, Copy, PartialEq)]
enum Format { Octal, Hex, Char }

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut format = Format::Octal;
    let mut addr_radix = 'o';
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-t" => {
                let spec = args.get(i + 1).map(|s| s.as_str()).unwrap_or("o");
                format = match spec { "x1" | "x" => Format::Hex, "c" => Format::Char, _ => Format::Octal };
                i += 1;
            }
            "-A" => {
                addr_radix = args.get(i + 1).and_then(|s| s.chars().next()).unwrap_or('o');
                i += 1;
            }
            "--help" => {
                println!("Usage: od [OPTION]... [FILE]...\nWrite an unambiguous representation, octal bytes by default, of FILE\nto standard output.\n\n  -A, --address-radix=RADIX   output format for file offsets; RADIX is one\n                              of [doxn], for Decimal, Octal, Hex or None\n  -t, --format=TYPE           select output format or formats\n      --help                  display this help and exit");
                return;
            }
            _ if arg.starts_with("-t") => {
                let val = arg.strip_prefix("-t").unwrap();
                format = match val { "x1" | "x" => Format::Hex, "c" => Format::Char, _ => Format::Octal };
            }
            _ if arg.starts_with("-A") => {
                let val = arg.strip_prefix("-A").unwrap();
                addr_radix = val.chars().next().unwrap_or('o');
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("od: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    let mut offset = 0usize;
    let mut buffer = [0u8; 16];

    for file in &files {
        let mut reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => { eprintln!("od: {}: {}", file, e); continue; }
            }
        };

        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 { break; }
            match addr_radix {
                'd' => print!("{:07} ", offset),
                'o' => print!("{:07o} ", offset),
                'x' => print!("{:06x} ", offset),
                'n' => {}
                _ => print!("{:07o} ", offset),
            }

            for &byte in &buffer[..n] {
                match format {
                    Format::Octal => print!(" {:03o}", byte),
                    Format::Hex => print!(" {:02x}", byte),
                    Format::Char => {
                        let c = byte as char;
                        if byte == b'\n' { print!("  \\n"); }
                        else if byte == b'\t' { print!("  \\t"); }
                        else if byte == b'\r' { print!("  \\r"); }
                        else if byte == b'\0' { print!("  \\0"); }
                        else if c.is_ascii_graphic() || c == ' ' { print!("   {}", c); }
                        else { print!(" {:03o}", byte); }
                    }
                }
            }
            println!();
            offset += n;
        }
    }

    match addr_radix {
        'd' => println!("{:07}", offset),
        'o' => println!("{:07o}", offset),
        'x' => println!("{:06x}", offset),
        _ => {}
    }
}
