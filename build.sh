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

function build_compile () {
    echo "Building Boson Compiler..."
    cargo build --verbose --release --bin boson-compile
}

function install() {
    echo "Installing binaries in system root /usr/local/bin..."
    sudo cp ./target/release/boson /usr/local/bin/
    sudo cp ./target/release/boson-eval /usr/local/bin/
    sudo cp ./target/release/boson-dis /usr/local/bin/
}

if [[ "$arg" == "eval" ]]; then
    build_eval
elif [[ "$arg" == "repl" ]]; then 
    build_repl
elif [[ "$arg" == "dis" ]]; then
    build_dis
elif [[ "$arg" == "compile" ]]; then
    build_compile
elif [[ "$arg" == "install" ]]; then
    install
else
    echo "Building all the binaries..."
    build_eval
    build_compile
    build_repl
    build_dis
    install
    echo "All Done, you can start using boson, boson-eval and boson-dis"
fi
