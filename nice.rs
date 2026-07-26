use std::env;
use std::process::{self, Command};

#[cfg(unix)]
extern "C" {
    fn getpriority(which: i32, who: u32) -> i32;
    fn setpriority(which: i32, who: u32, prio: i32) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut adjustment = 10i32;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-n" || arg == "--adjustment" {
            if i + 1 >= args.len() {
                eprintln!("nice: option requires an argument -- '{}'", arg);
                process::exit(125);
            }
            adjustment = args[i + 1].parse().unwrap_or(10);
            i += 1;
        } else if let Some(val) = arg.strip_prefix("-n") {
            adjustment = val.parse().unwrap_or(10);
        } else if arg == "--help" {
            println!("Usage: nice [OPTION] [COMMAND [ARG]...]\nRun COMMAND with an adjusted niceness, which affects process scheduling.\nWith no COMMAND, print the current niceness.\n\n  -n, --adjustment=N   add integer N to the niceness (default 10)\n      --help           display this help and exit");
            return;
        } else if arg.starts_with('-') && arg.len() > 1 && (arg.chars().nth(1).unwrap().is_ascii_digit() || arg.chars().nth(1).unwrap() == '-') {
            adjustment = arg[1..].parse().unwrap_or(10);
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("nice: unrecognized option '{}'", arg);
            process::exit(125);
        } else {
            break;
        }
        i += 1;
    }

    #[cfg(unix)]
    let current_prio = unsafe { getpriority(0, 0) };
    #[cfg(not(unix))]
    let current_prio = 0;

    if i >= args.len() {
        println!("{}", current_prio);
        return;
    }

    #[cfg(unix)]
    unsafe {
        let new_prio = (current_prio + adjustment).clamp(-20, 19);
        let _ = setpriority(0, 0, new_prio);
    }

    let cmd_name = &args[i];
    let cmd_args = &args[i + 1..];
    let mut command = Command::new(cmd_name);
    command.args(cmd_args);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = command.exec();
        eprintln!("nice: '{}': {}", cmd_name, err);
        if err.kind() == std::io::ErrorKind::NotFound { process::exit(127); }
        else { process::exit(126); }
    }
    #[cfg(not(unix))]
    {
        match command.status() {
            Ok(status) => process::exit(status.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("nice: '{}': {}", cmd_name, e);
                process::exit(127);
            }
        }
    }
}
