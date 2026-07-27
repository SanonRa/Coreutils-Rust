#!/usr/bin/env python3
"""
GNU Coreutils Test Suite Runner & Reporter
"""
import os
import sys
import subprocess
import argparse
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(description="Run GNU Coreutils tests against Rust binaries")
    parser.add_argument("--utility", "-u", default="all", help="Target utility to test (e.g., cat, echo, ls, or 'all')")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    bin_dir = repo_root / "target" / "release"
    gnu_dir = repo_root / "tests" / "gnu" / "coreutils-src"

    print(f"[*] Coreutils Rust Root: {repo_root}")
    print(f"[*] Target Utility: {args.utility}")

    if not bin_dir.exists():
        print("[!] Release binaries not found. Building release target...")
        subprocess.run(["cargo", "build", "--release"], cwd=repo_root, check=True)

    print("[*] Coreutils test harness ready.")

if __name__ == "__main__":
    main()
