#!/bin/bash

# Provide pre-build commands here

arg=$1
fname=$2

function build_eval () {
    echo "Building Boson Evaluator..."
    cargo build --verbose --release --bin boson-eval
}

function build_repl () {
    echo "Building Boson REPL console..."
    cargo build --verbose --release --bin boson --features=repl --manifest-path=boson/Cargo.toml
}

function build_dis () {
    echo "Building Boson Disassembler..."
    cargo build --verbose --release --bin boson-dis
}

if [[ "$arg" == "eval" ]]; then
    build_eval
elif [[ "$arg" == "repl" ]]; then 
    build_repl
elif [[ "$arg" == "dis" ]]; then
    build_dis
else
    echo "Building all the binaries..."
    build_eval
    build_repl
    build_dis
fi
