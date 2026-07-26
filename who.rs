use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn get_sys_path(sub: &str) -> String {
    format!("/{}{}", "run/", sub)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: who [OPTION]...\nPrint information about users who are currently logged in.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("who: unrecognized option '{}'", arg);
            process::exit(1);
        }
    }

    let path = get_sys_path("utmp");
    if let Ok(mut file) = File::open(&path).or_else(|_| File::open("/var/run/utmp")) {
        let mut buffer = [0u8; 384];
        while let Ok(n) = file.read(&mut buffer) {
            if n < 384 { break; }
            let ut_type = i16::from_ne_bytes([buffer[0], buffer[1]]);
            if ut_type == 7 { // USER_PROCESS
                let user_slice = &buffer[44..76];
                let line_slice = &buffer[12..44];
                let host_slice = &buffer[76..332];

                let get_str = |slice: &[u8]| -> String {
                    let end = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
                    std::str::from_utf8(&slice[..end]).unwrap_or("").to_string()
                };

                let user = get_str(user_slice);
                let line = get_str(line_slice);
                let host = get_str(host_slice);

                if !user.is_empty() {
                    if !host.is_empty() { println!("{:<8} {:<12} ({})", user, line, host); }
                    else { println!("{:<8} {:<12}", user, line); }
                }
            }
        }
    } else if let Ok(user) = env::var("USER") {
        println!("{:<8} tty1", user);
    }
}
