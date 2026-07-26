use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn remove_dir_clean(path: &Path, verbose: bool, ignore_non_empty: bool) -> Result<(), std::io::Error> {
    match fs::remove_dir(path) {
        Ok(_) => {
            if verbose { println!("rmdir: removing directory, '{}'", path.display()); }
            Ok(())
        }
        Err(e) => {
            if ignore_non_empty && (e.kind() == std::io::ErrorKind::DirectoryNotEmpty || e.raw_os_error() == Some(39) || e.raw_os_error() == Some(66)) {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parents = false;
    let mut verbose = false;
    let mut ignore_non_empty = false;
    let mut dirs = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-p" | "--parents" => parents = true,
            "-v" | "--verbose" => verbose = true,
            "--ignore-fail-on-non-empty" => ignore_non_empty = true,
            "-pv" | "-vp" => { parents = true; verbose = true; }
            "--help" => {
                println!("Usage: rmdir [OPTION]... DIRECTORY...\nRemove the DIRECTORY(ies), if they are empty.\n\n  --ignore-fail-on-non-empty   ignore each failure that is solely because a directory\n                               is non-empty\n  -p, --parents                remove DIRECTORY and its ancestors\n  -v, --verbose                output a diagnostic for every directory processed\n      --help                   display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("rmdir: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => dirs.push(arg.clone()),
        }
    }

    if dirs.is_empty() {
        eprintln!("rmdir: missing operand\nTry 'rmdir --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for dir in dirs {
        let mut curr = Path::new(&dir);
        if let Err(e) = remove_dir_clean(curr, verbose, ignore_non_empty) {
            eprintln!("rmdir: failed to remove '{}': {}", curr.display(), e);
            exit_code = 1;
            continue;
        }

        if parents {
            while let Some(parent) = curr.parent() {
                if parent.as_os_str().is_empty() || parent == Path::new(".") || parent == Path::new("/") { break; }
                curr = parent;
                if let Err(_) = remove_dir_clean(curr, verbose, ignore_non_empty) {
                    break;
                }
            }
        }
    }
    process::exit(exit_code);
}
