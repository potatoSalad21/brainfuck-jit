# BF-JIT

A brainfuck interpreter and x86_64 JIT compiler written in rust. Optimizations include
run-length encoding during the preprocessing step and pre-calculating jump addresses via backpatching technique in a single pass.

## Usage

JIT (default)
```sh
cargo run --release -- test.bf
```

Interpreter
```sh
cargo run --release -- test.bf --interpret
```

## Requirements

- x86_64 linux (or other system V AMD64 ABI target). JIT emits raw x86_64 machine code
and makes direct `mmap` and `mprotect` calls, so it won't work on other architectures or on windows.
