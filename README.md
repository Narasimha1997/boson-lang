# boson
An interpreted, dynamically-typed, multi-threaded, general purpose hobby programming language written in Rust.

## Features:
1. Multiple Data Types: char, int, float, string, array, hashtable, bytes and buffer
2. Airthmetic, Logical operations
3. Variables and Constants
4. Control and Looping structures
5. Functions and Lambda expressions
6. Threads and Multi-threading
7. Shell operator to run shell commands within the language statements
8. Some basic built-in functions
9. Iterators (psuedo iterators)
10. Byte code generation, serialization and loading

## Installation:
Building the language from source requires a working rust toolchain installed on the host machine. Check out the tutorial [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) to set-up Rust and Cargo.

1. Grab the source code:
```
git clone git@github.com:Narasimha1997/np-lang.git
```
2. Build boson:
```
./build.sh
```
The build script should install Boson suite of tools on your system. Explore different options supported by this script.

3. Run sample source code
```
boson-eval examples/hello.np
```
This should print `hello, world!` on screen.

## Language documentation:
