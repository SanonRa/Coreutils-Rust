// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::ffi::CString;
use std::fs;
use std::path::Path;
use std::process;

#[cfg(unix)]
extern "C" {
    fn chown(path: *const std::ffi::c_char, owner: u32, group: u32) -> i32;
    fn lchown(path: *const std::ffi::c_char, owner: u32, group: u32) -> i32;
}

fn resolve_gid(name: &str) -> Option<u32> {
    if let Ok(id) = name.parse::<u32>() { return Some(id); }
    if let Ok(group) = fs::read_to_string("/etc/group") {
        for line in group.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 && parts[0] == name { return parts[2].parse::<u32>().ok(); }
        }
    }
    None
}

fn chgrp_path(path: &Path, gid: u32, recursive: bool, verbose: bool, no_deref: bool) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        if let Ok(c_path) = CString::new(path.to_string_lossy().as_bytes()) {
            let res = unsafe { if no_deref { lchown(c_path.as_ptr(), u32::MAX, gid) } else { chown(c_path.as_ptr(), u32::MAX, gid) } };
            if res != 0 { return Err(std::io::Error::last_os_error()); }
            if verbose { println!("changed group of '{}'", path.display()); }
        }
        if recursive && path.is_dir() && !path.is_symlink() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let _ = chgrp_path(&entry.path(), gid, recursive, verbose, no_deref);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut recursive = false;
    let mut verbose = false;
    let mut no_deref = false;
    let mut group_spec = None;
    let mut files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-R" | "--recursive" => recursive = true,
            "-v" | "--verbose" => verbose = true,
            "-h" | "--no-dereference" => no_deref = true,
            "--help" => {
                println!("Usage: chgrp [OPTION]... GROUP FILE...\nChange the group of each FILE to GROUP.\n\n  -h, --no-dereference   affect symbolic links instead of any referenced file\n  -R, --recursive        operate on files and directories recursively\n  -v, --verbose          output a diagnostic for every file processed\n      --help             display this help and exit");
                return;
            }
            _ if arg.starts_with('-') && group_spec.is_some() => {
                eprintln!("chgrp: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => {
                if group_spec.is_none() { group_spec = Some(arg.clone()); }
                else { files.push(arg.clone()); }
            }
        }
    }

    let spec = match group_spec {
        Some(s) if !files.is_empty() => s,
        _ => {
            eprintln!("chgrp: missing operand\nTry 'chgrp --help' for more information.");
            process::exit(1);
        }
    };

    let gid = match resolve_gid(&spec) {
        Some(id) => id,
        None => { eprintln!("chgrp: invalid group: '{}'", spec); process::exit(1); }
    };

    let mut exit_code = 0;
    for file in files {
        if let Err(e) = chgrp_path(Path::new(&file), gid, recursive, verbose, no_deref) {
            eprintln!("chgrp: changing group of '{}': {}", file, e);
            exit_code = 1;
        }
    }
    process::exit(exit_code);
}
