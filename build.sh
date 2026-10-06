#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

for tool in x86_64-w64-mingw32-gcc x86_64-w64-mingw32-dlltool; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf 'Missing %s. Install the MinGW-w64 x64 GCC and binutils packages.\n' "$tool" >&2
        exit 1
    fi
done

cargo build --locked --release --target x86_64-pc-windows-gnu
printf 'Built %s\n' "$PWD/target/x86_64-pc-windows-gnu/release/askwam.exe"
