#!/bin/bash

BUILD_DIR=../target/release
INSTALL_DIR=/usr/local/lib/boson

for dir in $(find . -maxdepth 1 -mindepth 1 -type d -printf '%f\n')
do
    pushd $dir
        cargo build --release
        mv "$BUILD_DIR/lib${dir}.so" $INSTALL_DIR
    popd
done;

