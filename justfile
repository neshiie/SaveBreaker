export RUSTFLAGS := "-C target-cpu=native"
export RUST_BACKTRACE := "1"

alias b := build

build:
    cargo build

test:
    cargo test

repl:
    cargo run -- repl
