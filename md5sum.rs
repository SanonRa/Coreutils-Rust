use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

struct Md5 { state: [u32; 4], count: u64, buffer: [u8; 64] }
impl Md5 {
    fn new() -> Self {
        Self { state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476], count: 0, buffer: [0u8; 64] }
    }
    fn transform(&mut self, data: &[u8]) {
        let mut m = [0u32; 16];
        for i in 0..16 {
            m[i] = u32::from_le_bytes([data[i * 4], data[i * 4 + 1], data[i * 4 + 2], data[i * 4 + 3]]);
        }
        let mut a = self.state[0]; let mut b = self.state[1]; let mut c = self.state[2]; let mut d = self.state[3];
        let s = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
            5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
            4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
        ];
        let k: [u32; 64] = [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
        ];
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let temp = d; d = c; c = b;
            b = b.wrapping_add(a.wrapping_add(f).wrapping_add(k[i]).wrapping_add(m[g]).rotate_left(s[i]));
            a = temp;
        }
        self.state[0] = self.state[0].wrapping_add(a); self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c); self.state[3] = self.state[3].wrapping_add(d);
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
    fn finalize(mut self) -> [u8; 16] {
        let bits = (self.count * 8).to_le_bytes();
        let idx = (self.count as usize) & 63;
        let pad_len = if idx < 56 { 56 - idx } else { 120 - idx };
        let mut pad = [0u8; 64]; pad[0] = 0x80;
        self.update(&pad[..pad_len]);
        self.update(&bits);
        let mut out = [0u8; 16];
        for (i, &word) in self.state.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
        }
        out
    }
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else { Box::new(File::open(path)?) };
    let mut hasher = Md5::new();
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
                println!("Usage: md5sum [OPTION]... [FILE]...\nPrint or check MD5 (128-bit) checksums.\n\n  -b, --binary   read in binary mode\n  -c, --check    read MD5 sums from the FILEs and check them\n  -t, --text     read in text mode (default)\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("md5sum: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    if check_mode {
        let mut fail_count = 0;
        for file in files {
            let reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
                match File::open(&file) { Ok(f) => Box::new(f), Err(e) => { eprintln!("md5sum: {}: {}", file, e); continue; } }
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
        if fail_count > 0 { eprintln!("md5sum: WARNING: {} computed checksum did NOT match", fail_count); process::exit(1); }
    } else {
        let mode_char = if binary_mode { '*' } else { ' ' };
        let mut exit_code = 0;
        for file in files {
            match hash_file(&file) {
                Ok(hash) => println!("{} {}{}", hash, mode_char, file),
                Err(e) => { eprintln!("md5sum: {}: {}", file, e); exit_code = 1; }
            }
        }
        process::exit(exit_code);
    }
}
