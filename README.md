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

## Language examples:
1. Hello, world
```python
println('hello,world')
```

2. Keyboard input and display
```python
const ip = input()
const ip2 = input();
const greeting = "Hello! " + ip2 + " " + ip;
println(greeting);
```

3. Arithmetic operators
```python
const d = a + b + c;
const e = a * b - c;
const f = ((a + b) * c * d) / (a + b);

const g = (a + b) % c;

println(a, b, c, d, e, f, g); # 1 2 3 6 -1 18 0
```

4. Bitwise operators
```python
const x = 10;
const y = 20;

var z = ((x & 0) | y);
println(~z) # -21
```

5. Logical operators
```python
const m = 10;
const n = 20;

println(m > n, n < m, n > m + 5) # false, false, true
println(m == n - 10, !0, !(m == n - 10)) # true true false
```

6. 