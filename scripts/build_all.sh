#!/usr/bin/env bash
set -e

echo "Building Aegis OS v0.1 – the unbreakable fortress"

rustup target add aarch64-unknown-none

cargo build --release --target aarch64-unknown-none -p aegis-kernel

mkdir -p build
cp target/aarch64-unknown-none/release/libaegis_kernel.a build/kernel.a

echo "aegis-boot.img ready in ./build/"
echo "Flash with: sudo ./scripts/flash_rb6.sh build/aegis-boot.img"
