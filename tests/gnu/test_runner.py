#!/usr/bin/env python3
"""
GNU Coreutils Test Suite Summary Reporter
"""
import sys
import re
from pathlib import Path

def main():
    repo_root = Path(__file__).resolve().parent.parent.parent
    log_file = repo_root / "tests" / "gnu" / "coreutils-src" / "tests" / "test-suite.log"

    print("==========================================")
    print(" GNU Coreutils Test Suite Results Summary ")
    print("==========================================")

    if not log_file.exists():
        print(f"[!] Test suite log file not found at: {log_file}")
        sys.exit(0)

    content = log_file.read_text(encoding="utf-8", errors="ignore")

    total = 0
    passed = 0
    failed = 0
    skipped = 0

    for line in content.splitlines():
        if line.startswith("# TOTAL:"):
            total = int(re.search(r'\d+', line).group())
        elif line.startswith("# PASS:"):
            passed = int(re.search(r'\d+', line).group())
        elif line.startswith("# FAIL:"):
            failed = int(re.search(r'\d+', line).group())
        elif line.startswith("# SKIP:"):
            skipped = int(re.search(r'\d+', line).group())
        elif line.startswith("# ERROR:"):
            failed += int(re.search(r'\d+', line).group())

    print(f"Total Tests : {total}")
    print(f"Passed      : {passed}")
    print(f"Failed      : {failed}")
    print(f"Skipped     : {skipped}")
    print("==========================================")

if __name__ == "__main__":
    main()
