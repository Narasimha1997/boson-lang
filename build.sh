#!/bin/bash

# Provide pre-build commands here

arg=$1
fname=$2

if [[ "$arg" == "prod" ]]; then
    cargo build --release
elif [[ "$arg" == "debug" ]]; then 
    cargo build
else
    cargo run --bin boson $fname
fi