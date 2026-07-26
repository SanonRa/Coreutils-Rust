use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

struct Prng { state: u64 }
impl Prng {
    fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(12345) ^ (process::id() as u64);
        Self { state: seed }
    }
    fn next_u8(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 33) as u8
    }
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        for b in buf.iter_mut() { *b = self.next_u8(); }
    }
}

fn shred_file(path: &Path, iterations: usize, zero: bool, remove: bool, verbose: bool) -> io::Result<()> {
    let meta = fs::metadata(path)?;
    let size = meta.len();
    let mut file = OpenOptions::new().write(true).open(path)?;
    let mut prng = Prng::new();
    let mut buffer = vec![0u8; 65536];

    for i in 1..=iterations {
        if verbose { println!("shred: {}: pass {}/{} (random)...", path.display(), i, iterations); }
        let mut remaining = size;
        file.set_len(size)?;
        use std::io::Seek;
        file.seek(io::SeekFrom::Start(0))?;
        while remaining > 0 {
            let chunk = remaining.min(buffer.len() as u64) as usize;
            prng.fill_bytes(&mut buffer[..chunk]);
            file.write_all(&buffer[..chunk])?;
            remaining -= chunk as u64;
        }
        file.sync_all()?;
    }

    if zero {
        if verbose { println!("shred: {}: pass {}/{} (000000)...", path.display(), iterations + 1, iterations + 1); }
        buffer.fill(0);
        let mut remaining = size;
        use std::io::Seek;
        file.seek(io::SeekFrom::Start(0))?;
        while remaining > 0 {
            let chunk = remaining.min(buffer.len() as u64) as usize;
            file.write_all(&buffer[..chunk])?;
            remaining -= chunk as u64;
        }
        file.sync_all()?;
    }

    if remove {
        if verbose { println!("shred: {}: removing", path.display()); }
        fs::remove_file(path)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iterations = 3;
    let mut zero = false;
    let mut remove = false;
    let mut verbose = false;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-z" | "--zero" => zero = true,
            "-u" | "--remove" => remove = true,
            "-v" | "--verbose" => verbose = true,
            "-uz" | "-zu" => { zero = true; remove = true; }
            "-vuz" | "-vzu" | "-uvz" | "-zuv" => { zero = true; remove = true; verbose = true; }
            "-n" | "--iterations" => {
                if i + 1 >= args.len() { eprintln!("shred: option requires an argument -- '{}'", arg); process::exit(1); }
                iterations = args[i + 1].parse().unwrap_or(3);
                i += 1;
            }
            "--help" => {
                println!("Usage: shred [OPTION]... FILE...\nOverwrite the specified FILE(s) repeatedly, in order to make it harder\nfor even very expensive hardware probing to recover the data.\n\n  -n, --iterations=N  overwrite N times instead of the default (3)\n  -u, --remove        truncate and remove file after overwriting\n  -v, --verbose       show progress\n  -z, --zero          add a final overwrite with zeros to hide shredding\n      --help          display this help and exit");
                return;
            }
            _ if arg.starts_with("-n") => {
                let val = arg.strip_prefix("-n").unwrap();
                iterations = val.parse().unwrap_or(3);
            }
            _ if arg.starts_with('-') => { eprintln!("shred: unrecognized option '{}'", arg); process::exit(1); }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() { eprintln!("shred: missing file operand\nTry 'shred --help' for more information."); process::exit(1); }

    let mut exit_code = 0;
    for file in files {
        if let Err(e) = shred_file(Path::new(&file), iterations, zero, remove, verbose) {
            eprintln!("shred: {}: {}", file, e);
            exit_code = 1;
        }
    }
    process::exit(exit_code);
}
