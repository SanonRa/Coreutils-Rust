// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::collections::BTreeMap;
use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ignore_env = false;
    let mut unsets = Vec::new();
    let mut custom_vars = Vec::new();
    let mut cmd_idx = None;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-i" || arg == "-" || arg == "--ignore-environment" {
            ignore_env = true;
        } else if arg == "-u" || arg == "--unset" {
            if i + 1 >= args.len() {
                eprintln!("env: option requires an argument -- '{}'", arg);
                process::exit(125);
            }
            unsets.push(args[i + 1].clone());
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-u") {
            unsets.push(val.to_string());
        } else if arg == "--help" {
            println!("Usage: env [OPTION]... [NAME=VALUE]... [COMMAND [ARG]...]\nSet each NAME to VALUE in the environment and run COMMAND.\n\n  -i, --ignore-environment   start with an empty environment\n  -u, --unset=NAME           remove variable from the environment\n      --help                 display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("env: unrecognized option '{}'", arg);
            process::exit(125);
        } else if arg.contains('=') {
            custom_vars.push(arg.clone());
        } else {
            cmd_idx = Some(i);
            break;
        }
        i += 1;
    }

    let mut env_map: BTreeMap<String, String> = if ignore_env {
        BTreeMap::new()
    } else {
        env::vars().collect()
    };

    for u in &unsets {
        env_map.remove(u);
    }

    for kv in &custom_vars {
        if let Some(idx) = kv.find('=') {
            let (k, v) = (&kv[..idx], &kv[idx + 1..]);
            env_map.insert(k.to_string(), v.to_string());
        }
    }

    match cmd_idx {
        Some(idx) => {
            let cmd_name = &args[idx];
            let cmd_args = &args[idx + 1..];
            let mut command = Command::new(cmd_name);
            command.env_clear();
            command.envs(&env_map);
            command.args(cmd_args);

            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                let err = command.exec();
                eprintln!("env: '{}': {}", cmd_name, err);
                if err.kind() == std::io::ErrorKind::NotFound { process::exit(127); }
                else { process::exit(126); }
            }
            #[cfg(not(unix))]
            {
                match command.status() {
                    Ok(status) => process::exit(status.code().unwrap_or(1)),
                    Err(e) => {
                        eprintln!("env: '{}': {}", cmd_name, e);
                        process::exit(127);
                    }
                }
            }
        }
        None => {
            for (k, v) in env_map {
                println!("{}={}", k, v);
            }
        }
    }
}
