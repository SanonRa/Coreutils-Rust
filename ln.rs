use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut symbolic = false;
    let mut force = false;
    let mut verbose = false;
    let mut operands = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-s" | "--symbolic" => symbolic = true,
            "-f" | "--force" => force = true,
            "-v" | "--verbose" => verbose = true,
            "-sf" | "-fs" => { symbolic = true; force = true; }
            "-sfv" | "-fsv" | "-vsf" | "-vfs" => { symbolic = true; force = true; verbose = true; }
            "--help" => {
                println!("Usage: ln [OPTION]... TARGET LINK_NAME\nCreate a link to the specified TARGET with optional LINK_NAME.\n\n  -f, --force      remove existing destination files\n  -s, --symbolic   make symbolic links instead of hard links\n  -v, --verbose    print name of each linked file\n      --help       display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("ln: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => operands.push(arg.clone()),
        }
    }

    if operands.len() != 2 {
        eprintln!("ln: missing file operand or wrong number of arguments\nTry 'ln --help' for more information.");
        process::exit(1);
    }

    let (target, link_name) = (&operands[0], &operands[1]);
    let dest_path = Path::new(link_name);

    if dest_path.exists() || dest_path.is_symlink() {
        if force {
            let _ = fs::remove_file(dest_path);
            let _ = fs::remove_dir(dest_path);
        } else {
            eprintln!("ln: failed to create link '{}': File exists", link_name);
            process::exit(1);
        }
    }

    let res = if symbolic {
        #[cfg(unix)]
        { std::os::unix::fs::symlink(target, link_name) }
        #[cfg(windows)]
        {
            if Path::new(target).is_dir() {
                std::os::windows::fs::symlink_dir(target, link_name)
            } else {
                std::os::windows::fs::symlink_file(target, link_name)
            }
        }
    } else {
        fs::hard_link(target, link_name)
    };

    match res {
        Ok(_) => {
            if verbose {
                let arrow = if symbolic { "->" } else { "=>" };
                println!("'{}' {} '{}'", link_name, arrow, target);
            }
        }
        Err(e) => {
            eprintln!("ln: failed to create link '{}' to '{}': {}", link_name, target, e);
            process::exit(1);
        }
    }
}
