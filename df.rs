use std::env;
use std::ffi::CString;
use std::fs;
use std::process;

#[cfg(unix)]
#[repr(C)]
struct Statvfs {
    f_bsize: u64,
    f_frsize: u64,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    _padding: [u64; 11],
}

#[cfg(unix)]
extern "C" {
    fn statvfs(path: *const std::ffi::c_char, buf: *mut Statvfs) -> i32;
}

fn format_size(bytes: u64, human: bool) -> String {
    if !human { return (bytes / 1024).to_string(); }
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 { format!("{}", bytes) }
    else if size >= 10.0 { format!("{:.0}{}", size, UNITS[unit_idx]) }
    else { format!("{:.1}{}", size, UNITS[unit_idx]) }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut human = false;
    for arg in &args[1..] {
        if arg == "-h" || arg == "--human-readable" { human = true; }
        else if arg == "--help" {
            println!("Usage: df [OPTION]... [FILE]...\nShow information about the file system on which each FILE resides,\nor all file systems by default.\n\n  -h, --human-readable   print sizes in powers of 1024 (e.g., 1023M)\n      --help             display this help and exit");
            return;
        } else if arg.starts_with('-') {
            eprintln!("df: unrecognized option '{}'", arg);
            process::exit(1);
        }
    }

    let header_size = if human { "Size" } else { "1K-blocks" };
    println!("{:18} {:>10} {:>10} {:>10} {:>5} Mounted on", "Filesystem", header_size, "Used", "Avail", "Use%");

    let mounts = fs::read_to_string("/proc/mounts").or_else(|_| fs::read_to_string("/etc/mtab")).unwrap_or_default();
    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 { continue; }
        let (dev, mnt) = (parts[0], parts[1]);
        if dev.starts_with("none") || dev == "proc" || dev == "sysfs" || dev == "devtmpfs" { continue; }

        #[cfg(unix)]
        {
            if let Ok(c_path) = CString::new(mnt) {
                let mut stats: Statvfs = unsafe { std::mem::zeroed() };
                if unsafe { statvfs(c_path.as_ptr(), &mut stats) } == 0 {
                    if stats.f_blocks == 0 { continue; }
                    let total_bytes = stats.f_blocks * stats.f_frsize;
                    let avail_bytes = stats.f_bavail * stats.f_frsize;
                    let used_bytes = (stats.f_blocks.saturating_sub(stats.f_bfree)) * stats.f_frsize;
                    let use_pct = if stats.f_blocks > 0 {
                        ((stats.f_blocks.saturating_sub(stats.f_bfree)) as f64 / stats.f_blocks as f64 * 100.0).ceil() as u64
                    } else { 0 };

                    println!("{:18} {:>10} {:>10} {:>10} {:>4}% {}",
                        dev, format_size(total_bytes, human), format_size(used_bytes, human), format_size(avail_bytes, human), use_pct, mnt);
                }
            }
        }
    }
}
