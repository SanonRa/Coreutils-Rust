use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

struct Prng { state: u64 }
impl Prng {
    fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(12345) ^ (process::id() as u64);
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
    fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut echo = false;
    let mut input_range: Option<(i64, i64)> = None;
    let mut head_count: Option<usize> = None;
    let mut output_file: Option<String> = None;
    let mut operands = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-e" || arg == "--echo" {
            echo = true;
        } else if arg == "-i" || arg == "--input-range" {
            if i + 1 >= args.len() { eprintln!("shuf: option requires an argument"); process::exit(1); }
            let spec = &args[i + 1];
            if let Some(idx) = spec.find('-') {
                let lo = spec[..idx].parse().unwrap_or(1);
                let hi = spec[idx + 1..].parse().unwrap_or(0);
                input_range = Some((lo, hi));
            } else {
                eprintln!("shuf: invalid input range '{}'", spec); process::exit(1);
            }
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-i") {
            if let Some(idx) = val.find('-') {
                input_range = Some((val[..idx].parse().unwrap_or(1), val[idx + 1..].parse().unwrap_or(0)));
            }
        } else if arg == "-n" || arg == "--head-count" {
            if i + 1 >= args.len() { eprintln!("shuf: option requires an argument"); process::exit(1); }
            head_count = args[i + 1].parse().ok();
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-n") {
            head_count = val.parse().ok();
        } else if arg == "-o" || arg == "--output" {
            if i + 1 >= args.len() { eprintln!("shuf: option requires an argument"); process::exit(1); }
            output_file = Some(args[i + 1].clone());
            i += 1;
        } else if arg == "--help" {
            println!("Usage: shuf [OPTION]... [FILE]\n  or:  shuf -e [OPTION]... [ARG]...\n  or:  shuf -i LO-HI [OPTION]...\nWrite a random permutation of the input lines to standard output.\n\n  -e, --echo                treat each ARG as an input line\n  -i, --input-range=LO-HI   treat each number LO through HI as an input line\n  -n, --head-count=COUNT    output at most COUNT lines\n  -o, --output=FILE         write result to FILE instead of standard output\n      --help                display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" && !echo {
            eprintln!("shuf: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            operands.push(arg.clone());
        }
        i += 1;
    }

    let mut lines = Vec::new();
    if echo {
        lines = operands;
    } else if let Some((lo, hi)) = input_range {
        if lo <= hi {
            for n in lo..=hi { lines.push(n.to_string()); }
        }
    } else {
        let file_arg = operands.first().map(|s| s.as_str()).unwrap_or("-");
        let reader: Box<dyn io::Read> = if file_arg == "-" {
            Box::new(io::stdin())
        } else match File::open(file_arg) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("shuf: {}: {}", file_arg, e); process::exit(1); }
        };
        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            lines.push(line);
        }
    }

    let mut prng = Prng::new();
    prng.shuffle(&mut lines);

    let limit = head_count.unwrap_or(lines.len()).min(lines.len());
    let mut writer: Box<dyn Write> = match output_file {
        Some(path) => match File::create(&path) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("shuf: {}: {}", path, e); process::exit(1); }
        },
        None => Box::new(io::stdout()),
    };

    for line in &lines[..limit] {
        let _ = writeln!(writer, "{}", line);
    }
}
