#!/bin/bash

set -x -Cue

curl -o rustup.sh --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
sh rustup.sh -y
source "$HOME/.cargo/env"
yarn
yarn global add wasm-pack
yarn build
