#!/bin/bash

set -x -Cue

ls
yarn
npm install -g wasm-pack
which wasm-pack
cd crates
strace wasm-pack build --target web --release gl2d
