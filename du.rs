use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

struct DuOptions {
    summarize: bool,
    human: bool,
    all: bool,
}

fn format_size(bytes: u64, human: bool) -> String {
    if !human { return ((bytes + 1023) / 1024).to_string(); }
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 { format!("{}", bytes) }
    else if size >= 10.0 { format!("{:.0}{}", size, UNITS[unit_idx]) }
    else { format!("{:.1}{}", size, UNITS[unit_idx]) }
}

fn calc_usage(path: &Path, opts: &DuOptions, visited: &mut HashSet<(u64, u64)>) -> u64 {
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) => { eprintln!("du: cannot access '{}': {}", path.display(), e); return 0; }
    };

    #[cfg(unix)]
    let (dev, ino, bytes) = (meta.dev(), meta.ino(), meta.blocks() * 512);
    #[cfg(not(unix))]
    let (dev, ino, bytes) = (0, 0, meta.len());

    if !meta.is_dir() {
        if !visited.insert((dev, ino)) { return 0; }
        if opts.all && !opts.summarize {
            println!("{}\t{}", format_size(bytes, opts.human), path.display());
        }
        return bytes;
    }

    let mut total = bytes;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            total += calc_usage(&entry.path(), opts, visited);
        }
    }

    if !opts.summarize {
        println!("{}\t{}", format_size(total, opts.human), path.display());
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = DuOptions { summarize: false, human: false, all: false };
    let mut targets = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-s" | "--summarize" => opts.summarize = true,
            "-h" | "--human-readable" => opts.human = true,
            "-a" | "--all" => opts.all = true,
            "-sh" | "-hs" => { opts.summarize = true; opts.human = true; }
            "-ah" | "-ha" => { opts.all = true; opts.human = true; }
            "--help" => {
                println!("Usage: du [OPTION]... [FILE]...\nEstimate file space usage.\n\n  -a, --all            write counts for all files, not just directories\n  -h, --human-readable print sizes in human readable format (e.g., 1K 234M)\n  -s, --summarize      display only a total for each argument\n      --help           display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("du: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => targets.push(arg.clone()),
        }
    }

    if targets.is_empty() { targets.push(".".to_string()); }

    let mut visited = HashSet::new();
    for target in targets {
        let path = Path::new(&target);
        let total = calc_usage(path, &opts, &mut visited);
        if opts.summarize {
            println!("{}\t{}", format_size(total, opts.human), path.display());
        }
    }
}
