use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn escape_name(name: &str) -> String {
    let mut res = String::new();
    for c in name.chars() {
        match c {
            ' ' => res.push_str("\\ "),
            '\t' => res.push_str("\\t"),
            '\n' => res.push_str("\\n"),
            '\\' => res.push_str("\\\\"),
            _ => res.push(c),
        }
    }
    res
}

fn list_dir_long(dir: &Path, all: bool, reverse: bool) {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !all && name.starts_with('.') { continue; }
            entries.push(entry);
        }
    }
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    if reverse { entries.reverse(); }

    for entry in entries {
        let name = escape_name(&entry.file_name().to_string_lossy());
        if let Ok(meta) = entry.metadata() {
            let kind = if meta.is_dir() { "d" } else if meta.file_type().is_symlink() { "l" } else { "-" };
            println!("{} {:>10}  {}", kind, meta.len(), name);
        } else {
            println!("? {:>10}  {}", "?", name);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut all = false;
    let mut reverse = false;
    let mut paths = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" | "--all" => all = true,
            "-r" | "--reverse" => reverse = true,
            "--help" => {
                println!("Usage: vdir [OPTION]... [FILE]...\nList directory contents in long format.\n\n  -a, --all       do not ignore entries starting with .\n  -r, --reverse   reverse order while sorting\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => { eprintln!("vdir: unrecognized option '{}'", arg); process::exit(1); }
            _ => paths.push(arg.clone()),
        }
    }

    if paths.is_empty() { paths.push(".".to_string()); }
    for path in paths {
        let p = Path::new(&path);
        if p.is_dir() { list_dir_long(p, all, reverse); }
        else if p.exists() {
            let meta = fs::metadata(p).unwrap();
            let kind = if meta.is_dir() { "d" } else { "-" };
            println!("{} {:>10}  {}", kind, meta.len(), escape_name(&path));
        } else {
            eprintln!("vdir: cannot access '{}': No such file or directory", path);
        }
    }
}
