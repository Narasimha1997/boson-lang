#!/bin/bash

# Provide pre-build commands here

mode=$2
arg=$1
fname=$3

# this is the location where cargo saves all the indexes..
# this path will be mounted to the container to ensure the same indexes are reused.
# CARGO_HOME env is injected into the container to make this path as cargo home directory.
CARGO_INDEX_PATH=$HOME/.boson_cargo_index

function check_install_docker () {
    if ! command -v docker &> /dev/null; then
        echo "Docker not found on the system, installing...."
        if ! command -v curl &> /dev/null; then
            sudo apt update && apt install curl
        fi

        # install docker:
        curl https://get.docker.com/ | sh
        echo "Docker installed."
    fi
}

function use_docker_build () {
    check_install_docker
    local_arg="no-install"

    if [[ "$arg" != "" ]]; then
        local_arg=$arg
    fi

    # mount the pwd inside docker and run build:
    docker run --rm -ti \
        -v $PWD:/np -v $CARGO_INDEX_PATH:/root/.cargo \
        --env "CARGO_HOME=/root/.cargo" \
        rust bash -c "cd /np && ./build.sh $local_arg" \

    echo "Binaries are generated, now installing them locally..."
    bash ./build.sh install
}


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

function build_embed () {
    echo "Building Boson Compiler..."
    cargo build --verbose --release --bin boson-embed
}

function build_modules () {
    pushd modules
        bash ./install.sh
    popd
}

function install() {
    echo "Installing binaries in system root /usr/local/bin..."
    sudo cp ./target/release/boson /usr/local/bin/
    sudo cp ./target/release/boson-eval /usr/local/bin/
    sudo cp ./target/release/boson-dis /usr/local/bin/
    sudo cp ./target/release/boson-compile /usr/local/bin/
    sudo cp ./target/release/boson-embed /usr/local/bin/
}

function run_tests () {
    cargo test
}

if [[ "$arg" == "docker" ]]; then
    arg=""
    mode="docker"
fi

if [[ "$mode" == "" ]]; then
    mode="local"
fi

if [[ "$mode" == "docker" ]]; then
    use_docker_build
    exit 0;
fi

if [[ "$arg" == "test" ]]; then
    run_tests
elif [[ "$arg" == "eval" ]]; then
    build_eval
elif [[ "$arg" == "repl" ]]; then 
    build_repl
elif [[ "$arg" == "dis" ]]; then
    build_dis
elif [[ "$arg" == "compile" ]]; then
    build_compile
elif [[ "$arg" == "embed" ]]; then
    build_embed
elif [[ $arg == "modules" ]]; then
    build_modules
elif [[ "$arg" == "install" ]]; then
    install
elif [[ "$arg" == "no-install" ]]; then
    echo "Building all the binaries..."
    build_eval
    build_compile
    build_repl
    build_dis
    build_embed
else
    echo "Building all the binaries..."
    build_eval
    build_compile
    build_repl
    build_dis
    build_embed
    build_modules
    install
    echo "All Done, you can start using boson, boson-compile, boson-eval and boson-dis"
fi
