use std::env;
use std::ffi::CString;
use std::process::{self, Command};

#[cfg(unix)]
extern "C" {
    fn chroot(path: *const std::ffi::c_char) -> i32;
    fn chdir(path: *const std::ffi::c_char) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" {
        println!("Usage: chroot NEWROOT [COMMAND [ARG]...]\nRun COMMAND with root directory set to NEWROOT.\n\n      --help     display this help and exit");
        process::exit(if args.len() < 2 { 125 } else { 0 });
    }

    let new_root = &args[1];
    let cmd_name = if args.len() > 2 { &args[2] } else { "/bin/sh" };
    let cmd_args = if args.len() > 3 { &args[3..] } else { &[] };

    #[cfg(unix)]
    unsafe {
        if let Ok(c_root) = CString::new(new_root.as_str()) {
            if chroot(c_root.as_ptr()) != 0 {
                eprintln!("chroot: cannot change root directory to '{}': {}", new_root, std::io::Error::last_os_error());
                process::exit(125);
            }
            if chdir(b"/\0".as_ptr() as *const _) != 0 {
                eprintln!("chroot: cannot chdir to root directory: {}", std::io::Error::last_os_error());
                process::exit(125);
            }
        } else {
            eprintln!("chroot: invalid path '{}'", new_root);
            process::exit(125);
        }

        let mut command = Command::new(cmd_name);
        command.args(cmd_args);
        use std::os::unix::process::CommandExt;
        let err = command.exec();
        eprintln!("chroot: failed to run command '{}': {}", cmd_name, err);
        if err.kind() == std::io::ErrorKind::NotFound { process::exit(127); } else { process::exit(126); }
    }
    #[cfg(not(unix))]
    {
        eprintln!("chroot: changing root directories is not supported on Windows architectures");
        process::exit(125);
    }
}
