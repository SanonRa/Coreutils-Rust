use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut supp_1 = false;
    let mut supp_2 = false;
    let mut supp_3 = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-1" => supp_1 = true,
            "-2" => supp_2 = true,
            "-3" => supp_3 = true,
            "-12" => { supp_1 = true; supp_2 = true; }
            "-13" => { supp_1 = true; supp_3 = true; }
            "-23" => { supp_2 = true; supp_3 = true; }
            "-123" => { supp_1 = true; supp_2 = true; supp_3 = true; }
            "--help" => {
                println!("Usage: comm [OPTION]... FILE1 FILE2\nCompare sorted files FILE1 and FILE2 line by line.\n\n  -1              suppress column 1 (lines unique to FILE1)\n  -2              suppress column 2 (lines unique to FILE2)\n  -3              suppress column 3 (lines that appear in both files)\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && arg != "-" => { eprintln!("comm: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
    }

    if files.len() != 2 { eprintln!("comm: missing operand or wrong number of arguments\nTry 'comm --help' for more information."); process::exit(1); }

    let read_lines = |path: &str| -> Vec<String> {
        let reader: Box<dyn Read> = if path == "-" { Box::new(io::stdin()) } else {
            File::open(path).map(|f| Box::new(f) as Box<dyn Read>).unwrap_or_else(|e| { eprintln!("comm: {}: {}", path, e); process::exit(1); })
        };
        BufReader::new(reader).lines().map_while(Result::ok).collect()
    };

    let lines1 = read_lines(&files[0]);
    let lines2 = read_lines(&files[1]);

    let col1_prefix = "";
    let col2_prefix = if supp_1 { "" } else { "\t" };
    let col3_prefix = match (supp_1, supp_2) {
        (true, true) => "",
        (true, false) | (false, true) => "\t",
        (false, false) => "\t\t",
    };

    let mut i = 0;
    let mut j = 0;

    while i < lines1.len() && j < lines2.len() {
        match lines1[i].cmp(&lines2[j]) {
            Ordering::Less => {
                if !supp_1 { println!("{}{}", col1_prefix, lines1[i]); }
                i += 1;
            }
            Ordering::Greater => {
                if !supp_2 { println!("{}{}", col2_prefix, lines2[j]); }
                j += 1;
            }
            Ordering::Equal => {
                if !supp_3 { println!("{}{}", col3_prefix, lines1[i]); }
                i += 1; j += 1;
            }
        }
    }

    while i < lines1.len() {
        if !supp_1 { println!("{}{}", col1_prefix, lines1[i]); }
        i += 1;
    }
    while j < lines2.len() {
        if !supp_2 { println!("{}{}", col2_prefix, lines2[j]); }
        j += 1;
    }
}
