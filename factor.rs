use std::env;
use std::io::{self, BufRead};
use std::process;

fn factorize(mut n: u64) {
    print!("{}:", n);
    if n < 2 {
        println!();
        return;
    }

    while n % 2 == 0 { print!(" 2"); n /= 2; }
    while n % 3 == 0 { print!(" 3"); n /= 3; }
    while n % 5 == 0 { print!(" 5"); n /= 5; }

    let mut d = 7u64;
    let inc = [4u64, 2, 4, 2, 4, 6, 2, 6];
    let mut idx = 0;

    while d.saturating_mul(d) <= n {
        if n % d == 0 {
            print!(" {}", d);
            n /= d;
        } else {
            d += inc[idx];
            idx = (idx + 1) % 8;
        }
    }
    if n > 1 { print!(" {}", n); }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut operands = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: factor [NUMBER]...\nPrint the prime factors of each specified integer NUMBER.\nIf none are specified on the command line, read them from standard input.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("factor: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            operands.push(arg.clone());
        }
    }

    if operands.is_empty() {
        let stdin = io::stdin().lock();
        for line in stdin.lines().map_while(Result::ok) {
            for word in line.split_whitespace() {
                match word.parse::<u64>() {
                    Ok(num) => factorize(num),
                    Err(_) => eprintln!("factor: '{}' is not a valid positive integer", word),
                }
            }
        }
    } else {
        for word in operands {
            match word.parse::<u64>() {
                Ok(num) => factorize(num),
                Err(_) => {
                    eprintln!("factor: '{}' is not a valid positive integer", word);
                    process::exit(1);
                }
            }
        }
    }
}
