export RUSTFLAGS := "-C target-cpu=native"
export RUST_BACKTRACE := "1"

alias b := build
alias c := check
alias f := format

build:
    cargo build

format:
    cargo fmt
    cargo clippy --all-targets --all-features --fix

check:
    cargo fmt --check
    cargo clippy --all-targets --all-features

test:
    cargo test

repl:
    cargo run -- repl
