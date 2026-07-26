use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn civil_from_days(n: i64) -> (i32, u32, u32) {
    let z = n + 719468;
    let era = (if z >= 0 { z } else { z - 146096 + 1 }) / 146096;
    let doe = (z - era * 146096) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year as i32, m, d)
}

fn weekday_str(days: i64, short: bool) -> &'static str {
    let w = ((days + 4) % 7 + 7) % 7;
    match (w, short) {
        (0, true) => "Sun", (0, false) => "Sunday",
        (1, true) => "Mon", (1, false) => "Monday",
        (2, true) => "Tue", (2, false) => "Tuesday",
        (3, true) => "Wed", (3, false) => "Wednesday",
        (4, true) => "Thu", (4, false) => "Thursday",
        (5, true) => "Fri", (5, false) => "Friday",
        _         => if short { "Sat" } else { "Saturday" },
    }
}

fn month_str(m: u32, short: bool) -> &'static str {
    match (m, short) {
        (1, true) => "Jan", (1, false) => "January",
        (2, true) => "Feb", (2, false) => "February",
        (3, true) => "Mar", (3, false) => "March",
        (4, true) => "Apr", (4, false) => "April",
        (5, true) => "May", (5, false) => "May",
        (6, true) => "Jun", (6, false) => "June",
        (7, true) => "Jul", (7, false) => "July",
        (8, true) => "Aug", (8, false) => "August",
        (9, true) => "Sep", (9, false) => "September",
        (10, true) => "Oct", (10, false) => "October",
        (11, true) => "Nov", (11, false) => "November",
        _         => if short { "Dec" } else { "December" },
    }
}

fn format_date(format: &str, secs: u64, days: i64, year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> String {
    let mut res = String::new();
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('Y') => res.push_str(&format!("{:04}", year)),
                Some('m') => res.push_str(&format!("{:02}", month)),
                Some('d') => res.push_str(&format!("{:02}", day)),
                Some('H') => res.push_str(&format!("{:02}", hour)),
                Some('M') => res.push_str(&format!("{:02}", min)),
                Some('S') => res.push_str(&format!("{:02}", sec)),
                Some('A') => res.push_str(weekday_str(days, false)),
                Some('a') => res.push_str(weekday_str(days, true)),
                Some('B') => res.push_str(month_str(month, false)),
                Some('b') => res.push_str(month_str(month, true)),
                Some('s') => res.push_str(&secs.to_string()),
                Some('%') => res.push('%'),
                Some(other) => { res.push('%'); res.push(other); }
                None => res.push('%'),
            }
        } else { res.push(c); }
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut format_str = "+%a %b %d %H:%M:%S UTC %Y".to_string();

    for arg in &args[1..] {
        if arg == "-u" || arg == "--utc" || arg == "--universal" {
            // Default calculation is UTC
        } else if arg == "--help" {
            println!("Usage: date [OPTION]... [+FORMAT]\nDisplay the current time in the given FORMAT.\n\n  -u, --utc, --universal   print or set Coordinated Universal Time (UTC)\n      --help               display this help and exit");
            return;
        } else if let Some(val) = arg.strip_prefix('+') {
            format_str = format!("+{}", val);
        } else if arg.starts_with('-') {
            eprintln!("date: unrecognized option '{}'", arg);
            process::exit(1);
        }
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days = (now / 86400) as i64;
    let rem = (now % 86400) as u32;
    let (year, month, day) = civil_from_days(days);
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;
    let sec = rem % 60;

    if let Some(fmt) = format_str.strip_prefix('+') {
        println!("{}", format_date(fmt, now, days, year, month, day, hour, min, sec));
    }
}
