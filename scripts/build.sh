#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

TARGET=${1:-}

pushd frontend
CARGO_TARGET_DIR=../target-trunk trunk build --release
popd

if [[ -n "$TARGET" ]]; then
    cargo build --target "$TARGET" --bin server --release
else
    cargo build --bin server --release
fi

