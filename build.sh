#!/bin/bash
set -e


RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

if [ ! -d ./res ]; then
    mkdir ./res
fi

cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasm_file ./res/*.wasm
