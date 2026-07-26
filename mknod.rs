use std::env;
use std::ffi::CString;
use std::process;

#[cfg(unix)]
extern "C" {
    fn mknod(path: *const std::ffi::c_char, mode: u32, dev: u64) -> i32;
    fn gnu_dev_makedev(maj: u32, min: u32) -> u64;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = 0o666u32;
    let mut operands = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-m" || arg == "--mode" {
            if i + 1 >= args.len() { eprintln!("mknod: option requires an argument"); process::exit(1); }
            mode = u32::from_str_radix(&args[i + 1], 8).unwrap_or(0o666); i += 1;
        } else if let Some(val) = arg.strip_prefix("-m") {
            mode = u32::from_str_radix(val, 8).unwrap_or(0o666);
        } else if arg == "--help" {
            println!("Usage: mknod [OPTION]... NAME TYPE [MAJOR MINOR]\nCreate the special file NAME of the given TYPE.\n\n  -m, --mode=MODE   set file permission bits to MODE, not a=rw - umask\n  TYPEs include:\n    b      create a block (buffered) special file\n    c, u   create a character (unbuffered) special file\n    p      create a FIFO\n      --help        display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("mknod: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            operands.push(arg.clone());
        }
        i += 1;
    }

    if operands.len() < 2 {
        eprintln!("mknod: missing operand\nTry 'mknod --help' for more information.");
        process::exit(1);
    }

    let name = &operands[0];
    let kind = operands[1].chars().next().unwrap_or('p');

    #[cfg(unix)]
    unsafe {
        let (file_type, dev) = match kind {
            'p' => (0o010000, 0), // S_IFIFO
            'c' | 'u' => {
                if operands.len() < 4 { eprintln!("mknod: special files require major and minor numbers"); process::exit(1); }
                let maj: u32 = operands[2].parse().unwrap_or(0);
                let min: u32 = operands[3].parse().unwrap_or(0);
                (0o020000, gnu_dev_makedev(maj, min)) // S_IFCHR
            }
            'b' => {
                if operands.len() < 4 { eprintln!("mknod: special files require major and minor numbers"); process::exit(1); }
                let maj: u32 = operands[2].parse().unwrap_or(0);
                let min: u32 = operands[3].parse().unwrap_or(0);
                (0o060000, gnu_dev_makedev(maj, min)) // S_IFBLK
            }
            _ => { eprintln!("mknod: invalid file type '{}'", kind); process::exit(1); }
        };

        if let Ok(c_path) = CString::new(name.as_str()) {
            if mknod(c_path.as_ptr(), file_type | (mode & 0o7777), dev) != 0 {
                eprintln!("mknod: {}: {}", name, std::io::Error::last_os_error());
                process::exit(1);
            }
        }
    }
    #[cfg(not(unix))]
    {
        eprintln!("mknod: creating device nodes and named pipes is not supported on Windows architectures");
        process::exit(1);
    }
}
