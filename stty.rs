use std::env;
use std::process;

#[cfg(unix)]
#[repr(C)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

#[cfg(unix)]
#[repr(C)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

#[cfg(unix)]
extern "C" {
    fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios) -> i32;
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "--help" {
        println!("Usage: stty [SETTING]...\nPrint or change terminal characteristics.\n\n  echo / -echo    enable / disable echoing of input characters\n  icanon / -icanon enable / disable erase, kill, werase, and rprnt special characters\n  sane            reset terminal to standard sane settings\n  size            print window size in rows and columns\n      --help      display this help and exit");
        return;
    }

    #[cfg(unix)]
    unsafe {
        let mut t: Termios = std::mem::zeroed();
        if tcgetattr(0, &mut t) != 0 {
            eprintln!("stty: 'standard input': Inappropriate ioctl for device");
            process::exit(1);
        }

        if args.len() == 1 {
            println!("speed 38400 baud; line = {};", t.c_line);
            let echo = if t.c_lflag & 0o000010 != 0 { "echo" } else { "-echo" };
            let icanon = if t.c_lflag & 0o000002 != 0 { "icanon" } else { "-icanon" };
            println!("{} {}", echo, icanon);
            return;
        }

        let mut modified = false;
        for arg in &args[1..] {
            match arg.as_str() {
                "size" => {
                    let mut ws: Winsize = std::mem::zeroed();
                    if ioctl(0, 0x5413, &mut ws) == 0 { println!("{} {}", ws.ws_row, ws.ws_col); }
                    else { println!("0 0"); }
                }
                "echo" => { t.c_lflag |= 0o000010; modified = true; }
                "-echo" => { t.c_lflag &= !0o000010; modified = true; }
                "icanon" => { t.c_lflag |= 0o000002; modified = true; }
                "-icanon" => { t.c_lflag &= !0o000002; modified = true; }
                "sane" => {
                    t.c_lflag |= 0o000010 | 0o000002 | 0o000001 | 0o000004; // ECHO | ICANON | ISIG | ECHOE
                    t.c_iflag |= 0o000400 | 0o002000; // ICRNL | IXON
                    t.c_oflag |= 0o000001 | 0o000004; // OPOST | ONLCR
                    modified = true;
                }
                _ => { eprintln!("stty: invalid argument '{}'", arg); process::exit(1); }
            }
        }

        if modified && tcsetattr(0, 0, &t) != 0 {
            eprintln!("stty: failed to apply settings");
            process::exit(1);
        }
    }
    #[cfg(not(unix))]
    {
        eprintln!("stty: terminal ioctl operations are not supported on Windows");
        process::exit(1);
    }
}
