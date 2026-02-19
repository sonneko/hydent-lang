cargo build
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo miri setup || true
cargo miri test --lib || true
cargo test --verbose
