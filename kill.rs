// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::process;

#[cfg(unix)]
extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

fn parse_signal(s: &str) -> i32 {
    let clean = s.trim_start_matches('-').to_ascii_uppercase();
    if let Ok(num) = clean.parse::<i32>() { return num; }
    match clean.as_str() {
        "HUP" => 1,
        "INT" => 2,
        "QUIT" => 3,
        "ILL" => 4,
        "TRAP" => 5,
        "ABRT" => 6,
        "FPE" => 8,
        "KILL" => 9,
        "SEGV" => 11,
        "PIPE" => 13,
        "ALRM" => 14,
        "TERM" => 15,
        "CONT" => 18,
        "STOP" => 19,
        "TSTP" => 20,
        _ => { eprintln!("kill: invalid signal specification '{}'", s); process::exit(1); }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sig_num = 15; // SIGTERM
    let mut pids = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-l" || arg == "--list" {
            println!(" 1) SIGHUP   2) SIGINT   3) SIGQUIT  4) SIGILL   5) SIGTRAP\n 6) SIGABRT  8) SIGFPE   9) SIGKILL 11) SIGSEGV 13) SIGPIPE\n14) SIGALRM 15) SIGTERM 18) SIGCONT 19) SIGSTOP 20) SIGTSTP");
            return;
        } else if arg == "-s" || arg == "--signal" {
            if i + 1 >= args.len() { eprintln!("kill: option requires an argument"); process::exit(1); }
            sig_num = parse_signal(&args[i + 1]); i += 1;
        } else if arg == "--help" {
            println!("Usage: kill [-s SIGNAL | -SIGNAL] PID...\n  or:  kill -l [SIGNAL]\nSend a signal to processes.\n\n  -s, --signal=SIGNAL   specify the name or number of the signal to be sent\n  -l, --list            list signal names\n      --help            display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" && pids.is_empty() {
            sig_num = parse_signal(arg);
        } else {
            match arg.parse::<i32>() {
                Ok(pid) => pids.push(pid),
                Err(_) => { eprintln!("kill: invalid process id '{}'", arg); process::exit(1); }
            }
        }
        i += 1;
    }

    if pids.is_empty() {
        eprintln!("kill: not enough arguments\nTry 'kill --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for pid in pids {
        #[cfg(unix)]
        unsafe {
            if kill(pid, sig_num) != 0 {
                eprintln!("kill: sending signal to {}: {}", pid, std::io::Error::last_os_error());
                exit_code = 1;
            }
        }
        #[cfg(not(unix))]
        {
            let _ = sig_num;
            let status = process::Command::new("taskkill").args(["/F", "/PID", &pid.to_string()]).status();
            if status.is_err() || !status.unwrap().success() {
                eprintln!("kill: failed to terminate process {}", pid);
                exit_code = 1;
            }
        }
    }
    process::exit(exit_code);
}
