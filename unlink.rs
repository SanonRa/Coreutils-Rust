use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--help" {
        println!("Usage: unlink FILE\n  or:  unlink OPTION\nCall the unlink function to remove the specified FILE.\n\n      --help     display this help and exit");
        return;
    }

    if args.len() != 2 || args[1].starts_with('-') {
        eprintln!("unlink: missing operand\nTry 'unlink --help' for more information.");
        process::exit(1);
    }

    if let Err(e) = fs::remove_file(&args[1]) {
        eprintln!("unlink: cannot unlink '{}': {}", args[1], e);
        process::exit(1);
    }
}
