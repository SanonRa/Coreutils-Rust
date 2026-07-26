use std::env;
use std::fs;
use std::process;

fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{} day{}, {:02}:{:02}", days, if days == 1 { "" } else { "s" }, hours, mins)
    } else {
        format!("{:02}:{:02}", hours, mins)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: uptime [OPTION]...\nTell how long the system has been running.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("uptime: unrecognized option '{}'", arg);
            process::exit(1);
        }
    }

    let up_secs = if let Ok(content) = fs::read_to_string("/proc/uptime") {
        content.split_whitespace().next().and_then(|s| s.parse::<f64>().ok()).map(|f| f as u64).unwrap_or(3600)
    } else { 3600 };

    let load_str = if let Ok(content) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = content.split_whitespace().take(3).collect();
        parts.join(", ")
    } else { "0.00, 0.00, 0.00".to_string() };

    let mut user_count = 0;
    if let Ok(mut file) = fs::File::open("/run/utmp").or_else(|_| fs::File::open("/var/run/utmp")) {
        use std::io::Read;
        let mut buffer = [0u8; 384];
        while let Ok(n) = file.read(&mut buffer) {
            if n < 384 { break; }
            if i16::from_ne_bytes([buffer[0], buffer[1]]) == 7 { user_count += 1; }
        }
    }
    if user_count == 0 { user_count = 1; }

    println!(" up {},  {} user{},  load average: {}", format_duration(up_secs), user_count, if user_count == 1 { "" } else { "s" }, load_str);
}
