cd script
npm run self-test
npm start -- ci
npm run compiler-test
cd ../
cargo build
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo miri setup || true
cargo miri test --lib || true
cargo test

