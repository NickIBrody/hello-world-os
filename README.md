# hello-world-os
Hello World OS

A simple educational OS written in ASM + Rust.
Upon boot, it displays: Hello World OS
Project files:

· boot.asm — bootloader (prints text)
· kernel/src/main.rs — minimal Rust kernel
· linker.ld — linker script for Rust

This OS only works in the QEMU emulator and is created for learning the basics of low-level programming.




# Hello World OS

Tiny OS (ASM bootloader + Rust kernel).  
Boots in QEMU and prints **"Hello World OS"**.

**Only works in QEMU!**

### Requirements
```bash
# Termux/Ubuntu
pkg/apt install nasm binutils qemu-system-x86 qemu-utils
rustup default nightly && rustup component add rust-src

# Quick Build & Run

git clone https://github.com/NickIBrody/hello-world-os
cd hello-world-os

# Create Cargo.toml (kernel/)
cd kernel && echo '[package]
name = "hello-kernel"
version = "0.1.0"
edition = "2021"

[profile.release]
panic = "abort"
opt-level = "z"
lto = true' > Cargo.toml && cd ..

# Custom target
echo '{
  "llvm-target": "x86_64-unknown-none",
  "os": "none",
  "executables": true,
  "panic-strategy": "abort"
}' > x86_64-bare.json

# Build kernel
cd kernel && cargo build --target ../x86_64-bare.json --release && cd ..

# Assemble & link
nasm -f bin boot.asm -o boot.bin
ld -T linker.ld -o kernel.bin target/x86_64-bare/release/libhello_kernel.a

cat boot.bin kernel.bin > os.bin

dd if=/dev/zero of=os.img bs=512 count=2880
dd if=os.bin of=os.img conv=notrunc

# Run!
qemu-system-x86_64 -fda os.img
