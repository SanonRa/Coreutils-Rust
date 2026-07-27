#!/usr/bin/env bash
# GNU Coreutils Test Harness Script

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GNU_TEST_DIR="${REPO_ROOT}/tests/gnu/coreutils-src"
BIN_DIR="${REPO_ROOT}/target/release"

echo "=========================================="
echo " GNU Coreutils Test Harness for Rust Implementation "
echo "=========================================="

echo "[1/4] Building Rust Coreutils Binaries (Release Mode)..."
cd "${REPO_ROOT}"
cargo build --release

echo "[2/4] Preparing Binary Symlinks..."
mkdir -p "${REPO_ROOT}/tests/gnu/bin"
for bin in "${BIN_DIR}"/*; do
  if [ -f "$bin" ] && [ -x "$bin" ] && [[ "$bin" != *.* ]]; then
    bin_name=$(basename "$bin")
    ln -sf "$bin" "${REPO_ROOT}/tests/gnu/bin/${bin_name}"
  fi
done

export PATH="${REPO_ROOT}/tests/gnu/bin:$PATH"

echo "[3/4] Checking GNU Coreutils Source Repository..."
if [ ! -d "${GNU_TEST_DIR}" ]; then
  echo "Cloning official GNU Coreutils repository into tests/gnu/coreutils-src..."
  git clone --depth 1 https://github.com/coreutils/coreutils.git "${GNU_TEST_DIR}"
else
  echo "GNU Coreutils test directory already present."
fi

echo "[4/4] Environment ready for testing."
echo "Binaries available in PATH:"
ls -la "${REPO_ROOT}/tests/gnu/bin/"
