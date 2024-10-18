#!/bin/bash

echo "Installing rust"
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -y
echo "Rust installed successfully"
source $HOME/.cargo/env
echo "Rust version"
rustc --version
echo "Cargo build"
cargo build
