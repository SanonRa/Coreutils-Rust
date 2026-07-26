use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

const IV: [u64; 8] = [
    0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
    0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
];

const SIGMA: [[usize; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

struct Blake2b {
    h: [u64; 8],
    t: [u64; 2],
    f: [u64; 2],
    buf: [u8; 128],
    buflen: usize,
}

impl Blake2b {
    fn new() -> Self {
        let mut h = IV;
        h[0] ^= 0x01010040; // digest_length=64, key_length=0, fanout=1, depth=1
        Self { h, t: [0; 2], f: [0; 2], buf: [0u8; 128], buflen: 0 }
    }

    fn compress(&mut self, last: bool) {
        let mut m = [0u64; 16];
        for i in 0..16 {
            m[i] = u64::from_le_bytes([
                self.buf[i * 8], self.buf[i * 8 + 1], self.buf[i * 8 + 2], self.buf[i * 8 + 3],
                self.buf[i * 8 + 4], self.buf[i * 8 + 5], self.buf[i * 8 + 6], self.buf[i * 8 + 7],
            ]);
        }

        let mut v = [0u64; 16];
        v[..8].copy_from_slice(&self.h);
        v[8..].copy_from_slice(&IV);
        v[12] ^= self.t[0];
        v[13] ^= self.t[1];
        if last { v[14] ^= !0u64; }

        let mut g = |a: usize, b: usize, c: usize, d: usize, x: u64, y: u64| {
            v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
            v[d] = (v[d] ^ v[a]).rotate_right(32);
            v[c] = v[c].wrapping_add(v[d]);
            v[b] = (v[b] ^ v[c]).rotate_right(24);
            v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
            v[d] = (v[d] ^ v[a]).rotate_right(16);
            v[c] = v[c].wrapping_add(v[d]);
            v[b] = (v[b] ^ v[c]).rotate_right(63);
        };

        for s in SIGMA {
            g(0, 4, 8, 12, m[s[0]], m[s[1]]);
            g(1, 5, 9, 13, m[s[2]], m[s[3]]);
            g(2, 6, 10, 14, m[s[4]], m[s[5]]);
            g(3, 7, 11, 15, m[s[6]], m[s[7]]);
            g(0, 5, 10, 15, m[s[8]], m[s[9]]);
            g(1, 6, 11, 12, m[s[10]], m[s[11]]);
            g(2, 7, 8, 13, m[s[12]], m[s[13]]);
            g(3, 4, 9, 14, m[s[14]], m[s[15]]);
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }

    fn update(&mut self, data: &[u8]) {
        let mut i = 0;
        while i < data.len() {
            if self.buflen == 128 {
                self.t[0] = self.t[0].wrapping_add(128);
                if self.t[0] < 128 { self.t[1] += 1; }
                self.compress(false);
                self.buflen = 0;
            }
            let step = (128 - self.buflen).min(data.len() - i);
            self.buf[self.buflen..self.buflen + step].copy_from_slice(&data[i..i + step]);
            self.buflen += step;
            i += step;
        }
    }

    fn finalize(mut self) -> [u8; 64] {
        self.t[0] = self.t[0].wrapping_add(self.buflen as u64);
        if self.t[0] < (self.buflen as u64) { self.t[1] += 1; }
        while self.buflen < 128 {
            self.buf[self.buflen] = 0;
            self.buflen += 1;
        }
        self.compress(true);

        let mut out = [0u8; 64];
        for (i, &val) in self.h.iter().enumerate() {
            out[i * 8..i * 8 + 8].copy_from_slice(&val.to_le_bytes());
        }
        out
    }
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else { Box::new(File::open(path)?) };
    let mut hasher = Blake2b::new();
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
                println!("Usage: b2sum [OPTION]... [FILE]...\nPrint or check BLAKE2 (512-bit) checksums.\n\n  -b, --binary   read in binary mode\n  -c, --check    read BLAKE2 sums from the FILEs and check them\n  -t, --text     read in text mode (default)\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("b2sum: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    if check_mode {
        let mut fail_count = 0;
        for file in files {
            let reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
                match File::open(&file) { Ok(f) => Box::new(f), Err(e) => { eprintln!("b2sum: {}: {}", file, e); continue; } }
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
        if fail_count > 0 { eprintln!("b2sum: WARNING: {} computed checksum did NOT match", fail_count); process::exit(1); }
    } else {
        let mode_char = if binary_mode { '*' } else { ' ' };
        let mut exit_code = 0;
        for file in files {
            match hash_file(&file) {
                Ok(hash) => println!("{} {}{}", hash, mode_char, file),
                Err(e) => { eprintln!("b2sum: {}: {}", file, e); exit_code = 1; }
            }
        }
        process::exit(exit_code);
    }
}
