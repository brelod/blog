#!/bin/bash

clean() {
    rm -rf target
}

build() {
    mkdir -p target
    rustc -g -O \
        -C force-frame-pointers=yes \
        -C force-unwind-tables=false \
        -C panic=abort \
        -C link-args='-fpic -pic' \
        --crate-type=lib lib.rs \
        -o target/liblinux.rlib
    rustc -g -O \
        -C force-frame-pointers=yes \
        -C force-unwind-tables=false \
        -C panic=abort \
        -C link-args='-nostartfiles -pie -Wl,--no-dynamic-linker' \
        -L target ./bin.rs -o target/bin
    #rustc -g  --crate-type=lib lib.rs -o target/liblinux.rlib
    #rustc -g  -C link-args='-nostartfiles -static' -L target ./bin.rs -o target/bin
}

run() {
    build
    ./target/bin
}

dump() {
    objdump --disassemble=$1 -M intel --visualize-jumps=extended-color target/bin
    #objdump --disassemble=$1 -M intel target/bin
}

case "$1" in
    clean) clean;;
    build) build;;
    run) run;;
    dump) dump $2;;
    *) echo "Invalid argument '$1'";;
esac
