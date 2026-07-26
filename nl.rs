use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut body_style = 't';
    let mut separator = "\t".to_string();
    let mut width = 6;
    let mut incr = 1;
    let mut line_num = 1;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-b" => {
                if i + 1 >= args.len() {
                    eprintln!("nl: option requires an argument -- '-b'");
                    process::exit(1);
                }
                body_style = args[i + 1].chars().next().unwrap_or('t');
                i += 1;
            }
            "-s" => {
                if i + 1 >= args.len() {
                    eprintln!("nl: option requires an argument -- '-s'");
                    process::exit(1);
                }
                separator = args[i + 1].clone();
                i += 1;
            }
            "-w" => {
                if i + 1 >= args.len() {
                    eprintln!("nl: option requires an argument -- '-w'");
                    process::exit(1);
                }
                width = args[i + 1].parse().unwrap_or(6);
                i += 1;
            }
            "-i" => {
                if i + 1 >= args.len() {
                    eprintln!("nl: option requires an argument -- '-i'");
                    process::exit(1);
                }
                incr = args[i + 1].parse().unwrap_or(1);
                i += 1;
            }
            "-v" => {
                if i + 1 >= args.len() {
                    eprintln!("nl: option requires an argument -- '-v'");
                    process::exit(1);
                }
                line_num = args[i + 1].parse().unwrap_or(1);
                i += 1;
            }
            "--help" => {
                println!("Usage: nl [OPTION]... [FILE]...\nWrite each FILE to standard output, with line numbers added.\n\n  -b, --body-numbering=STYLE   use STYLE for body line numbers (a=all, t=non-empty)\n  -i, --line-increment=NUMBER  line number increment at each line\n  -s, --number-separator=STRING add STRING after line number\n  -v, --starting-line-number=NUMBER first line number\n  -w, --number-width=NUMBER    use NUMBER columns for line numbers\n      --help                   display this help and exit");
                return;
            }
            _ if arg.starts_with("-b") => body_style = arg.strip_prefix("-b").unwrap().chars().next().unwrap_or('t'),
            _ if arg.starts_with("-s") => separator = arg.strip_prefix("-s").unwrap().to_string(),
            _ if arg.starts_with("-w") => width = arg.strip_prefix("-w").unwrap().parse().unwrap_or(6),
            _ if arg.starts_with("-i") => incr = arg.strip_prefix("-i").unwrap().parse().unwrap_or(1),
            _ if arg.starts_with("-v") => line_num = arg.strip_prefix("-v").unwrap().parse().unwrap_or(1),
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("nl: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    for file in &files {
        let reader: Box<dyn Read> = if file == "-" {
            Box::new(io::stdin())
        } else {
            match File::open(file) {
                Ok(f) => Box::new(f),
                Err(e) => { eprintln!("nl: {}: {}", file, e); continue; }
            }
        };

        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            let number_this = match body_style {
                'a' => true,
                't' => !line.trim().is_empty(),
                _ => false,
            };

            if number_this {
                println!("{:width$}{}{}", line_num, separator, line, width = width);
                line_num += incr;
            } else {
                println!("{:width$} {}", "", line, width = width);
            }
        }
    }
}
