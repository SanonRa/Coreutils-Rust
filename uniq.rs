use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

struct UniqOpts {
    count: bool,
    repeated: bool,
    unique: bool,
    ignore_case: bool,
    skip_fields: usize,
    skip_chars: usize,
}

fn compare_key<'a>(line: &'a str, opts: &UniqOpts) -> String {
    let mut s = line;
    if opts.skip_fields > 0 {
        let mut fields = s.split_whitespace();
        for _ in 0..opts.skip_fields {
            if let Some(f) = fields.next() {
                let idx = f.as_ptr() as usize - s.as_ptr() as usize + f.len();
                if idx < s.len() { s = &s[idx..]; } else { s = ""; break; }
            } else { s = ""; break; }
        }
        s = s.trim_start();
    }
    if opts.skip_chars > 0 {
        let char_count = s.chars().count();
        if opts.skip_chars >= char_count { s = ""; }
        else {
            let idx = s.char_indices().nth(opts.skip_chars).map(|(i, _)| i).unwrap_or(s.len());
            s = &s[idx..];
        }
    }
    if opts.ignore_case { s.to_lowercase() } else { s.to_string() }
}

fn emit(line: &str, count: usize, opts: &UniqOpts) {
    if count == 0 { return; }
    if opts.repeated && count < 2 { return; }
    if opts.unique && count > 1 { return; }
    if opts.count { println!("{:7} {}", count, line); }
    else { println!("{}", line); }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = UniqOpts { count: false, repeated: false, unique: false, ignore_case: false, skip_fields: 0, skip_chars: 0 };
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-c" | "--count" => opts.count = true,
            "-d" | "--repeated" => opts.repeated = true,
            "-u" | "--unique" => opts.unique = true,
            "-i" | "--ignore-case" => opts.ignore_case = true,
            "-f" | "--skip-fields" => {
                if i + 1 >= args.len() { eprintln!("uniq: option requires an argument"); process::exit(1); }
                opts.skip_fields = args[i + 1].parse().unwrap_or(0); i += 1;
            }
            "-s" | "--skip-chars" => {
                if i + 1 >= args.len() { eprintln!("uniq: option requires an argument"); process::exit(1); }
                opts.skip_chars = args[i + 1].parse().unwrap_or(0); i += 1;
            }
            "--help" => {
                println!("Usage: uniq [OPTION]... [INPUT [OUTPUT]]\nFilter adjacent matching lines from INPUT (or standard input),\nwriting to OUTPUT (or standard output).\n\n  -c, --count           prefix lines by the number of occurrences\n  -d, --repeated        only print duplicate lines, one for each group\n  -f, --skip-fields=N   avoid comparing the first N fields\n  -i, --ignore-case     ignore differences in case when comparing\n  -s, --skip-chars=N    avoid comparing the first N characters\n  -u, --unique          only print unique lines\n      --help            display this help and exit");
                return;
            }
            _ if let Some(val) = arg.strip_prefix("-f") => opts.skip_fields = val.parse().unwrap_or(0),
            _ if let Some(val) = arg.strip_prefix("-s") => opts.skip_chars = val.parse().unwrap_or(0),
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("uniq: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    let input_path = files.first().map(|s| s.as_str()).unwrap_or("-");
    let reader: Box<dyn Read> = if input_path == "-" { Box::new(io::stdin()) } else {
        match File::open(input_path) {
            Ok(f) => Box::new(f),
            Err(e) => { eprintln!("uniq: {}: {}", input_path, e); process::exit(1); }
        }
    };

    let mut prev_line = String::new();
    let mut prev_key = String::new();
    let mut count = 0;

    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        let key = compare_key(&line, &opts);
        if count == 0 {
            prev_line = line;
            prev_key = key;
            count = 1;
        } else if key == prev_key {
            count += 1;
        } else {
            emit(&prev_line, count, &opts);
            prev_line = line;
            prev_key = key;
            count = 1;
        }
    }
    emit(&prev_line, count, &opts);
}
