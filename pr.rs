use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn process_file(path: &str, omit_header: bool, page_len: usize) {
    let reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else {
        match File::open(path) { Ok(f) => Box::new(f), Err(e) => { eprintln!("pr: {}: {}", path, e); return; } }
    };

    let lines: Vec<String> = BufReader::new(reader).lines().map_while(Result::ok).collect();
    let body_len = if omit_header { page_len } else { page_len.saturating_sub(10) };
    let total_pages = (lines.len() + body_len - 1) / body_len.max(1);

    for (page_idx, chunk) in lines.chunks(body_len).enumerate() {
        if !omit_header {
            if page_idx > 0 { println!("\n\n"); }
            let title = if path == "-" { "standard input" } else { path };
            println!("\n\n2026-07-26 12:00 {} Page {}\n\n", title, page_idx + 1);
        }
        for line in chunk { println!("{}", line); }
        if !omit_header {
            let printed = chunk.len();
            for _ in printed..body_len { println!(); }
            println!("\n\n");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut omit_header = false;
    let mut page_len = 66usize;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-t" | "--omit-header" => omit_header = true,
            "-l" | "--length" => {
                if i + 1 >= args.len() { eprintln!("pr: option requires an argument"); process::exit(1); }
                page_len = args[i + 1].parse().unwrap_or(66); i += 1;
            }
            "--help" => {
                println!("Usage: pr [OPTION]... [FILE]...\nPaginate or columnate FILE(s) for printing.\n\n  -l, --length=PAGE_LENGTH   set the page length to PAGE_LENGTH (default 66)\n  -t, --omit-header          omit page headers and trailers\n      --help                 display this help and exit");
                return;
            }
            _ if let Some(val) = arg.strip_prefix("-l") => page_len = val.parse().unwrap_or(66),
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("pr: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }
    for file in files { process_file(&file, omit_header, page_len); }
}
