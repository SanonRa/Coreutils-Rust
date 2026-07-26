use std::env;
use std::process;

#[cfg(unix)]
extern "C" {
    fn gethostid() -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: hostid [OPTION]...\nPrint the numeric identifier (in hexadecimal) for the current host.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("hostid: unrecognized option '{}'\nTry 'hostid --help' for more information.", arg);
            process::exit(1);
        }
    }

    #[cfg(unix)]
    {
        let id = unsafe { gethostid() };
        println!("{:08x}", id as u32);
    }
    #[cfg(not(unix))]
    {
        eprintln!("hostid: unsupported platform");
        process::exit(1);
    }
}
