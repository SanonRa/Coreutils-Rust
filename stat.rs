use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::UNIX_EPOCH;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: stat [OPTION]... FILE...\nDisplay file or file system status.\n\n      --help     display this help and exit");
            return;
        } else if arg.starts_with('-') && arg != "-" {
            eprintln!("stat: unrecognized option '{}'", arg);
            process::exit(1);
        } else {
            files.push(arg.clone());
        }
    }

    if files.is_empty() {
        eprintln!("stat: missing operand\nTry 'stat --help' for more information.");
        process::exit(1);
    }

    let mut exit_code = 0;
    for file in files {
        let path = Path::new(&file);
        match fs::symlink_metadata(path) {
            Ok(meta) => {
                let file_type = if meta.is_dir() { "directory" } else if meta.file_type().is_symlink() { "symbolic link" } else { "regular file" };
                
                #[cfg(unix)]
                {
                    println!("  File: {}", file);
                    println!("  Size: {:<10} \tBlocks: {:<10} IO Block: {:<6} {}", meta.size(), meta.blocks(), meta.blksize(), file_type);
                    println!("Device: {:x}h/{:<8} \tInode: {:<10} Links: {}", meta.dev(), meta.dev(), meta.ino(), meta.nlink());
                    println!("Access: ({:04o}/{:?})  Uid: ({:>5})   Gid: ({:>5})", meta.mode() & 0o7777, meta.permissions(), meta.uid(), meta.gid());
                    println!("Access: {}", meta.accessed().unwrap_or(UNIX_EPOCH).duration_since(UNIX_EPOCH).unwrap().as_secs());
                    println!("Modify: {}", meta.modified().unwrap_or(UNIX_EPOCH).duration_since(UNIX_EPOCH).unwrap().as_secs());
                    println!("Change: {}", meta.ctime());
                }
                #[cfg(not(unix))]
                {
                    println!("  File: {}", file);
                    println!("  Size: {:<10} \tType: {}", meta.len(), file_type);
                }
            }
            Err(e) => {
                eprintln!("stat: cannot statx '{}': {}", file, e);
                exit_code = 1;
            }
        }
    }
    process::exit(exit_code);
}
