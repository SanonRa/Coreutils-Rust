// Coreutils-Rust
// Copyright (C) 2026 Saketh Rayudu .A
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::env;
use std::process::{self, Command};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[cfg(unix)]
extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

fn parse_duration(s: &str) -> Option<Duration> {
    let mut num_str = s.to_string();
    let mut mult = 1.0;
    if let Some(c) = s.chars().last() {
        match c {
            's' | 'S' => { num_str.pop(); }
            'm' | 'M' => { mult = 60.0; num_str.pop(); }
            'h' | 'H' => { mult = 3600.0; num_str.pop(); }
            'd' | 'D' => { mult = 86400.0; num_str.pop(); }
            _ => {}
        }
    }
    num_str.parse::<f64>().ok().map(|n| Duration::from_secs_f64(n * mult))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sig_num = 15; // SIGTERM
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-s" || arg == "--signal" {
            if i + 1 >= args.len() { eprintln!("timeout: option requires an argument"); process::exit(125); }
            sig_num = args[i + 1].parse().unwrap_or(15);
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-s") {
            sig_num = val.parse().unwrap_or(15);
        } else if arg == "--help" {
            println!("Usage: timeout [OPTION] DURATION COMMAND [ARG]...\nStart COMMAND, and kill it if still running after DURATION.\n\n  -s, --signal=SIGNAL   specify the signal to be sent on timeout (default: TERM)\n      --help            display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("timeout: unrecognized option '{}'", arg);
            process::exit(125);
        } else {
            break;
        }
        i += 1;
    }

    if i + 1 >= args.len() {
        eprintln!("timeout: missing operand\nTry 'timeout --help' for more information.");
        process::exit(125);
    }

    let dur = match parse_duration(&args[i]) {
        Some(d) => d,
        None => { eprintln!("timeout: invalid time interval '{}'", args[i]); process::exit(125); }
    };

    let cmd_name = &args[i + 1];
    let cmd_args = &args[i + 2..];

    let mut child = match Command::new(cmd_name).args(cmd_args).spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("timeout: failed to run command '{}': {}", cmd_name, e);
            if e.kind() == std::io::ErrorKind::NotFound { process::exit(127); } else { process::exit(126); }
        }
    };

    let child_id = child.id();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let res = child.wait();
        let _ = tx.send(res);
    });

    match rx.recv_timeout(dur) {
        Ok(Ok(status)) => {
            process::exit(status.code().unwrap_or(128 + 15));
        }
        Ok(Err(_)) => process::exit(126),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            #[cfg(unix)]
            unsafe { kill(child_id as i32, sig_num); }
            #[cfg(not(unix))]
            let _ = Command::new("taskkill").args(["/F", "/PID", &child_id.to_string()]).status();
            process::exit(124);
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => process::exit(126),
    }
}
