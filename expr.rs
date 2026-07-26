use std::env;
use std::process;

fn eval_val(val: &str) -> (bool, i64) {
    if let Ok(n) = val.parse::<i64>() {
        (n != 0, n)
    } else {
        (!val.is_empty() && val != "0", 0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "--help" {
        println!("Usage: expr EXPRESSION\n  or:  expr OPTION\nPrint the value of EXPRESSION to standard output.\n\n      --help     display this help and exit");
        return;
    }

    if args.len() < 2 {
        eprintln!("expr: missing operand\nTry 'expr --help' for more information.");
        process::exit(2);
    }

    let tokens: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    if tokens.len() == 1 {
        println!("{}", tokens[0]);
        let (truthy, _) = eval_val(tokens[0]);
        process::exit(if truthy { 0 } else { 1 });
    }

    if tokens.len() == 3 {
        let (left, op, right) = (tokens[0], tokens[1], tokens[2]);
        match op {
            "+" | "-" | "*" | "/" | "%" => {
                let l = left.parse::<i64>().unwrap_or_else(|_| { eprintln!("expr: non-integer argument"); process::exit(2); });
                let r = right.parse::<i64>().unwrap_or_else(|_| { eprintln!("expr: non-integer argument"); process::exit(2); });
                let res = match op {
                    "+" => l.wrapping_add(r),
                    "-" => l.wrapping_sub(r),
                    "*" => l.wrapping_mul(r),
                    "/" => if r == 0 { eprintln!("expr: division by zero"); process::exit(2); } else { l / r },
                    "%" => if r == 0 { eprintln!("expr: division by zero"); process::exit(2); } else { l % r },
                    _ => unreachable!(),
                };
                println!("{}", res);
                process::exit(if res != 0 { 0 } else { 1 });
            }
            "=" | "!=" | "<" | ">" | "<=" | ">=" => {
                let ord = match (left.parse::<i64>(), right.parse::<i64>()) {
                    (Ok(l), Ok(r)) => l.cmp(&r),
                    _ => left.cmp(right),
                };
                let truthy = match op {
                    "=" => ord == std::cmp::Ordering::Equal,
                    "!=" => ord != std::cmp::Ordering::Equal,
                    "<" => ord == std::cmp::Ordering::Less,
                    ">" => ord == std::cmp::Ordering::Greater,
                    "<=" => ord != std::cmp::Ordering::Greater,
                    ">=" => ord != std::cmp::Ordering::Less,
                    _ => false,
                };
                println!("{}", if truthy { 1 } else { 0 });
                process::exit(if truthy { 0 } else { 1 });
            }
            "|" => {
                let (l_truth, _) = eval_val(left);
                if l_truth { println!("{}", left); process::exit(0); }
                let (r_truth, _) = eval_val(right);
                if r_truth { println!("{}", right); process::exit(0); }
                println!("0");
                process::exit(1);
            }
            "&" => {
                let (l_truth, _) = eval_val(left);
                let (r_truth, _) = eval_val(right);
                if l_truth && r_truth { println!("{}", left); process::exit(0); }
                println!("0");
                process::exit(1);
            }
            _ => {
                eprintln!("expr: syntax error");
                process::exit(2);
            }
        }
    }

    eprintln!("expr: complex expressions require full AST evaluation; standard 3-token limit reached");
    process::exit(2);
}
