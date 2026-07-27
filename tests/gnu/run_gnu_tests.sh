#!/usr/bin/env bash
# GNU Coreutils Official Test Harness Script

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GNU_TEST_DIR="${REPO_ROOT}/tests/gnu/coreutils-src"
BIN_DIR="${REPO_ROOT}/target/release"

echo "=================================================="
echo " GNU Coreutils Official Test Harness Setup "
echo "=================================================="

echo "[1/6] Building Rust Coreutils Binaries (Release Mode)..."
cd "${REPO_ROOT}"
cargo build --release

echo "[2/6] Preparing GNU Coreutils Source Repository..."
mkdir -p "${REPO_ROOT}/tests/gnu"
if [ ! -d "${GNU_TEST_DIR}" ]; then
  echo "Cloning official GNU Coreutils repository into tests/gnu/coreutils-src..."
  git clone --depth 1 https://github.com/coreutils/coreutils.git "${GNU_TEST_DIR}"
else
  echo "GNU Coreutils repository already cloned."
fi

cd "${GNU_TEST_DIR}"

echo "[3/6] Bootstrapping GNU Coreutils..."
if [ ! -f "configure" ]; then
  ./bootstrap
fi

echo "[4/6] Configuring GNU Coreutils..."
if [ ! -f "Makefile" ]; then
  ./configure FORCE_UNSAFE_CONFIGURE=1 --quiet
fi

echo "[5/6] Installing Compiled Rust Coreutils Binaries into Test Bin Dir..."
mkdir -p src
for bin in "${BIN_DIR}"/*; do
  if [ -f "$bin" ] && [ -x "$bin" ] && [[ "$bin" != *.* ]]; then
    bin_name=$(basename "$bin")
    cp -f "$bin" "src/${bin_name}"
    chmod +x "src/${bin_name}"
  fi
done

export PATH="${GNU_TEST_DIR}/src:$PATH"

echo "[6/6] Executing GNU Coreutils Test Suite..."
make -k check || true
