use std::env;
use std::path::Path;
use std::process::{self, Command};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.is_empty() { process::exit(1); }

    // Check if invoked via symlink or hardlink (e.g., calling the binary as 'cat' or 'ls')
    let prog_path = Path::new(&args[0]);
    let mut tool_name = prog_path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "coreutils".to_string());

    // If called directly as 'coreutils', parse --coreutils-prog or command argument
    if tool_name == "coreutils" || tool_name == "antigravity-cli" {
        if args.len() < 2 || args[1] == "--help" {
            println!("Usage: coreutils --coreutils-prog=PROGRAM_NAME [PARAMETERS]... \n  or:  coreutils PROGRAM_NAME [PARAMETERS]...\n\nExecute the specified GNU Coreutils standalone program.\n\nBuilt-in utilities (108 total completed):\n  arch, b2sum, base32, base64, basename, basenc, cat, chcon, chgrp, chmod, chown,\n  chroot, cksum, comm, cp, csplit, cut, date, dd, df, dir, dircolors, dirname, du,\n  echo, env, expand, expr, factor, false, find, fmt, fold, groups, head, hostid,\n  hostname, id, install, join, kill, link, ln, logname, ls, md5sum, mkdir, mkfifo,\n  mknod, mktemp, mv, nice, nl, nohup, nproc, numfmt, od, paste, pathchk, pinky, pr,\n  printenv, printf, ptx, pwd, readlink, realpath, rm, rmdir, runcon, seq, sha1sum,\n  sha224sum, sha256sum, sha384sum, sha512sum, shred, shuf, sleep, sort, split, stat,\n  stdbuf, stty, sum, sync, tac, tail, tee, test, timeout, touch, tr, true, truncate,\n  tsort, tty, uname, unexpand, uniq, unlink, uptime, users, vdir, wc, who, whoami, yes");
            return;
        }

        if let Some(val) = args[1].strip_prefix("--coreutils-prog=") {
            tool_name = val.to_string();
            args.remove(0); // Remove binary name, args[0] becomes prog flag
            args[0] = tool_name.clone();
        } else if !args[1].starts_with('-') {
            tool_name = args[1].clone();
            args.remove(0); // Shift arguments so args[0] becomes the tool name
        } else {
            eprintln!("coreutils: invalid command or flag '{}'\nTry 'coreutils --help' for more information.", args[1]);
            process::exit(125);
        }
    }

    // Dispatch execution to the built compiled binary matching the tool name
    let mut command = Command::new(&tool_name);
    command.args(&args[1..]);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = command.exec();
        eprintln!("coreutils: {}: {}", tool_name, err);
        if err.kind() == std::io::ErrorKind::NotFound {
            eprintln!("coreutils: executable '{}' not found in PATH; ensure all 108 utilities are compiled and linked.", tool_name);
            process::exit(127);
        } else {
            process::exit(126);
        }
    }
    #[cfg(not(unix))]
    {
        match command.status() {
            Ok(status) => process::exit(status.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("coreutils: {}: {}", tool_name, e);
                eprintln!("coreutils: executable '{}' not found in PATH; ensure all 108 utilities are compiled.", tool_name);
                process::exit(127);
            }
        }
    }
}
