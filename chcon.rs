// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::ffi::CString;
use std::path::Path;
use std::process;

#[cfg(target_os = "linux")]
extern "C" {
    fn lsetxattr(path: *const std::ffi::c_char, name: *const std::ffi::c_char, value: *const std::ffi::c_void, size: usize, flags: i32) -> i32;
}

fn set_selinux_context(path: &Path, context: &str) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    unsafe {
        let c_path = CString::new(path.to_string_lossy().as_bytes()).map_err(|e| e.to_string())?;
        let c_name = CString::new("security.selinux").unwrap();
        let c_val = CString::new(context).map_err(|e| e.to_string())?;
        if lsetxattr(c_path.as_ptr(), c_name.as_ptr(), c_val.as_ptr() as *const _, c_val.as_bytes().len() + 1, 0) != 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (path, context);
        Err("SELinux attributes are not supported on non-Linux architectures".to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut context: Option<String> = None;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            println!("Usage: chcon [OPTION]... CONTEXT FILE...\nChange the SELinux security context of each FILE to CONTEXT.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("chcon: unrecognized option '{}'", arg);
            process::exit(1);
        } else if context.is_none() {
            context = Some(arg.clone());
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let target_ctx = match context {
        Some(c) => c,
        None => {
            eprintln!("chcon: missing operand\nTry 'chcon --help' for more information.");
            process::exit(1);
        }
    };

    if files.is_empty() {
        eprintln!("chcon: missing file operand after '{}'", target_ctx);
        process::exit(1);
    }

    let mut exit_code = 0;
    for file in files {
        let p = Path::new(&file);
        if let Err(e) = set_selinux_context(p, &target_ctx) {
            eprintln!("chcon: failed to change context of '{}' to '{}': {}", file, target_ctx, e);
            exit_code = 1;
        }
    }
    process::exit(exit_code);
}
