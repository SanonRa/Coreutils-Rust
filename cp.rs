// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

fn copy_recursive(src: &Path, dst: &Path, verbose: bool, force: bool) -> io::Result<()> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
            if verbose { println!("created directory '{}'", dst.display()); }
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = dst.join(entry.file_name());
            copy_recursive(&entry_path, &dest_path, verbose, force)?;
        }
    } else {
        if dst.exists() && force {
            let _ = fs::remove_file(dst);
        }
        fs::copy(src, dst)?;
        if verbose { println!("'{}' -> '{}'", src.display(), dst.display()); }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut recursive = false;
    let mut force = false;
    let mut verbose = false;
    let mut paths = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-r" | "-R" | "--recursive" => recursive = true,
            "-f" | "--force" => force = true,
            "-v" | "--verbose" => verbose = true,
            "-rf" | "-fr" | "-rv" | "-vr" | "-rfv" | "-frv" | "-vrf" => {
                if arg.contains('r') || arg.contains('R') { recursive = true; }
                if arg.contains('f') { force = true; }
                if arg.contains('v') { verbose = true; }
            }
            "--help" => {
                println!("Usage: cp [OPTION]... SOURCE... DIRECTORY\nCopy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n\n  -f, --force       if an existing destination file cannot be opened, remove it and try again\n  -r, -R, --recursive copy directories recursively\n  -v, --verbose     explain what is being done\n      --help        display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("cp: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => paths.push(PathBuf::from(arg)),
        }
    }

    if paths.len() < 2 {
        eprintln!("cp: missing file operand\nTry 'cp --help' for more information.");
        process::exit(1);
    }

    let dest = paths.pop().unwrap();
    let mut exit_code = 0;

    if paths.len() > 1 || dest.is_dir() {
        if !dest.exists() {
            eprintln!("cp: target '{}' is not a directory", dest.display());
            process::exit(1);
        }
        for src in paths {
            let target_file = dest.join(src.file_name().unwrap_or_default());
            if src.is_dir() && !recursive {
                eprintln!("cp: -r not specified; omitting directory '{}'", src.display());
                exit_code = 1;
                continue;
            }
            if let Err(e) = copy_recursive(&src, &target_file, verbose, force) {
                eprintln!("cp: cannot copy '{}' to '{}': {}", src.display(), target_file.display(), e);
                exit_code = 1;
            }
        }
    } else {
        let src = &paths[0];
        if src.is_dir() && !recursive {
            eprintln!("cp: -r not specified; omitting directory '{}'", src.display());
            process::exit(1);
        }
        if let Err(e) = copy_recursive(src, &dest, verbose, force) {
            eprintln!("cp: cannot copy '{}' to '{}': {}", src.display(), dest.display(), e);
            process::exit(1);
        }
    }
    process::exit(exit_code);
}
