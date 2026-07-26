use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

struct Sha512 { state: [u64; 8], count: u128, buffer: [u8; 128] }
impl Sha512 {
    fn new() -> Self {
        Self {
            state: [
                0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
            ],
            count: 0,
            buffer: [0u8; 128],
        }
    }
    fn transform(&mut self, data: &[u8]) {
        const K: [u64; 80] = [
            0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
            0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
            0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
            0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
            0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
            0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
            0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
            0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec, 0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
            0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
            0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
        ];
        let mut w = [0u64; 80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes([data[i * 8], data[i * 8 + 1], data[i * 8 + 2], data[i * 8 + 3], data[i * 8 + 4], data[i * 8 + 5], data[i * 8 + 6], data[i * 8 + 7]]);
        }
        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut a = self.state[0]; let mut b = self.state[1]; let mut c = self.state[2]; let mut d = self.state[3];
        let mut e = self.state[4]; let mut f = self.state[5]; let mut g = self.state[6]; let mut h = self.state[7];
        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g; g = f; f = e; e = d.wrapping_add(temp1); d = c; c = b; b = a; a = temp1.wrapping_add(temp2);
        }
        self.state[0] = self.state[0].wrapping_add(a); self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c); self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e); self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g); self.state[7] = self.state[7].wrapping_add(h);
    }
    fn update(&mut self, data: &[u8]) {
        let mut idx = (self.count as usize) & 127;
        self.count += data.len() as u128;
        let mut i = 0;
        while i < data.len() {
            let step = (128 - idx).min(data.len() - i);
            self.buffer[idx..idx + step].copy_from_slice(&data[i..i + step]);
            idx += step; i += step;
            if idx == 128 {
                let buf = self.buffer;
                self.transform(&buf);
                idx = 0;
            }
        }
    }
    fn finalize(mut self) -> [u8; 64] {
        let bits = (self.count * 8).to_be_bytes();
        let idx = (self.count as usize) & 127;
        let pad_len = if idx < 112 { 112 - idx } else { 240 - idx };
        let mut pad = [0u8; 128]; pad[0] = 0x80;
        self.update(&pad[..pad_len]);
        self.update(&bits);
        let mut out = [0u8; 64];
        for (i, &word) in self.state.iter().enumerate() {
            out[i * 8..i * 8 + 8].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else { Box::new(File::open(path)?) };
    let mut hasher = Sha512::new();
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
                println!("Usage: sha512sum [OPTION]... [FILE]...\nPrint or check SHA512 (512-bit) checksums.\n\n  -b, --binary   read in binary mode\n  -c, --check    read SHA512 sums from the FILEs and check them\n  -t, --text     read in text mode (default)\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("sha512sum: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.is_empty() { files.push("-".to_string()); }

    if check_mode {
        let mut fail_count = 0;
        for file in files {
            let reader: Box<dyn Read> = if file == "-" { Box::new(io::stdin()) } else {
                match File::open(&file) { Ok(f) => Box::new(f), Err(e) => { eprintln!("sha512sum: {}: {}", file, e); continue; } }
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
        if fail_count > 0 { eprintln!("sha512sum: WARNING: {} computed checksum did NOT match", fail_count); process::exit(1); }
    } else {
        let mode_char = if binary_mode { '*' } else { ' ' };
        let mut exit_code = 0;
        for file in files {
            match hash_file(&file) {
                Ok(hash) => println!("{} {}{}", hash, mode_char, file),
                Err(e) => { eprintln!("sha512sum: {}: {}", file, e); exit_code = 1; }
            }
        }
        process::exit(exit_code);
    }
}
