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
    let mut sys = false;
    let mut node = false;
    let mut rel = false;
    let mut ver = false;
    let mut mach = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" | "--all" => { sys = true; node = true; rel = true; ver = true; mach = true; }
            "-s" | "--kernel-name" => sys = true,
            "-n" | "--nodename" => node = true,
            "-r" | "--kernel-release" => rel = true,
            "-v" | "--kernel-version" => ver = true,
            "-m" | "--machine" => mach = true,
            "--help" => {
                println!("Usage: uname [OPTION]...\nPrint certain system information. With no OPTION, same as -s.\n\n  -a, --all                print all information\n  -s, --kernel-name        print the kernel name\n  -n, --nodename           print the network node hostname\n  -r, --kernel-release     print the kernel release\n  -v, --kernel-version     print the kernel version\n  -m, --machine            print the machine hardware name\n      --help               display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("uname: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => {}
        }
    }

    if !sys && !node && !rel && !ver && !mach { sys = true; }

    #[cfg(unix)]
    unsafe {
        let mut u: Utsname = std::mem::zeroed();
        if uname(&mut u) == 0 {
            let mut out = Vec::new();
            if sys { out.push(CStr::from_ptr(u.sysname.as_ptr()).to_string_lossy().into_owned()); }
            if node { out.push(CStr::from_ptr(u.nodename.as_ptr()).to_string_lossy().into_owned()); }
            if rel { out.push(CStr::from_ptr(u.release.as_ptr()).to_string_lossy().into_owned()); }
            if ver { out.push(CStr::from_ptr(u.version.as_ptr()).to_string_lossy().into_owned()); }
            if mach { out.push(CStr::from_ptr(u.machine.as_ptr()).to_string_lossy().into_owned()); }
            println!("{}", out.join(" "));
            return;
        }
    }

    #[cfg(not(unix))]
    {
        let mut out = Vec::new();
        if sys { out.push("Windows_NT".to_string()); }
        if node { out.push(env::var("COMPUTERNAME").unwrap_or_else(|_| "localhost".to_string())); }
        if rel { out.push("10.0".to_string()); }
        if ver { out.push("Build".to_string()); }
        if mach { out.push("x86_64".to_string()); }
        println!("{}", out.join(" "));
    }
}
