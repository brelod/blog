#!/bin/bash
set -e

# ==============================================================================
# ld.so
# ==============================================================================
nasm -f elf64 crt.s -o target/crt.o

rustc -g -O \
    -C panic=abort \
    -C force-frame-pointers=yes \
    -C force-unwind-tables=false \
    -C link-args='-nostartfiles -pie -Wl,--no-dynamic-linker target/crt.o' \
    -o target/ld.so \
    bin.rs

# ==============================================================================
# time
# ==============================================================================
nasm -f elf64 time.s -o target/time.o
ld -pie --dynamic-linker ld.so -o target/time target/time.o -L./target -lvdso
