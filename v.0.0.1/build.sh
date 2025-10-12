#!/bin/bash

echo "🐺 Building woflOS..."

# Build the kernel
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""
echo "🚀 Launching woflOS in QEMU..."
echo "   (Press Ctrl+A then X to quit QEMU)"
echo ""

# Run in QEMU - MUST be in WSL/Linux for this to work!
qemu-system-riscv64 \
    -machine virt \
    -cpu rv64 \
    -smp 1 \
    -m 128M \
    -nographic \
    -serial mon:stdio \
    -kernel target/riscv64gc-unknown-none-elf/release/woflos