use std::env;
use std::io::{self, Read, Write};
use std::process;

fn expand_set(s: &str) -> Vec<char> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '-' {
            let (start, end) = (chars[i] as u32, chars[i + 2] as u32);
            if start <= end {
                for c in start..=end {
                    if let Some(ch) = char::from_u32(c) { result.push(ch); }
                }
                i += 3;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut delete = false;
    let mut squeeze = false;
    let mut sets = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-d" | "--delete" => delete = true,
            "-s" | "--squeeze-repeats" => squeeze = true,
            "-ds" | "-sd" => { delete = true; squeeze = true; }
            "--help" => {
                println!("Usage: tr [OPTION]... SET1 [SET2]\nTranslate, squeeze, and/or delete characters from standard input.\n\n  -d, --delete            delete characters in SET1, do not translate\n  -s, --squeeze-repeats   replace each sequence of a repeated character\n                          that is listed in the last specified SET\n      --help              display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("tr: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => sets.push(arg.clone()),
        }
    }

    if sets.is_empty() {
        eprintln!("tr: missing operand\nTry 'tr --help' for more information.");
        process::exit(1);
    }

    let set1 = expand_set(&sets[0]);
    let set2 = if sets.len() > 1 { expand_set(&sets[1]) } else { Vec::new() };

    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("tr: error reading stdin: {}", e);
        process::exit(1);
    }

    let mut output = String::with_capacity(input.len());
    let mut last_char: Option<char> = None;

    for ch in input.chars() {
        if delete && set1.contains(&ch) {
            continue;
        }

        let mut mapped = ch;
        if !delete && !set2.is_empty() {
            if let Some(idx) = set1.iter().position(|&c| c == ch) {
                mapped = if idx < set2.len() { set2[idx] } else { *set2.last().unwrap() };
            }
        }

        let squeeze_set = if delete { &set2 } else if !set2.is_empty() { &set2 } else { &set1 };
        if squeeze && squeeze_set.contains(&mapped) && Some(mapped) == last_char {
            continue;
        }

        output.push(mapped);
        last_char = Some(mapped);
    }

    let _ = io::stdout().write_all(output.as_bytes());
}
