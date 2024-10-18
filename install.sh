echo "Installing rust"
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -y
echo "Rust installed successfully"
rustc --version
echo "Rust version"
echo "Cargo build"
cargo build
