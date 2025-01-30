default:
    @just --list

check:
    cargo check --quiet --workspace --all-targets
    cargo check --quiet --workspace --all-features --target wasm32-unknown-unknown
    cargo fmt --all -- --check
    cargo clippy --quiet --workspace --all-targets --all-features -- -D warnings -W clippy::all
    cargo test --quiet --workspace --all-targets --all-features
    cargo test --quiet --workspace --doc
    
build: check
    trunk build
