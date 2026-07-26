use std::env;
use std::ffi::CStr;
use std::process;

#[cfg(unix)]
#[repr(C)]
struct Utsname {
    sysname: [std::ffi::c_char; 65],
    nodename: [std::ffi::c_char; 65],
    release: [std::ffi::c_char; 65],
    version: [std::ffi::c_char; 65],
    machine: [std::ffi::c_char; 65],
    _domainname: [std::ffi::c_char; 65],
}

#[cfg(unix)]
extern "C" {
    fn uname(buf: *mut Utsname) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: arch [OPTION]...\nPrint machine architecture.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("arch: unrecognized option '{}'", arg);
            process::exit(1);
        }
    }

    #[cfg(unix)]
    unsafe {
        let mut u: Utsname = std::mem::zeroed();
        if uname(&mut u) == 0 {
            println!("{}", CStr::from_ptr(u.machine.as_ptr()).to_string_lossy());
            return;
        }
    }

    #[cfg(not(unix))]
    {
        if let Ok(arch) = env::var("PROCESSOR_ARCHITECTURE") {
            println!("{}", arch.to_lowercase());
        } else {
            println!("x86_64");
        }
    }
}
