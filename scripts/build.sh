#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

TARGET=${1:-}

pushd frontend
CARGO_TARGET_DIR=../target-trunk trunk build --release
popd

WASM_FILES=(dist/*.wasm)
if [ ${#WASM_FILES[@]} -eq 0 ]; then
    echo "Warning: No WASM files found in dist/. Skipping optimization."
else
    for WASM_FILE in "${WASM_FILES[@]}"; do
        echo "Optimizing WASM file: $WASM_FILE"
        wasm-opt -Oz -o "${WASM_FILE}.opt" "$WASM_FILE"
        mv "${WASM_FILE}.opt" "$WASM_FILE"
        echo "Optimization complete for $WASM_FILE"
    done
fi

if [[ -n "$TARGET" ]]; then
    cargo build --target "$TARGET" --bin server --release
else
    cargo build --bin server --release
fi

