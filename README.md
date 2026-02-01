# Hello World OS

Minimal OS: Assembly bootloader + Rust kernel (no_std).  
Boots in QEMU and prints **"Hello World OS"** in VGA text mode.

**Runs only in QEMU emulator.**

## Files

- `boot.asm`      — BIOS bootloader (prints message fallback)  
- `linker.ld`     — linker script  
- `kernel/`  
  - `Cargo.toml`  — Rust crate config  
  - `src/main.rs` — kernel entry point (_start)

## Requirements

- nasm  
- binutils (ld)  
- qemu-system-x86_64  
- Rust nightly:  
  ```bash
  rustup default nightly
  rustup component add rust-src

  # Install on Termux/Ubuntu:
  pkg/apt install nasm binutils qemu-system-x86 qemu-utils

#  Build & Run

git clone https://github.com/NickIBrody/hello-world-os
cd hello-world-os

# Build kernel
cd kernel
cargo build --release
cd ..

# Assemble bootloader
nasm -f bin boot.asm -o boot.bin

# Link (check exact .a name with ls target/release/)
ld -n -T linker.ld -o kernel.bin \
    target/release/libhello_kernel.a

# Combine
cat boot.bin kernel.bin > os.bin

# Create 1.44 MB floppy image
dd if=/dev/zero of=os.img bs=512 count=2880 status=none
dd if=os.bin of=os.img conv=notrunc status=none

# Launch
qemu-system-x86_64 -fda os.img

Expected: QEMU window with black background and white "Hello World OS" text.


# Troubleshooting
Cargo complains about target  add .cargo/config.toml in root:
[build]
target = "x86_64-unknown-none"



Wrong .a file name run ls target/release/ and fix the ld command
No output kernel may not be loaded; try -d int in qemu for debug
Ready for upgrades: Makefile, panic handler, colored text, multiboot.
