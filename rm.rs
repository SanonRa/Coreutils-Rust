use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut recursive = false;
    let mut force = false;
    let mut verbose = false;
    let mut targets = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-r" | "-R" | "--recursive" => recursive = true,
            "-f" | "--force" => force = true,
            "-v" | "--verbose" => verbose = true,
            "-rf" | "-fr" | "-rF" | "-Fr" => { recursive = true; force = true; }
            "-rfv" | "-frv" | "-vrf" | "-vfr" => { recursive = true; force = true; verbose = true; }
            "--help" => {
                println!("Usage: rm [OPTION]... FILE...\nRemove (unlink) the FILE(s).\n\n  -f, --force       ignore nonexistent files and arguments, never prompt\n  -r, -R, --recursive   remove directories and their contents recursively\n  -v, --verbose     explain what is being done\n      --help        display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                if !force { eprintln!("rm: unrecognized option '{}'", arg); }
                process::exit(1);
            }
            _ => targets.push(arg.clone()),
        }
    }

    if targets.is_empty() && !force {
        eprintln!("rm: missing operand\nTry 'rm --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for target in targets {
        let path = Path::new(&target);
        if !path.exists() && !path.is_symlink() {
            if !force {
                eprintln!("rm: cannot remove '{}': No such file or directory", target);
                exit_code = 1;
            }
            continue;
        }

        let res = if path.is_dir() && !path.is_symlink() {
            if recursive {
                fs::remove_dir_all(path)
            } else {
                fs::remove_dir(path)
            }
        } else {
            fs::remove_file(path)
        };

        match res {
            Ok(_) => {
                if verbose { println!("removed '{}'", target); }
            }
            Err(e) => {
                if !force {
                    eprintln!("rm: cannot remove '{}': {}", target, e);
                    exit_code = 1;
                }
            }
        }
    }
    process::exit(exit_code);
}
