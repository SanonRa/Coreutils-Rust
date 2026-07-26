use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path = "/run/utmp";

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: users [OPTION]... [FILE]\nOutput who is currently logged in according to FILE.\nIf FILE is not specified, use /run/utmp.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("users: unrecognized option '{}'\nTry 'users --help' for more information.", arg);
            process::exit(1);
        } else {
            file_path = arg;
        }
    }

    let mut users = Vec::new();

    if let Ok(mut file) = File::open(file_path).or_else(|_| File::open("/var/run/utmp")) {
        let mut buffer = [0u8; 384];
        while let Ok(n) = file.read(&mut buffer) {
            if n < 384 {
                break;
            }
            let ut_type = i16::from_ne_bytes([buffer[0], buffer[1]]);
            if ut_type == 7 {
                let user_slice = &buffer[44..76];
                if let Some(end) = user_slice.iter().position(|&b| b == 0) {
                    if let Ok(username) = std::str::from_utf8(&user_slice[..end]) {
                        if !username.is_empty() {
                            users.push(username.to_string());
                        }
                    }
                }
            }
        }
    } else if let Ok(user) = env::var("USER") {
        users.push(user);
    }

    users.sort();
    println!("{}", users.join(" "));
}
