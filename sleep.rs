// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::process;

const VERSION: &str = "9.11";

fn print_version(program_name: &str, authors: &[&str]) {
    println!("{} (GNU coreutils) {}", program_name, VERSION);
    println!("Copyright (C) 2026 Free Software Foundation, Inc.");
    println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.");
    println!("This is free software: you are free to change and redistribute.");
    println!("There is NO WARRANTY, to the extent permitted by law.\n");
    if !authors.is_empty() {
        if authors.len() == 1 {
            println!("Written by {}.", authors[0]);
        } else {
            print!("Written by ");
            for (i, author) in authors.iter().enumerate() {
                if i > 0 {
                    if i == authors.len() - 1 {
                        print!(" and ");
                    } else {
                        print!(", ");
                    }
                }
                print!("{}", author);
            }
            println!(".");
        }
    }
}

fn print_help_epilogue(program_name: &str) {
    println!("\nGNU coreutils online help: <https://www.gnu.org/software/coreutils/>");
    println!("Full documentation <https://www.gnu.org/software/coreutils/{}>", program_name);
    println!("or available locally via: info '(coreutils) {} invocation'", program_name);
}

fn print_help() {
    println!("Usage: sleep NUMBER[SUFFIX]...");
    println!("  or:  sleep OPTION");
    println!("Pause for NUMBER seconds, where NUMBER is an integer or floating-point.");
    println!("SUFFIX may be 's','m','h', or 'd', for seconds, minutes, hours, days.");
    println!("With multiple arguments, pause for the sum of their values.\n");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    print_help_epilogue("sleep");
}

fn parse_duration(arg: &str) -> Result<f64, String> {
    if arg.is_empty() {
        return Err(format!("invalid time interval '{}'", arg));
    }
    
    // 1. Try parsing the entire string as a float (e.g. "1.5", "inf")
    if let Ok(val) = arg.parse::<f64>() {
        if val < 0.0 || val.is_nan() {
            return Err(format!("invalid time interval '{}'", arg));
        }
        return Ok(val);
    }
    
    // 2. If it fails, check if the last character is a valid suffix
    let last_char = arg.chars().last().unwrap();
    let multiplier = match last_char {
        's' => 1.0,
        'm' => 60.0,
        'h' => 3600.0,
        'd' => 86400.0,
        _ => return Err(format!("invalid time interval '{}'", arg)),
    };
    
    // Strip suffix and parse the rest
    let num_part = &arg[..arg.len() - last_char.len_utf8()];
    if let Ok(val) = num_part.parse::<f64>() {
        if val < 0.0 || val.is_nan() {
            return Err(format!("invalid time interval '{}'", arg));
        }
        return Ok(val * multiplier);
    }
    
    Err(format!("invalid time interval '{}'", arg))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check standard options
    if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
            process::exit(0);
        } else if args[1] == "--version" {
            print_version("sleep", &["Jim Meyering", "Paul Eggert"]);
            process::exit(0);
        }
    }
    
    if args.len() == 1 {
        eprintln!("sleep: missing operand");
        eprintln!("Try 'sleep --help' for more information.");
        process::exit(1);
    }
    
    let mut total_seconds = 0.0;
    let mut ok = true;
    
    for arg in &args[1..] {
        match parse_duration(arg) {
            Ok(secs) => {
                total_seconds += secs;
            }
            Err(err_msg) => {
                eprintln!("sleep: {}", err_msg);
                ok = false;
            }
        }
    }
    
    if !ok {
        eprintln!("Try 'sleep --help' for more information.");
        process::exit(1);
    }
    
    if total_seconds.is_infinite() {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(86400));
        }
    } else {
        std::thread::sleep(std::time::Duration::from_secs_f64(total_seconds));
    }
    
    process::exit(0);
}
