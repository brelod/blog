#!/bin/bash

pids=()
trap teardown INT
teardown() {
    for pid in "${pids[@]}"; do
        echo "Stop $pid"
        kill -9 "$pid"
    done
    exit 0
}

mkdir -p html

python3 -m http.server --directory html --bind 0.0.0.0 8001 &
pids+=($!)

while $(inotifywait -r -e modify,move,create,delete --exclude code/target code &>/dev/null); do
    rustdoc code/lib.rs -o html/code
done &
pids+=($!)

while $(inotifywait -r -e modify,move,create,delete ./book &>/dev/null); do
    mdbook build -d html/book
done &
pids+=($!)

while true; do
    sleep 1
done
