#!/bin/bash
set -e

if [[ -d ./res ]]
then
    #echo "/etc exists on your filesystem."
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/*.wasm ./res/
	near dev-deploy --wasm_file ./res/*.wasm
	source ./neardev/dev-account.env
else
	mkdir ./res
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/*.wasm ./res/
	near dev-deploy --wasm_file ./res/*.wasm
	source ./neardev/dev-account.env
fi

#mkdir ./res ||  RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
#cp target/wasm32-unknown-unknown/release/*.wasm ./res/
