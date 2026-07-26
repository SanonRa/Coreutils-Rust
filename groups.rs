use std::env;
use std::fs;
use std::process;

fn get_user_groups(username: &str) -> Vec<String> {
    let mut groups = Vec::new();
    let mut primary_gid = None;

    if let Ok(passwd) = fs::read_to_string("/etc/passwd") {
        for line in passwd.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 && parts[0] == username {
                primary_gid = Some(parts[3].to_string());
                break;
            }
        }
    }

    if let Ok(group_file) = fs::read_to_string("/etc/group") {
        for line in group_file.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                let gname = parts[0];
                let gid = parts[2];
                let members: Vec<&str> = parts[3].split(',').collect();

                if Some(gid.to_string()) == primary_gid || members.contains(&username) {
                    if !groups.contains(&gname.to_string()) {
                        groups.push(gname.to_string());
                    }
                }
            }
        }
    }
    groups
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut targets = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: groups [OPTION]... [USERNAME]...\nPrint group memberships for each USERNAME or, if no USERNAME is specified, for\nthe current process (which may differ if the groups database has changed).\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("groups: unrecognized option '{}'\nTry 'groups --help' for more information.", arg);
            process::exit(1);
        } else {
            targets.push(arg.clone());
        }
    }

    if targets.is_empty() {
        let user = env::var("USER").or_else(|_| env::var("LOGNAME")).unwrap_or_else(|_| {
            eprintln!("groups: cannot find current user");
            process::exit(1);
        });
        let user_groups = get_user_groups(&user);
        println!("{} : {}", user, user_groups.join(" "));
    } else {
        for user in targets {
            let user_groups = get_user_groups(&user);
            if user_groups.is_empty() {
                eprintln!("groups: '{}': no such user", user);
            } else {
                println!("{} : {}", user, user_groups.join(" "));
            }
        }
    }
}
