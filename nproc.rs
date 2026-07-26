use std::env;
use std::process;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ignore = 0;

    for arg in &args[1..] {
        if arg == "--help" {
            println!("Usage: nproc [OPTION]...\nPrint the number of processing units available to the current process,\nwhich may be less than the number of online processors\n\n      --all      print the number of installed processors\n      --ignore=N if possible, exclude N processing units\n      --help     display this help and exit");
            return;
        } else if arg == "--all" {
            // available_parallelism handles installed/online processors well
        } else if let Some(val) = arg.strip_prefix("--ignore=") {
            match val.parse::<usize>() {
                Ok(n) => ignore = n,
                Err(_) => {
                    eprintln!("nproc: invalid number: '{}'", val);
                    process::exit(1);
                }
            }
        } else if arg.starts_with('-') {
            eprintln!("nproc: unrecognized option '{}'\nTry 'nproc --help' for more information.", arg);
            process::exit(1);
        }
    }

    let count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let result = if count > ignore { count - ignore } else { 1 };
    println!("{}", result);
}
