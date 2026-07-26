use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn parse_num(s: &str) -> usize {
    let mut num_str = s.to_string();
    let mut mult = 1;
    if let Some(c) = s.chars().last() {
        match c.to_ascii_uppercase() {
            'K' => { mult = 1024; num_str.pop(); }
            'M' => { mult = 1024 * 1024; num_str.pop(); }
            'G' => { mult = 1024 * 1024 * 1024; num_str.pop(); }
            _ => {}
        }
    }
    num_str.parse::<usize>().unwrap_or(512) * mult
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut if_path: Option<String> = None;
    let mut of_path: Option<String> = None;
    let mut bs = 512;
    let mut count: Option<usize> = None;

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: dd [OPERAND]...\nCopy a file, converting and formatting according to the operands.\n\n  bs=BYTES        read and write up to BYTES bytes at a time (default: 512)\n  count=N         copy only N input blocks\n  if=FILE         read from FILE instead of stdin\n  of=FILE         write to FILE instead of stdout\n      --help      display this help and exit");
            return;
        } else if let Some(val) = arg.strip_prefix("if=") { if_path = Some(val.to_string()); }
        else if let Some(val) = arg.strip_prefix("of=") { of_path = Some(val.to_string()); }
        else if let Some(val) = arg.strip_prefix("bs=") { bs = parse_num(val); }
        else if let Some(val) = arg.strip_prefix("count=") { count = Some(parse_num(val)); }
    }

    let mut reader: Box<dyn Read> = match if_path {
        Some(ref p) => match File::open(p) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("dd: failed to open '{}': {}", p, e); process::exit(1); }
        },
        None => Box::new(io::stdin()),
    };

    let mut writer: Box<dyn Write> = match of_path {
        Some(ref p) => match File::create(p) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("dd: failed to open '{}': {}", p, e); process::exit(1); }
        },
        None => Box::new(io::stdout()),
    };

    let mut buffer = vec![0u8; bs];
    let mut blocks_in_full = 0;
    let mut blocks_in_part = 0;
    let mut blocks_out_full = 0;
    let mut blocks_out_part = 0;
    let mut total_bytes = 0u64;
    let mut current_blocks = 0;

    while count.map_or(true, |c| current_blocks < c) {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                if n == bs { blocks_in_full += 1; } else { blocks_in_part += 1; }
                if let Err(e) = writer.write_all(&buffer[..n]) {
                    eprintln!("dd: error writing: {}", e);
                    break;
                }
                if n == bs { blocks_out_full += 1; } else { blocks_out_part += 1; }
                total_bytes += n as u64;
                current_blocks += 1;
            }
            Err(e) => {
                eprintln!("dd: error reading: {}", e);
                break;
            }
        }
    }
    let _ = writer.flush();

    eprintln!("{}+{} records in", blocks_in_full, blocks_in_part);
    eprintln!("{}+{} records out", blocks_out_full, blocks_out_part);
    eprintln!("{} bytes copied", total_bytes);
}
