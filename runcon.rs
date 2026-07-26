use std::env;
use std::fs;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] == "--help" {
        println!("Usage: runcon CONTEXT COMMAND [args...]\nRun a program in a different SELinux security context.\n\n      --help     display this help and exit");
        process::exit(if args.len() < 3 { 125 } else { 0 });
    }

    let context = &args[1];
    let cmd_name = &args[2];
    let cmd_args = &args[3..];

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = fs::write("/proc/self/attr/exec", context.as_bytes()) {
            eprintln!("runcon: warning: failed to set SELinux exec context to '{}': {}", context, e);
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!("runcon: warning: SELinux execution contexts are unsupported on this operating system");
    }

    let mut command = Command::new(cmd_name);
    command.args(cmd_args);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = command.exec();
        eprintln!("runcon: '{}': {}", cmd_name, err);
        if err.kind() == std::io::ErrorKind::NotFound { process::exit(127); } else { process::exit(126); }
    }
    #[cfg(not(unix))]
    {
        match command.status() {
            Ok(status) => process::exit(status.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("runcon: '{}': {}", cmd_name, e);
                process::exit(127);
            }
        }
    }
}
