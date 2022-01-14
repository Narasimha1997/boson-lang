# boson
An interpreted, dynamically-typed, multi-threaded, general purpose hobby programming language written in Rust.

## Features:
1. Multiple Data Types: char, int, float, string, array, hashtable, bytes and buffer.
2. Variables and Constants
3. Control and Looping structures
4. Functions and Lambda expressions
5. Threads and Multi-threading
6. Shell operator to run shell commands within the language statements
7. Some basic built-in functions
8. Iterators (psuedo iterators)
9. Byte code generation, serialization and loading
10. 

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

