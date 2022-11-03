# boson
An interpreted, dynamically-typed, multi-threaded, general purpose hobby programming language written in Rust.

## Features:
1. Multiple Data Types: char, int, float, string, array, hashtable, bytes and buffer
2. Airthmetic, Logical operations
3. Variables and Constants
4. Control and Looping structures
5. Functions and Lambda expressions
6. Many built-in functions
7. Threads and Multi-threading
8. Shell operator to run shell commands within the language statements
9. Some basic built-in functions
10. Iterators (psuedo iterators)
11. Byte code generation, serialization and loading
12. Dynamic modules - can load rust modules at runtime as addons
13. System calls

Note: The documentation of this project is still in early stages.

## Installation:
Building the language from source requires a working rust toolchain installed on the host machine. Check out the tutorial [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) to set-up Rust and Cargo.

1. Grab the source code:
```
git clone git@github.com:Narasimha1997/boson-lang.git
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

### Using the tools
If compilation is successful, it should generate four binary tools, these are:

1. boson: This is the REPL of boson lang, you can execute boson language statements in the CLI.
```
Welcome to Boson REPL
This is a REPL binary for boson - a general purpose programming language written in rust. (Ctrl + C to quit)
Boson v0.0.1
VM Check - Passed.
>> println(10 + 20)
30
```

2. boson-dis: This tool generates stringified representation of the compiled version of source file.
```
boson-dis examples/hello.np
```

This should generate the output:
```
Instructions: 
00000000 IConstant 0
00000003 ILoadBuiltIn 2
00000006 ICall 1

Constants: 
00000000 hello, world!
```

3. boson-compile: This tool generates the compiled bytecode of the source file, which can then be executed.
```
boson-eval ./examples/hello.np
```
This should generates  a file called `hello.np.b` in the same folder `hello.np` was present, i.e `examples/hello.np.b`. This file has the binary representation of the compiled bytecode.

4. boson-eval: Evaluates the source file or the bytecode file and stdouts the result.
```
boson-eval ./examples/hello.np
```

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

6. Arrays
```python
var array = [1, 2, 3, 4, "Hello", 6.455]
println(array[0] + 2) # 3
println(array[4] + ", world") # Hello, world
println(array) # Array([1, 2, 3, 4, Hello, 6.455])

array[4] = 9678967;
println(array) # Array([1, 2, 3, 4, 9678967, 6.455])
```

7. Hash tables
```python
var myHashMap = {
    "name": "Prasanna",
    "age": 24,
    "country": "India"
}

println(myHashMap) # HashTable({age: 24, country: India, name: Prasanna})
println(myHashMap["age"] + 2) # 26

const key = "name"
println("Hey! " + myHashMap[key]) # Hey! Prasanna

myHashMap["city"] = "Bengaluru"
println(myHashMap["city"]) # Bengaluru
```

8. While loop
```python
const N = 100;

var n0 = 0;
var n1 = 1;
var n2 = 0;
var idx = 2;

while (idx <= N ) {
    n2 = n0 + n1;
    n0 = n1;
    n1 = n2;
    idx = idx + 1;
}

println(n1);
```

9. If else:

```python

const x = 10;

if (x > 20) {
    println("X > 20");
} else {
    println("X < 20"); # this will be executed
}

```

10. Functions:

```python
func fib(N) {
    
    if (N == 0) {
        return 0;
    }

    if (N == 1) {
        return 1;
    }

    return fib(N - 1) + fib(N - 2);
}

const result = fib(10);
println('got result: ', result);
```

11. Shell operator:

Shell operator can be used to execute shell commands within the program statements.
```python

# count the number of files in the given directory
func count_files() {
    const res = $ "ls | wc -l";
    return int(res[1]);
}

# call the function and print it's output
println(count_files());

# count the number of occurences of a given pattern in the given file
func count_occurences(file, pattern) {
    const res = $ "cat "+file+" | grep -c "+pattern; 
    return int(res[1])
}

const res = count_occurences("LICENSE", "GPL")
println(res);
```

12. Lambda functions:

```python
# define a adder that takes two parameters
const lambda_adder = lambda x, y => x + y
println(lambda_adder(10, 20)) # 30
```

13. Functions as objects:

```python
# here the adder function accepts a function as argument and executes it
func adder_exec(fn, x, y) {
    return fn(x, y)
}

# adder function is defined here
func adder(x, y) {
    return x + y
}

# adder function is passed as the parameter
const result = adder_exec(adder, 10, 20)
println(result) # 30
```

14. Closures:

```python

# this is the wrapper function that returns a adder function with enclosed local variables
func wrap_adder(x, y) {
    const z = 30
    func adder() {
        return x + y + z
    }

    return adder
}

# call the wrapper and obtain the inner child
const adder = wrap_adder(10, 20)

# call the inner child adder function
const result = adder()
println(result) # result = 60
```

15. Iterators:

**Note**: Iterators are yet to be tested completely
```
const arr = [1, 2, 3, 4]
const iterator = iter(arr)
while (has_next(iterator)) {
    println(next(iterator))
}
```

16. Multithreading:
```python
# this function prints Hello, world from <thread> every 3 seconds 
func print_periodic(name) {
    while (true) {
        println("Hello, world from ", name);
        sleep_sec(3);
    }
}

# spawn thread1 and spawn thread2
const th1 = thread print_periodic("thread1");
const th2 = thread print_periodic("thread2");

# wait for thread 1 and thread 2 to complete
wait(th1)
wait(th2)
```

**Threads and global variables**: In boson, every thread gets it's own copy of global variables space, so when a thread mutates a global variable, it mutates it's local variable copy and not the one in global space.

### Native modules
Boson has a support for native modules written in Rust, every module has to implement a set of functions as per the standard function signatures shown below:
```rust
pub type OpenFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type CloseFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type ReadFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type WriteFunctionSymbol = unsafe extern "Rust" fn(Rc<Object>) -> DynamicModuleResult;
pub type ExecFunctionSymbol = unsafe extern "Rust" fn(String, &Vec<Rc<Object>>) -> DynamicModuleResult;
```

Any cargo crate that implements these functions can be loaded and used as a dynamic module. Look at the [hello-world module](https://github.com/Narasimha1997/boson-lang/blob/main/modules/hello_world/src/lib.rs) for reference. Make sure you add the following lines to `Cargo.toml` of the cargo crate you are trying to build as a module:
```
[lib]
crate-type = ["rlib", "cdylib"]
```

The native modules that are under `modules/` directory will be installed at `/usr/local/lib/boson` and can be loaded using `mopen`:
```
# you can pass init parameters also while calling `mopen`
const regex = mopen("std::re", none)[0];
```
This will look for `libre.so` in `/usr/local/lib/boson`, in general it will look at `/usr/local/bin/boson/lib{{module-name}}.so`. However you can also load modules directly by their file-paths like:

```
const regex = mopen("/usr/local/lib/boson/libre.so", none)[0];
```

### Embedding Boson:
Boson language compiler + VM can be easily integrated into other projects using the API. As of now, any Rust codebase can import statically the Boson crate or use foreign function interface (FFI) to load Boson shared library by manually defining the ABI. We are yet to test [CXXABI](https://github.com/gcc-mirror/gcc/blob/master/libstdc%2B%2B-v3/libsupc%2B%2B/cxxabi.h) compatibility of the boson crate, so it can be considered unsafe to import boson in Non-Rust codebases as of now. 

Here is how you can embed boson in other rust projects:
```rust
extern crate boson;

use boson::api::BosonLang;

pub fn main() {
    let result = BosonLang::eval_buffer(
        "10 + 3 / 2"
            .as_bytes()
            .to_vec()
    );

    println!("eval result: {}", result);
    // will output "eval result: Some(Float(11.5))"
}
```


### Running tests
You can use cargo test tools to run the test
```
cargo test
```

### Benchmarks
Benchmarks are still under implementation

### Credits
1. [Monkey lang](https://monkeylang.org/)
2. [Monkey lang rust version](https://github.com/wadackel/rs-monkey-lang)

### TODO:
1. Web assembly port
2. Proper documentation
3. Proper test cases
4. Bug fixes

### Contributing
Feel free to raise any issues, make Pull Requests, suggest changes, clone the project and make your own changes.