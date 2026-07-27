# GNU Coreutils Testing Framework

This directory (`tests/gnu/`) is configured to host, fetch, and execute the official GNU Coreutils test suite against our Rust implementations.

## Usage

### Local Execution (Linux / WSL / macOS)

Run the test suite setup script:
```bash
./tests/gnu/run_gnu_tests.sh
```

### GitHub Actions Online Workflow

This repository includes an online GitHub Actions workflow configured in `.github/workflows/gnu-tests.yml`.

You can run it manually online from GitHub:
1. Go to the **Actions** tab on GitHub: `https://github.com/SanonRa/Coreutils-Rust/actions`
2. Select **GNU Coreutils Test Suite** from the left sidebar.
3. Click **Run workflow**.

## Structure
- `tests/gnu/run_gnu_tests.sh`: Helper script to compile Rust binaries, clone the GNU Coreutils suite, and execute test cases.
- `tests/gnu/test_runner.py`: Python harness for running and reporting individual utility test results.
