# GNU Coreutils in Rust 🦀

A clean, high-performance, zero-dependency reimplementation of the **GNU Coreutils** suite in pure Rust, released under the **GNU General Public License v3.0 (GPL-v3)**.

---

## 🌟 Overview

This repository contains **109 complete, standalone Rust implementations** covering the full ecosystem of GNU Coreutils:

* **Zero External Dependencies:** Built strictly using Rust's standard library (`std`) and safe platform system call wrappers (`libc` FFI).
* **Standalone Executables:** Every utility compiles into its own self-contained binary (`cat`, `ls`, `cp`, `mv`, `sha256sum`, `find`, etc.).
* **Master Multiplexer:** Includes a BusyBox-style `coreutils` multiplexer binary that routes calls based on `argv[0]` or `--coreutils-prog=NAME`.

---

## 🛠️ Implemented Utilities (109 / 109 Complete)

| Category | Utilities |
| --- | --- |
| **File Operations** | `cp`, `mv`, `rm`, `ln`, `install`, `touch`, `mkdir`, `rmdir`, `mktemp`, `mkfifo`, `mknod`, `pathchk`, `chmod`, `chown`, `chgrp`, `df`, `du`, `stat`, `sync`, `link`, `unlink`, `truncate`, `shred` |
| **Text Processing** | `cat`, `tac`, `head`, `tail`, `wc`, `cut`, `paste`, `tr`, `expand`, `unexpand`, `nl`, `fmt`, `fold`, `sort`, `uniq`, `shuf`, `tsort`, `pr`, `csplit`, `join`, `comm`, `ptx` |
| **System & Inspection** | `ls`, `dir`, `vdir`, `dircolors`, `uname`, `arch`, `hostname`, `hostid`, `tty`, `stty`, `whoami`, `users`, `groups`, `id`, `who`, `pinky`, `uptime`, `nproc`, `logname`, `printenv`, `env`, `pwd`, `readlink`, `realpath`, `chroot`, `chcon`, `runcon`, `find` |
| **Encoders & Hashers** | `base64`, `base32`, `basenc`, `md5sum`, `sha1sum`, `sha224sum`, `sha256sum`, `sha384sum`, `sha512sum`, `b2sum`, `cksum`, `sum`, `od`, `dd`, `numfmt` |
| **Control & Logic** | `sleep`, `true`, `false`, `yes`, `echo`, `printf`, `seq`, `factor`, `expr`, `test` / `[`, `timeout`, `nice`, `nohup`, `tee`, `stdbuf`, `kill`, `coreutils` |

---

## ⚙️ Building

Compile all 109 binaries in release mode using Cargo:

```bash
cargo build --release
```

All compiled binaries will be located in:
```text
target/release/
```

### Running via the Coreutils Multiplexer

You can also run any utility using the single `coreutils` multiplexer:

```bash
target/release/coreutils cat sample.txt
target/release/coreutils --coreutils-prog=ls -la
```

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](LICENSE) or [COPYING](COPYING) file for details.
