# stark-rust
zk-STARK for fibonacci sequence in Rust

## How it works
This library is based on libSTARK developed by [StarkWare Industries Ltd.](https://www.starkware.co/)
On top of it, [this c++ wrapper](https://github.com/LayerXcom/libSTARK/tree/libstark-rs/fsrs)
is implemented for simple FFI with this Rust library.
These make it possible to call functions of STARK execution from Rust.

More information about zk-STARK and libSTARK is in [here](https://github.com/elibensasson/libSTARK).

## Setup on macOS
Before building, you may need to retrieve the source code of the wrapper implementation.
```
$ git submodule update --init --recursive
```
Install dependencies for compilation 
```
$ brew install libomp gcc
```
Install Rust
```
$ curl https://sh.rustup.rs -sSf | sh 
$ source ~/.cargo/env
```

## How to run the code
Arguments format:
```
$ cargo run <A: initial number> <B: initial number>
```

 for exmaple:
 ```
 $ cargo run 52 9
 ```
 The above execution results in execution of STARK simulation over the fibonacchi sequence.The statement is "I know secret numbers A, B such that the 5th element of the Fibonacci sequence is 131. They are A=52, B=9"

## References
1. Scalable, transparent, and post-quantum secure computational integrity. <https://eprint.iacr.org/2018/046.pdf>
