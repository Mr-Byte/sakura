# About
Sakura is a language with a hybrid of Rust/Go-like syntax and pulling ideas from Kotlin that's intended to target WebAssembly using the GC proposal.

# License
Sakura is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

# Building

## macOS
### Setup
```
brew install cmake llvm
brew link llvm --force
cargo build
```

## Windows
### Setup
```
winget install cmake
winget install llvm
cargo build
```
