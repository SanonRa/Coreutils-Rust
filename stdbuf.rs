use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut env_vars = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-i" || arg == "--input" {
            if i + 1 >= args.len() { eprintln!("stdbuf: option requires an argument"); process::exit(125); }
            env_vars.push(("_STDBUF_I", args[i + 1].clone())); i += 1;
        } else if let Some(val) = arg.strip_prefix("-i") {
            env_vars.push(("_STDBUF_I", val.to_string()));
        } else if arg == "-o" || arg == "--output" {
            if i + 1 >= args.len() { eprintln!("stdbuf: option requires an argument"); process::exit(125); }
            env_vars.push(("_STDBUF_O", args[i + 1].clone())); i += 1;
        } else if let Some(val) = arg.strip_prefix("-o") {
            env_vars.push(("_STDBUF_O", val.to_string()));
        } else if arg == "-e" || arg == "--error" {
            if i + 1 >= args.len() { eprintln!("stdbuf: option requires an argument"); process::exit(125); }
            env_vars.push(("_STDBUF_E", args[i + 1].clone())); i += 1;
        } else if let Some(val) = arg.strip_prefix("-e") {
            env_vars.push(("_STDBUF_E", val.to_string()));
        } else if arg == "--help" {
            println!("Usage: stdbuf OPTION... COMMAND [ARG]...\nRun COMMAND, with modified buffering operations for its standard streams.\n\n  -i, --input=MODE    adjust standard input stream buffering\n  -o, --output=MODE   adjust standard output stream buffering\n  -e, --error=MODE    adjust standard error stream buffering\n      --help          display this help and exit\n\nMODE is 'L' for line buffered, '0' for unbuffered, or a byte size.");
            return;
        } else if arg.starts_with('-') {
            eprintln!("stdbuf: unrecognized option '{}'", arg);
            process::exit(125);
        } else {
            break;
        }
        i += 1;
    }

    if i >= args.len() {
        eprintln!("stdbuf: missing operand\nTry 'stdbuf --help' for more information.");
        process::exit(125);
    }

    let cmd_name = &args[i];
    let cmd_args = &args[i + 1..];
    let mut command = Command::new(cmd_name);
    command.args(cmd_args);

    for (k, v) in env_vars {
        command.env(k, v);
    }

    // Inject libstdbuf preload hint if standard library is available
    #[cfg(target_os = "linux")]
    command.env("LD_PRELOAD", "libstdbuf.so");
    #[cfg(target_os = "macos")]
    command.env("DYLD_INSERT_LIBRARIES", "libstdbuf.dylib");

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = command.exec();
        eprintln!("stdbuf: failed to run command '{}': {}", cmd_name, err);
        if err.kind() == std::io::ErrorKind::NotFound { process::exit(127); } else { process::exit(126); }
    }
    #[cfg(not(unix))]
    {
        match command.status() {
            Ok(status) => process::exit(status.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("stdbuf: failed to run command '{}': {}", cmd_name, e);
                process::exit(127);
            }
        }
    }
}
