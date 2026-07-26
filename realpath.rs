use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: realpath [OPTION]... FILE...\nPrint the resolved absolute file name.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("realpath: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
    }

    if files.is_empty() {
        eprintln!("realpath: missing operand\nTry 'realpath --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for file in files {
        match fs::canonicalize(&file) {
            Ok(path) => {
                let display = path.display().to_string();
                if let Some(stripped) = display.strip_prefix(r"\\?\") {
                    println!("{}", stripped);
                } else {
                    println!("{}", display);
                }
            }
            Err(e) => {
                eprintln!("realpath: {}: {}", file, e);
                exit_code = 1;
            }
        }
    }
    process::exit(exit_code);
}
