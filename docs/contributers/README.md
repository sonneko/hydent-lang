## 

## compiler fases

* SURFACE
    - `cli`: receive user's input and send that into `compiler` and return the results
    - `compiler`: manage all compile process and resource with scheduling
* FRONTEND(using `CompilerFrontendContext`)
    - `tokenizer`: tokenize input and generate `Vec<Token>`
    - `parser`: parse `Vec<Token>` and generate `AST`
    - `dependency_resolution`: resolve file and package level devendency
    - `name_resolution`: resolve name and type
    - `type_checker`: check if type is correct
* MIDDLEEND(using `CompilerMiddleendContext`)
    - `hir_gen`: generate hir that is more low-level and brief lang from `AST`
    - `hir_transform`: optimize or transform hir
    - `mir_gen`: generate mir from hir
    - `mir_transform`: optimize or transform mir
* BACKEND(using `CompilerBackendContext`)
    - `llvmir_gen`: generate llvm-ir
    - `linker`: link the output binary to binaries from other programming language and export one binary
* OTHER
    - `diagnostic`: manage compiler diagnostic from any fases
    - `linter`: lint
    - `doc_gen`: generate documents from source code
    - `common`: utility for all fases

## How to build Compiler

### dependencies

* Rust stable toolchains (cargo, rustfmt, rustc, clippy)
* Rust nightly toolchains (miri)
* Node.js
* (in the future) LLVM_21 with libpolly-21-dev

### Build

1. Install script dependencies bia npm

```bash
cd script && npm install
```

2. Build compiler bia cargo

```bash
cargo build --release
```
