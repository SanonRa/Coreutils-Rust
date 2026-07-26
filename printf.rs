use std::env;
use std::io::{self, Write};
use std::process;

fn parse_escapes(s: &str) -> String {
    let mut res = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => res.push('\n'),
                Some('t') => res.push('\t'),
                Some('r') => res.push('\r'),
                Some('0') => res.push('\0'),
                Some('\\') => res.push('\\'),
                Some('\"') => res.push('\"'),
                Some('\'') => res.push('\''),
                Some(other) => { res.push('\\'); res.push(other); }
                None => res.push('\\'),
            }
        } else { res.push(c); }
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" {
        println!("Usage: printf FORMAT [ARGUMENT]...\nFormat and print ARGUMENT(s) according to FORMAT.\n\n      --help     display this help and exit");
        return;
    }

    let format = parse_escapes(&args[1]);
    let mut arg_idx = 2;
    let mut out = io::stdout().lock();

    loop {
        let mut chars = format.chars().peekable();
        let mut printed_any = false;

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.next() {
                    Some('%') => { let _ = write!(out, "%"); printed_any = true; }
                    Some('s') => {
                        let val = args.get(arg_idx).map(|s| s.as_str()).unwrap_or("");
                        let _ = write!(out, "{}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some('d') | Some('i') => {
                        let val = args.get(arg_idx).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
                        let _ = write!(out, "{}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some('u') => {
                        let val = args.get(arg_idx).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let _ = write!(out, "{}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some('x') => {
                        let val = args.get(arg_idx).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let _ = write!(out, "{:x}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some('X') => {
                        let val = args.get(arg_idx).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let _ = write!(out, "{:X}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some('o') => {
                        let val = args.get(arg_idx).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let _ = write!(out, "{:o}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some('c') => {
                        let val = args.get(arg_idx).and_then(|s| s.chars().next()).unwrap_or('\0');
                        let _ = write!(out, "{}", val);
                        arg_idx += 1;
                        printed_any = true;
                    }
                    Some(other) => { let _ = write!(out, "%{}", other); printed_any = true; }
                    None => { let _ = write!(out, "%"); printed_any = true; }
                }
            } else {
                let _ = write!(out, "{}", c);
                printed_any = true;
            }
        }
        if arg_idx >= args.len() || !printed_any { break; }
    }
}
