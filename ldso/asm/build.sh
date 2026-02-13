#!/bin/bash
set -e
set -u

nasm -f elf64 elf.s -o bin/elf.o
nasm -f elf64 mem.s -o bin/mem.o
nasm -f elf64 env.s -o bin/env.o
nasm -f elf64 main.s -o bin/main.o

ld \
    -pie \
    -x \
    --no-dynamic-linker \
    --gc-sections \
    bin/*.o -o exe

