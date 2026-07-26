use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn apply_mode(path: &Path, mode_str: &str) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        if let Ok(octal) = u32::from_str_radix(mode_str, 8) {
            let perms = fs::Permissions::from_mode(octal);
            fs::set_permissions(path, perms)?;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dir_mode = false;
    let mut verbose = false;
    let mut mode = "755".to_string();
    let mut paths = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-d" | "--directory" => dir_mode = true,
            "-v" | "--verbose" => verbose = true,
            "-m" | "--mode" => {
                if i + 1 >= args.len() {
                    eprintln!("install: option requires an argument -- '{}'", arg);
                    process::exit(1);
                }
                mode = args[i + 1].clone();
                i += 1;
            }
            "--help" => {
                println!("Usage: install [OPTION]... SOURCE... DIRECTORY\n  or:  install -d [OPTION]... DIRECTORY...\nCopy files and set attributes, or create directories.\n\n  -d, --directory   treat all arguments as directory names; create all\n                    components of the specified directories\n  -m, --mode=MODE   set permission mode (as in chmod), instead of 0755\n  -v, --verbose     print the name of each directory created or file copied\n      --help        display this help and exit");
                return;
            }
            _ if arg.starts_with("-m") => mode = arg.strip_prefix("-m").unwrap().to_string(),
            _ if arg.starts_with('-') => {
                eprintln!("install: unrecognized option '{}'", arg);
                process::exit(1);
            }
            _ => paths.push(PathBuf::from(arg)),
        }
        i += 1;
    }

    if paths.is_empty() {
        eprintln!("install: missing file operand\nTry 'install --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    if dir_mode {
        for p in paths {
            if let Err(e) = fs::create_dir_all(&p) {
                eprintln!("install: cannot create directory '{}': {}", p.display(), e);
                exit_code = 1;
            } else {
                if verbose { println!("install: created directory '{}'", p.display()); }
                let _ = apply_mode(&p, &mode);
            }
        }
    } else {
        if paths.len() < 2 {
            eprintln!("install: missing destination file operand after '{}'", paths[0].display());
            process::exit(1);
        }
        let dest = paths.pop().unwrap();
        for src in paths {
            let target = if dest.is_dir() { dest.join(src.file_name().unwrap_or_default()) } else { dest.clone() };
            if let Err(e) = fs::copy(&src, &target) {
                eprintln!("install: cannot copy '{}' to '{}': {}", src.display(), target.display(), e);
                exit_code = 1;
            } else {
                if verbose { println!("'{}' -> '{}'", src.display(), target.display()); }
                let _ = apply_mode(&target, &mode);
            }
        }
    }
    process::exit(exit_code);
}
