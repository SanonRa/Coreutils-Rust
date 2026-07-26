use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--help" {
        println!("Usage: tsort [OPTION] [FILE]\nWrite totally ordered list consistent with the partial ordering in FILE.\n\n      --help     display this help and exit");
        return;
    }

    let file_arg = args.get(1).map(|s| s.as_str()).unwrap_or("-");
    let reader: Box<dyn Read> = if file_arg == "-" {
        Box::new(io::stdin())
    } else match File::open(file_arg) {
        Ok(f) => Box::new(f),
        Err(e) => {
            eprintln!("tsort: {}: {}", file_arg, e);
            process::exit(1);
        }
    };

    let mut tokens = Vec::new();
    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        for word in line.split_whitespace() {
            tokens.push(word.to_string());
        }
    }

    if tokens.len() % 2 != 0 {
        eprintln!("tsort: {}: input contains an odd number of tokens", file_arg);
        process::exit(1);
    }

    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();

    for chunk in tokens.chunks(2) {
        let (u, v) = (&chunk[0], &chunk[1]);
        adj.entry(u.clone()).or_default();
        adj.entry(v.clone()).or_default();
        in_degree.entry(u.clone()).or_insert(0);
        in_degree.entry(v.clone()).or_insert(0);

        if u != v && adj.get_mut(u).unwrap().insert(v.clone()) {
            *in_degree.get_mut(v).unwrap() += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (node, &deg) in &in_degree {
        if deg == 0 { queue.push_back(node.clone()); }
    }

    let mut sorted = Vec::new();
    while let Some(curr) = queue.pop_front() {
        sorted.push(curr.clone());
        if let Some(neighbors) = adj.get(&curr) {
            for next in neighbors {
                let deg = in_degree.get_mut(next).unwrap();
                *deg -= 1;
                if *deg == 0 { queue.push_back(next.clone()); }
            }
        }
    }

    if sorted.len() < adj.len() {
        eprintln!("tsort: {}: input contains a loop:", file_arg);
        for (node, &deg) in &in_degree {
            if deg > 0 { eprintln!("tsort: {}", node); }
        }
        process::exit(1);
    }

    for node in sorted {
        println!("{}", node);
    }
}
