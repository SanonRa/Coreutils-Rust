// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
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
    println!("Usage: cat [OPTION]... [FILE]...");
    println!("Concatenate FILE(s) to standard output.\n");
    println!("With no FILE, or when FILE is -, read standard input.\n");
    println!("  -A, --show-all           equivalent to -vET");
    println!("  -b, --number-nonblank    number nonempty output lines, overrides -n");
    println!("  -e                       equivalent to -vE");
    println!("  -E, --show-ends          display $ at end of each line");
    println!("  -n, --number             number all output lines");
    println!("  -s, --squeeze-blank      suppress repeated empty output lines");
    println!("  -t                       equivalent to -vT");
    println!("  -T, --show-tabs          display TAB characters as ^I");
    println!("  -u                       (ignored)");
    println!("  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
    println!("\nExamples:");
    println!("  cat f - g  Output f's contents, then standard input, then g's contents.");
    println!("  cat        Copy standard input to standard output.");
    print_help_epilogue("cat");
}

fn copy_fast<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut buf = [0u8; 65536];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n])?;
    }
    Ok(())
}

struct CatOptions {
    show_nonprinting: bool,
    show_tabs: bool,
    number: bool,
    number_nonblank: bool,
    show_ends: bool,
    squeeze_blank: bool,
}

fn cat_process<R: Read, W: Write>(
    reader: R,
    writer: &mut W,
    opts: &CatOptions,
    line_number: &mut u64,
    at_line_start: &mut bool,
    consecutive_newlines: &mut u32,
) -> io::Result<()> {
    let mut buf_reader = BufReader::new(reader);
    let mut buf = [0u8; 65536];
    
    loop {
        let n = buf_reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        
        let mut idx = 0;
        while idx < n {
            let ch = buf[idx];
            idx += 1;
            
            if ch == b'\n' {
                *consecutive_newlines += 1;
                if opts.squeeze_blank && *consecutive_newlines > 2 {
                    continue;
                }
                
                if *at_line_start && opts.number && !opts.number_nonblank {
                    write!(writer, "{:>6}\t", line_number)?;
                    *line_number += 1;
                }
                
                if opts.show_ends {
                    writer.write_all(b"$")?;
                }
                writer.write_all(b"\n")?;
                *at_line_start = true;
            } else {
                if *at_line_start {
                    if opts.number || opts.number_nonblank {
                        write!(writer, "{:>6}\t", line_number)?;
                        *line_number += 1;
                    }
                    *at_line_start = false;
                }
                *consecutive_newlines = 0;
                
                if ch == b'\t' && opts.show_tabs {
                    writer.write_all(b"^I")?;
                } else if ch == b'\r' && opts.show_ends && idx < n && buf[idx] == b'\n' {
                    writer.write_all(b"^M")?;
                } else if opts.show_nonprinting {
                    if ch < 32 {
                        if ch == b'\t' {
                            writer.write_all(b"\t")?;
                        } else {
                            writer.write_all(&[b'^', ch + 64])?;
                        }
                    } else if ch == 127 {
                        writer.write_all(b"^?")?;
                    } else if ch > 127 {
                        writer.write_all(b"M-")?;
                        let val = ch - 128;
                        if val < 32 {
                            if val == b'\t' {
                                writer.write_all(b"\t")?;
                            } else if val == b'\n' {
                                writer.write_all(b"\n")?;
                            } else {
                                writer.write_all(&[b'^', val + 64])?;
                            }
                        } else if val == 127 {
                            writer.write_all(b"^?")?;
                        } else {
                            writer.write_all(&[val])?;
                        }
                    } else {
                        writer.write_all(&[ch])?;
                    }
                } else {
                    writer.write_all(&[ch])?;
                }
            }
        }
    }
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut show_nonprinting = false;
    let mut show_tabs = false;
    let mut number = false;
    let mut number_nonblank = false;
    let mut show_ends = false;
    let mut squeeze_blank = false;
    
    let mut files = Vec::new();
    let mut print_h = false;
    let mut print_v = false;
    
    // Custom option parsing
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            print_h = true;
            break;
        } else if arg == "--version" {
            print_v = true;
            break;
        } else if arg.starts_with("--") {
            match arg.as_str() {
                "--show-all" => {
                    show_nonprinting = true;
                    show_ends = true;
                    show_tabs = true;
                }
                "--number-nonblank" => number_nonblank = true,
                "--show-ends" => show_ends = true,
                "--number" => number = true,
                "--squeeze-blank" => squeeze_blank = true,
                "--show-tabs" => show_tabs = true,
                "--show-nonprinting" => show_nonprinting = true,
                _ => {
                    eprintln!("cat: unrecognized option '{}'", arg);
                    eprintln!("Try 'cat --help' for more information.");
                    process::exit(1);
                }
            }
        } else if arg.starts_with('-') && arg != "-" {
            let mut invalid = None;
            for c in arg.chars().skip(1) {
                match c {
                    'A' => {
                        show_nonprinting = true;
                        show_ends = true;
                        show_tabs = true;
                    }
                    'b' => number_nonblank = true,
                    'e' => {
                        show_nonprinting = true;
                        show_ends = true;
                    }
                    'E' => show_ends = true,
                    'n' => number = true,
                    's' => squeeze_blank = true,
                    't' => {
                        show_nonprinting = true;
                        show_tabs = true;
                    }
                    'T' => show_tabs = true,
                    'u' => {} // Ignored
                    'v' => show_nonprinting = true,
                    _ => {
                        invalid = Some(c);
                        break;
                    }
                }
            }
            if let Some(c) = invalid {
                eprintln!("cat: invalid option -- '{}'", c);
                eprintln!("Try 'cat --help' for more information.");
                process::exit(1);
            }
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }
    
    if print_h {
        print_help();
        process::exit(0);
    }
    if print_v {
        print_version("cat", &["Torbjorn Granlund", "Richard M. Stallman"]);
        process::exit(0);
    }
    
    if files.is_empty() {
        files.push("-".to_string());
    }
    
    let any_options = show_nonprinting || show_tabs || number || number_nonblank || show_ends || squeeze_blank;
    let opts = CatOptions {
        show_nonprinting,
        show_tabs,
        number,
        number_nonblank,
        show_ends,
        squeeze_blank,
    };
    
    let stdout = io::stdout();
    let mut out_writer = BufWriter::new(stdout.lock());
    
    let mut line_number = 1;
    let mut at_line_start = true;
    let mut consecutive_newlines = 0;
    
    let mut exit_code = 0;
    
    for file in files {
        if file == "-" {
            let stdin = io::stdin();
            let stdin_lock = stdin.lock();
            if any_options {
                if let Err(e) = cat_process(stdin_lock, &mut out_writer, &opts, &mut line_number, &mut at_line_start, &mut consecutive_newlines) {
                    eprintln!("cat: <stdin>: {}", e);
                    exit_code = 1;
                }
            } else {
                if let Err(e) = copy_fast(stdin_lock, &mut out_writer) {
                    eprintln!("cat: <stdin>: {}", e);
                    exit_code = 1;
                }
            }
        } else {
            match File::open(&file) {
                Ok(f) => {
                    if any_options {
                        if let Err(e) = cat_process(f, &mut out_writer, &opts, &mut line_number, &mut at_line_start, &mut consecutive_newlines) {
                            eprintln!("cat: {}: {}", file, e);
                            exit_code = 1;
                        }
                    } else {
                        if let Err(e) = copy_fast(f, &mut out_writer) {
                            eprintln!("cat: {}: {}", file, e);
                            exit_code = 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("cat: {}: {}", file, e);
                    exit_code = 1;
                }
            }
        }
    }
    
    if let Err(e) = out_writer.flush() {
        eprintln!("cat: {}", e);
        process::exit(1);
    }
    
    process::exit(exit_code);
}
