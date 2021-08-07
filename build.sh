#!/bin/bash

# Provide pre-build commands here

arg=$1
fname=$2

if [[ "$arg" == "eval" ]]; then
    cargo build --release --bin boson-eval
elif [[ "$arg" == "repl" ]]; then 
    cargo build --release --bin boson --features=repl --manifest-path=boson/Cargo.toml
else
    cargo run --bin boson $fname
fi