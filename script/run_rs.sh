#!/bin/bash
set -xue

QEMU=qemu-system-riscv32

# Rust 커널 빌드 (manual implementation)
cargo build --release --features=manual

# 생성된 바이너리를 kernel.elf로 복사
cp ../target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines kernel.elf

# QEMU 실행
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -kernel kernel.elf
