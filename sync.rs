use std::env;
use std::fs::OpenOptions;
use std::process;

#[cfg(unix)]
extern "C" {
    fn sync();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: sync [OPTION]... [FILE]...\nSynchronize cached writes to persistent storage.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("sync: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
    }

    if files.is_empty() {
        #[cfg(unix)]
        unsafe { sync(); }
        #[cfg(not(unix))]
        println!("sync: global sync not supported on this operating system");
    } else {
        let mut exit_code = 0;
        for file in files {
            match OpenOptions::new().read(true).open(&file) {
                Ok(f) => {
                    if let Err(e) = f.sync_all() {
                        eprintln!("sync: error syncing '{}': {}", file, e);
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("sync: error opening '{}': {}", file, e);
                    exit_code = 1;
                }
            }
        }
        process::exit(exit_code);
    }
}
