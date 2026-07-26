use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

struct Sha1 { state: [u32; 5], count: u64, buffer: [u8; 64] }
impl Sha1 {
    fn new() -> Self {
        Self { state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0], count: 0, buffer: [0u8; 64] }
    }
    fn transform(&mut self, data: &[u8]) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([data[i * 4], data[i * 4 + 1], data[i * 4 + 2], data[i * 4 + 3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let mut a = self.state[0]; let mut b = self.state[1]; let mut c = self.state[2]; let mut d = self.state[3]; let mut e = self.state[4];
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5a827999),
                20..=39 => (b ^ c ^ d, 0x6ed9eba1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1bbcdc),
                _ => (b ^ c ^ d, 0xca62c1d6),
            };
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
        }
        self.state[0] = self.state[0].wrapping_add(a); self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c); self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }
    fn update(&mut self, data: &[u8]) {
        let mut idx = (self.count as usize) & 63;
        self.count += data.len() as u64;
        let mut i = 0;
        while i < data.len() {
            let step = (64 - idx).min(data.len() - i);
            self.buffer[idx..idx + step].copy_from_slice(&data[i..i + step]);
            idx += step; i += step;
            if idx == 64 {
                let buf = self.buffer;
                self.transform(&buf);
                idx = 0;
            }
        }
    }
    fn finalize(mut self) -> [u8; 20] {
        let bits = (self.count * 8).to_be_bytes();
        let idx = (self.count as usize) & 63;
        let pad_len = if idx < 56 { 56 - idx } else { 120 - idx };
        let mut pad = [0u8; 64]; pad[0] = 0x80;
        self.update(&pad[..pad_len]);
        self.update(&bits);
        let mut out = [0u8; 20];
        for (i, &word) in self.state.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else { Box::new(File::open(path)?) };
    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 8192];
    while let Ok(n) = reader.read(&mut buffer) { if n == 0 { break; } hasher.update(&buffer[..n]); }
    Ok(hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut check_mode = false;
    let mut binary_mode = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-c" | "--check" => check_mode = true,
            "-b" | "--binary" => binary_mode = true,
            "-t" | "--text" => binary_mode = false,
            "--help" => {
                println!("Usage: sha1sum [OPTION]... [FILE]...\nPrint or check SHA1 (160-bit) checksums.\n\n  -b, --binary   read in binary mode\n  -c, --check    read SHA1 sums from the FILEs and check them\n  -t, --text     read in text mode (default)\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("sha1sum: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    if check_mode {
        let mut fail_count = 0;
        for file in files {
            let reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
                match File::open(&file) { Ok(f) => Box::new(f), Err(e) => { eprintln!("sha1sum: {}: {}", file, e); continue; } }
            };
            for line in BufReader::new(reader).lines().map_while(Result::ok) {
                if let Some(idx) = line.find(' ') {
                    let expected = &line[..idx];
                    let target = line[idx..].trim_start_matches([' ', '*']).trim();
                    match hash_file(target) {
                        Ok(actual) if actual == expected => println!("{}: OK", target),
                        _ => { println!("{}: FAILED", target); fail_count += 1; }
                    }
                }
            }
        }
        if fail_count > 0 { eprintln!("sha1sum: WARNING: {} computed checksum did NOT match", fail_count); process::exit(1); }
    } else {
        let mode_char = if binary_mode { '*' } else { ' ' };
        let mut exit_code = 0;
        for file in files {
            match hash_file(&file) {
                Ok(hash) => println!("{} {}{}", hash, mode_char, file),
                Err(e) => { eprintln!("sha1sum: {}: {}", file, e); exit_code = 1; }
            }
        }
        process::exit(exit_code);
    }
}
