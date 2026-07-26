use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

struct JoinLine {
    key: String,
    remainder: String,
}

fn parse_line(line: &str, field_idx: usize, delim: Option<char>) -> JoinLine {
    let parts: Vec<&str> = match delim {
        Some(d) => line.split(d).collect(),
        None => line.split_whitespace().collect(),
    };
    let key = parts.get(field_idx).copied().unwrap_or("").to_string();
    let mut rem_parts = parts.clone();
    if field_idx < rem_parts.len() { rem_parts.remove(field_idx); }
    let remainder = match delim {
        Some(d) => rem_parts.join(&d.to_string()),
        None => rem_parts.join(" "),
    };
    JoinLine { key, remainder }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f1_field = 0;
    let mut f2_field = 0;
    let mut delimiter = None;
    let mut print_unpairable_1 = false;
    let mut print_unpairable_2 = false;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-1" => { f1_field = args[i + 1].parse::<usize>().unwrap_or(1).saturating_sub(1); i += 1; }
            "-2" => { f2_field = args[i + 1].parse::<usize>().unwrap_or(1).saturating_sub(1); i += 1; }
            "-t" => { delimiter = args[i + 1].chars().next(); i += 1; }
            "-a" => {
                if args[i + 1] == "1" { print_unpairable_1 = true; }
                else if args[i + 1] == "2" { print_unpairable_2 = true; }
                i += 1;
            }
            "--help" => {
                println!("Usage: join [OPTION]... FILE1 FILE2\nFor each pair of input lines with identical join fields, write a line to\nstandard output.\n\n  -1 FIELD          join on this FIELD of file 1\n  -2 FIELD          join on this FIELD of file 2\n  -a FILENUM        also print unpairable lines from file FILENUM, where\n                    FILENUM is 1 or 2\n  -t CHAR           use CHAR as input and output field separator\n      --help        display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("join: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.len() != 2 { eprintln!("join: missing operand or too many arguments\nTry 'join --help' for more information."); process::exit(1); }

    let read_file = |path: &str| -> Vec<String> {
        let reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else {
            File::open(path).map(|f| Box::new(f) as Box<dyn Read>).unwrap_or_else(|e| { eprintln!("join: {}: {}", path, e); process::exit(1); })
        };
        BufReader::new(reader).lines().map_while(Result::ok).collect()
    };

    let lines1 = read_file(&files[0]);
    let lines2 = read_file(&files[1]);

    let parsed1: Vec<JoinLine> = lines1.iter().map(|l| parse_line(l, f1_field, delimiter)).collect();
    let parsed2: Vec<JoinLine> = lines2.iter().map(|l| parse_line(l, f2_field, delimiter)).collect();

    let sep = delimiter.unwrap_or(' ');
    let mut i = 0;
    let mut j = 0;

    while i < parsed1.len() && j < parsed2.len() {
        match parsed1[i].key.cmp(&parsed2[j].key) {
            Ordering::Equal => {
                let r1 = &parsed1[i].remainder;
                let r2 = &parsed2[j].remainder;
                if r1.is_empty() && r2.is_empty() { println!("{}", parsed1[i].key); }
                else if r1.is_empty() { println!("{}{}{}", parsed1[i].key, sep, r2); }
                else if r2.is_empty() { println!("{}{}{}", parsed1[i].key, sep, r1); }
                else { println!("{}{}{}{}{}", parsed1[i].key, sep, r1, sep, r2); }
                i += 1;
                j += 1;
            }
            Ordering::Less => {
                if print_unpairable_1 {
                    let r1 = &parsed1[i].remainder;
                    if r1.is_empty() { println!("{}", parsed1[i].key); } else { println!("{}{}{}", parsed1[i].key, sep, r1); }
                }
                i += 1;
            }
            Ordering::Greater => {
                if print_unpairable_2 {
                    let r2 = &parsed2[j].remainder;
                    if r2.is_empty() { println!("{}", parsed2[j].key); } else { println!("{}{}{}", parsed2[j].key, sep, r2); }
                }
                j += 1;
            }
        }
    }

    while i < parsed1.len() {
        if print_unpairable_1 {
            let r1 = &parsed1[i].remainder;
            if r1.is_empty() { println!("{}", parsed1[i].key); } else { println!("{}{}{}", parsed1[i].key, sep, r1); }
        }
        i += 1;
    }
    while j < parsed2.len() {
        if print_unpairable_2 {
            let r2 = &parsed2[j].remainder;
            if r2.is_empty() { println!("{}", parsed2[j].key); } else { println!("{}{}{}", parsed2[j].key, sep, r2); }
        }
        j += 1;
    }
}
