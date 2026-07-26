use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn get_reader(path: &str) -> io::Result<BufReader<Box<dyn Read>>> {
    if path == "-" {
        Ok(BufReader::new(Box::new(io::stdin())))
    } else {
        File::open(path).map(|f| BufReader::new(Box::new(f) as Box<dyn Read>))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut serial = false;
    let mut delimiters = vec!['\t'];
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-s" | "--serial" => serial = true,
            "-d" | "--delimiters" => {
                if i + 1 >= args.len() {
                    eprintln!("paste: option requires an argument -- '{}'", arg);
                    process::exit(1);
                }
                delimiters = args[i + 1].chars().collect();
                if delimiters.is_empty() { delimiters.push('\t'); }
                i += 1;
            }
            "--help" => {
                println!("Usage: paste [OPTION]... [FILE]...\nWrite lines consisting of the sequentially corresponding lines from\neach FILE, separated by TABs, to standard output.\n\n  -d, --delimiters=LIST   reuse characters from LIST instead of TABs\n  -s, --serial            paste one file at a time instead of in parallel\n      --help              display this help and exit");
                return;
            }
            _ if arg.starts_with("-d") => {
                let val = arg.strip_prefix("-d").unwrap();
                delimiters = val.chars().collect();
                if delimiters.is_empty() { delimiters.push('\t'); }
            }
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("paste: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() { files.push("-".to_string()); }

    if serial {
        for file in &files {
            match get_reader(file) {
                Ok(reader) => {
                    let mut first = true;
                    let mut d_idx = 0;
                    for line in reader.lines().map_while(Result::ok) {
                        if !first {
                            print!("{}", delimiters[d_idx % delimiters.len()]);
                            d_idx += 1;
                        }
                        print!("{}", line);
                        first = false;
                    }
                    println!();
                }
                Err(e) => eprintln!("paste: {}: {}", file, e),
            }
        }
    } else {
        let mut readers: Vec<Option<BufReader<Box<dyn Read>>>> = files
            .iter()
            .map(|f| match get_reader(f) {
                Ok(r) => Some(r),
                Err(e) => { eprintln!("paste: {}: {}", f, e); None }
            })
            .collect();

        loop {
            let mut any_read = false;
            let mut line_buffers = Vec::new();

            for reader_opt in &mut readers {
                let mut line = String::new();
                if let Some(reader) = reader_opt {
                    match reader.read_line(&mut line) {
                        Ok(n) if n > 0 => {
                            if line.ends_with('\n') { line.pop(); }
                            if line.ends_with('\r') { line.pop(); }
                            any_read = true;
                            line_buffers.push(line);
                        }
                        _ => line_buffers.push(String::new()),
                    }
                } else {
                    line_buffers.push(String::new());
                }
            }

            if !any_read { break; }

            for (idx, line) in line_buffers.iter().enumerate() {
                if idx > 0 {
                    print!("{}", delimiters[(idx - 1) % delimiters.len()]);
                }
                print!("{}", line);
            }
            println!();
        }
    }
}
