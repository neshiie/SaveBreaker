export RUSTFLAGS := "-C target-cpu=native"
export RUST_BACKTRACE := "1"

alias b := build
alias c := check
alias f := format

build:
    cargo build

format:
    cargo fmt

check:
    cargo fmt --check
    cargo clippy

test:
    cargo test

repl:
    cargo run -- repl
