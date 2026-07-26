// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
fn parse_symbolic_mode(current_mode: u32, spec: &str) -> Result<u32, String> {
    let mut mode = current_mode & 0o7777; // preserve special bits too
    for clause in spec.split(',') {
        let clause = clause.trim();
        if clause.is_empty() { continue; }
        
        let op_idx = clause.find(|c| c == '+' || c == '-' || c == '=')
            .ok_or_else(|| format!("invalid mode spec: '{}'", clause))?;
            
        let users_part = &clause[..op_idx];
        let op = clause.chars().nth(op_idx).unwrap();
        let perms_part = &clause[op_idx + 1..];
        
        let mut apply_user = false;
        let mut apply_group = false;
        let mut apply_other = false;
        
        if users_part.is_empty() {
            apply_user = true;
            apply_group = true;
            apply_other = true;
        } else {
            for c in users_part.chars() {
                match c {
                    'u' => apply_user = true,
                    'g' => apply_group = true,
                    'o' => apply_other = true,
                    'a' => {
                        apply_user = true;
                        apply_group = true;
                        apply_other = true;
                    }
                    _ => return Err(format!("invalid user: '{}'", c)),
                }
            }
        }
        
        let mut perm_bits = 0;
        for c in perms_part.chars() {
            match c {
                'r' => {
                    if apply_user { perm_bits |= 0o400; }
                    if apply_group { perm_bits |= 0o040; }
                    if apply_other { perm_bits |= 0o004; }
                }
                'w' => {
                    if apply_user { perm_bits |= 0o200; }
                    if apply_group { perm_bits |= 0o020; }
                    if apply_other { perm_bits |= 0o002; }
                }
                'x' => {
                    if apply_user { perm_bits |= 0o100; }
                    if apply_group { perm_bits |= 0o010; }
                    if apply_other { perm_bits |= 0o001; }
                }
                _ => return Err(format!("invalid permission: '{}'", c)),
            }
        }
        
        let mut affected_mask = 0;
        if apply_user { affected_mask |= 0o700; }
        if apply_group { affected_mask |= 0o070; }
        if apply_other { affected_mask |= 0o007; }
        
        match op {
            '+' => mode |= perm_bits,
            '-' => mode &= !perm_bits,
            '=' => mode = (mode & !affected_mask) | perm_bits,
            _ => unreachable!(),
        }
    }
    Ok(mode)
}

fn chmod_path(path: &Path, mode_spec: &str, recursive: bool, verbose: bool) -> std::io::Result<()> {
    let meta = fs::symlink_metadata(path)?;
    
    #[cfg(unix)]
    {
        let current_mode = meta.permissions().mode();
        let new_mode = if let Ok(octal) = u32::from_str_radix(mode_spec, 8) {
            octal
        } else {
            match parse_symbolic_mode(current_mode, mode_spec) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("chmod: {}", e);
                    process::exit(1);
                }
            }
        };
        
        if (current_mode & 0o7777) != (new_mode & 0o7777) {
            let perms = fs::Permissions::from_mode(new_mode);
            fs::set_permissions(path, perms)?;
            if verbose {
                println!("mode of '{}' changed to {:o}", path.display(), new_mode & 0o7777);
            }
        }
    }
    
    if recursive && meta.is_dir() && !meta.file_type().is_symlink() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let _ = chmod_path(&entry.path(), mode_spec, recursive, verbose);
        }
    }
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut recursive = false;
    let mut verbose = false;
    let mut mode_spec = None;
    let mut files = Vec::new();
    
    for arg in &args[1..] {
        match arg.as_str() {
            "-R" | "--recursive" => recursive = true,
            "-v" | "--verbose" => verbose = true,
            "--help" => {
                println!("Usage: chmod [OPTION]... MODE[,MODE]... FILE...\nChange the mode of each FILE to MODE.\n\n  -R, --recursive        change files and directories recursively\n  -v, --verbose          output a diagnostic for every file processed\n      --help             display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("chmod: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => {
                if mode_spec.is_none() {
                    mode_spec = Some(arg.clone());
                } else {
                    files.push(arg.clone());
                }
            }
        }
    }
    
    let spec = match mode_spec {
        Some(s) if !files.is_empty() => s,
        _ => {
            eprintln!("chmod: missing operand\nTry 'chmod --help' for more information.");
            process::exit(1);
        }
    };
    
    let mut exit_code = 0;
    for file in files {
        if let Err(e) = chmod_path(Path::new(&file), &spec, recursive, verbose) {
            eprintln!("chmod: cannot access '{}': {}", file, e);
            exit_code = 1;
        }
    }
    process::exit(exit_code);
}
