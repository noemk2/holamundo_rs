#!/bin/bash
set -e

if [[ -d ./res ]]
then
    #echo "/etc exists on your filesystem."
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/*.wasm ./res/

else
	mkdir ./res
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/*.wasm ./res/
fi

#mkdir ./res ||  RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
#cp target/wasm32-unknown-unknown/release/*.wasm ./res/
