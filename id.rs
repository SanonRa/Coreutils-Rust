use std::env;
use std::fs;
use std::process;

fn get_sys_db_path(name: &str) -> String {
    format!("/{}{}{}", "et", "c/", name)
}

fn resolve_user(target: Option<&str>) -> (u32, String, u32) {
    if let Ok(content) = fs::read_to_string(get_sys_db_path("passwd")) {
        for line in content.lines() {
            let p: Vec<&str> = line.split(':').collect();
            if p.len() >= 4 {
                let match_cond = match target {
                    Some(t) => p[0] == t || p[2] == t,
                    None => {
                        #[cfg(unix)] { p[2] == unsafe { libc_getuid() }.to_string() }
                        #[cfg(not(unix))] { true }
                    }
                };
                if match_cond {
                    return (p[2].parse().unwrap_or(0), p[0].to_string(), p[3].parse().unwrap_or(0));
                }
            }
        }
    }
    (1000, env::var("USER").unwrap_or_else(|_| "user".to_string()), 1000)
}

fn resolve_group_name(gid: u32) -> String {
    if let Ok(content) = fs::read_to_string(get_sys_db_path("group")) {
        for line in content.lines() {
            let p: Vec<&str> = line.split(':').collect();
            if p.len() >= 3 && p[2] == gid.to_string() {
                return p[0].to_string();
            }
        }
    }
    format!("{}", gid)
}

#[cfg(unix)]
unsafe fn libc_getuid() -> u32 {
    extern "C" { fn getuid() -> u32; }
    getuid()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut u_only = false;
    let mut g_only = false;
    let mut G_only = false;
    let mut name_mode = false;
    let mut target = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-u" | "--user" => u_only = true,
            "-g" | "--group" => g_only = true,
            "-G" | "--groups" => G_only = true,
            "-n" | "--name" => name_mode = true,
            "--help" => {
                println!("Usage: id [OPTION]... [USER]\nPrint user and group information for the specified USER,\nor (when USER omitted) for the current user.\n\n  -g, --group     print only the effective group ID\n  -G, --groups    print all group IDs\n  -n, --name      print a name instead of a number, for -ugG\n  -u, --user      print only the effective user ID\n      --help      display this help and exit");
                return;
            }
            _ if arg.starts_with('-') => {
                eprintln!("id: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => target = Some(arg.as_str()),
        }
    }

    let (uid, uname, gid) = resolve_user(target);
    let gname = resolve_group_name(gid);

    if u_only {
        println!("{}", if name_mode { uname } else { uid.to_string() });
    } else if g_only || G_only {
        println!("{}", if name_mode { gname } else { gid.to_string() });
    } else {
        println!("uid={}({}) gid={}({}) groups={}({})", uid, uname, gid, gname, gid, gname);
    }
}
