#!/bin/bash

spinner_pid=
function start_spinner {
    set +m
    echo -n "$1 "
    { while : ; do for X in '⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏' ; do echo -en "\b$X" ; sleep 0.1 ; done ; done & } 2>/dev/null
    spinner_pid=$!
}
function stop_spinner {
    { kill -9 $spinner_pid && wait; } 2>/dev/null
    set -m
    echo -en "\033[2K\r"
}
trap stop_spinner EXIT

echo Clear previous build
rm -rf docs/wasm docs/assets

echo Build rust wasm binary
cargo build --profile wasm-release --target wasm32-unknown-unknown

echo Bind wasm
start_spinner
wasm-bindgen --out-name tappy-plane --out-dir docs/wasm --target web target/wasm32-unknown-unknown/wasm-release/tappy-plane.wasm
stop_spinner

echo Optimise wasm
start_spinner
wasm-opt -Oz --output optimized.wasm docs/wasm/tappy-plane_bg.wasm
stop_spinner

echo Store optimised wasm
mv optimized.wasm docs/wasm/tappy-plane_bg.wasm

echo Link assets
cp -R ./assets ./docs/assets

echo Tidy generated files
start_spinner
rm docs/wasm/*.ts
uglifyjs docs/wasm/tappy-plane.js -c -m -o docs/wasm/tappy-plane.js --module
stop_spinner
