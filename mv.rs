use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn move_item(src: &Path, dst: &Path, verbose: bool, force: bool) -> std::io::Result<()> {
    if dst.exists() && force {
        if dst.is_dir() { let _ = fs::remove_dir_all(dst); }
        else { let _ = fs::remove_file(dst); }
    }

    match fs::rename(src, dst) {
        Ok(_) => {
            if verbose { println!("renamed '{}' -> '{}'", src.display(), dst.display()); }
            Ok(())
        }
        Err(_) => {
            if src.is_dir() {
                // Cross-device fallback for directory
                copy_dir_recursive(src, dst)?;
                fs::remove_dir_all(src)?;
            } else {
                fs::copy(src, dst)?;
                fs::remove_file(src)?;
            }
            if verbose { println!("'{}' -> '{}'", src.display(), dst.display()); }
            Ok(())
        }
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let new_dst = dst.join(entry.file_name());
        if entry.path().is_dir() { copy_dir_recursive(&entry.path(), &new_dst)?; }
        else { fs::copy(entry.path(), &new_dst)?; }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut force = false;
    let mut verbose = false;
    let mut paths = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-f" | "--force" => force = true,
            "-v" | "--verbose" => verbose = true,
            "-fv" | "-vf" => { force = true; verbose = true; }
            "--help" => {
                println!("Usage: mv [OPTION]... SOURCE... DIRECTORY\nRename SOURCE to DEST, or move SOURCE(s) to DIRECTORY.\n\n  -f, --force    do not prompt before overwriting\n  -v, --verbose  explain what is being done\n      --help     display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("mv: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => paths.push(PathBuf::from(arg)),
        }
    }

    if paths.len() < 2 {
        eprintln!("mv: missing file operand\nTry 'mv --help' for more information.");
        process::exit(1);
    }

    let dest = paths.pop().unwrap();
    let mut exit_code = 0;

    if paths.len() > 1 || dest.is_dir() {
        if !dest.is_dir() {
            eprintln!("mv: target '{}' is not a directory", dest.display());
            process::exit(1);
        }
        for src in paths {
            let target = dest.join(src.file_name().unwrap_or_default());
            if let Err(e) = move_item(&src, &target, verbose, force) {
                eprintln!("mv: cannot move '{}' to '{}': {}", src.display(), target.display(), e);
                exit_code = 1;
            }
        }
    } else {
        let src = &paths[0];
        if let Err(e) = move_item(src, &dest, verbose, force) {
            eprintln!("mv: cannot move '{}' to '{}': {}", src.display(), dest.display(), e);
            process::exit(1);
        }
    }
    process::exit(exit_code);
}
