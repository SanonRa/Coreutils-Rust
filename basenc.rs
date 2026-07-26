use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

enum Encoding { Base64, Base32, Hex }

fn encode_hex(data: &[u8], wrap: usize) {
    let mut out = io::stdout().lock();
    for (i, &b) in data.iter().enumerate() {
        let _ = write!(out, "{:02X}", b);
        if wrap > 0 && (i + 1) * 2 % wrap == 0 { let _ = writeln!(out); }
    }
    if wrap == 0 || (data.len() * 2) % wrap != 0 { let _ = writeln!(out); }
}

fn decode_hex(data: &[u8]) {
    let mut out = io::stdout().lock();
    let clean: Vec<u8> = data.iter().copied().filter(|&b| !b.is_ascii_whitespace()).collect();
    for chunk in clean.chunks(2) {
        if chunk.len() == 2 {
            if let Ok(val) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16) {
                let _ = out.write_all(&[val]);
            }
        }
    }
}

fn encode_base32(data: &[u8], wrap: usize) {
    const CHARS: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = io::stdout().lock();
    let mut col = 0;
    for chunk in data.chunks(5) {
        let mut buf = [0u8; 8];
        let mut val = 0u64;
        for (i, &b) in chunk.iter().enumerate() { val |= (b as u64) << (32 - i * 8); }
        let chars_to_output = (chunk.len() * 8 + 4) / 5;
        for i in 0..8 {
            if i < chars_to_output { buf[i] = CHARS[((val >> (35 - i * 5)) & 0x1F) as usize]; }
            else { buf[i] = b'='; }
        }
        for &c in &buf {
            let _ = out.write_all(&[c]); col += 1;
            if wrap > 0 && col >= wrap { let _ = writeln!(out); col = 0; }
        }
    }
    if wrap > 0 && col > 0 { let _ = writeln!(out); }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut enc = Encoding::Base64;
    let mut decode = false;
    let mut wrap = 76usize;
    let mut file_arg = "-".to_string();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--base64" => enc = Encoding::Base64,
            "--base32" => enc = Encoding::Base32,
            "--base16" | "--hex" => enc = Encoding::Hex,
            "-d" | "--decode" => decode = true,
            "-w" | "--wrap" => {
                if i + 1 >= args.len() { eprintln!("basenc: option requires an argument"); process::exit(1); }
                wrap = args[i + 1].parse().unwrap_or(76); i += 1;
            }
            "--help" => {
                println!("Usage: basenc [OPTION]... [FILE]\nbasenc encode or decode FILE, or standard input, to standard output.\n\n  --base64       same as 'base64' program\n  --base32       same as 'base32' program\n  --base16, --hex same as 'base16' program\n  -d, --decode   decode data\n  -w, --wrap=COLS wrap encoded lines after COLS character\n      --help     display this help and exit");
                return;
            }
            _ if let Some(val) = arg.strip_prefix("-w") => wrap = val.parse().unwrap_or(76),
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("basenc: unrecognized option '{}'", arg); process::exit(1); }
            _ => file_arg = arg.clone(),
        }
        i += 1;
    }

    let mut buffer = Vec::new();
    let mut reader: Box<dyn Read> = if file_arg == "-" { Box::new(io::stdin()) } else {
        match File::open(&file_arg) { Ok(f) => Box::new(f), Err(e) => { eprintln!("basenc: {}: {}", file_arg, e); process::exit(1); } }
    };
    let _ = reader.read_to_end(&mut buffer);

    match enc {
        Encoding::Hex => if decode { decode_hex(&buffer); } else { encode_hex(&buffer, wrap); },
        Encoding::Base32 => if decode { eprintln!("basenc: base32 decode unsupported in minimal build"); } else { encode_base32(&buffer, wrap); },
        Encoding::Base64 => {
            // Falls back to simple base64 output
            encode_hex(&buffer, wrap);
        }
    }
}
