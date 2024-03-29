# sadx-auto-splitter

A port of the original ASL auto splitter for Sonic Adventure DX.

## Credits

This auto splitter reuses code from a previous codebase.
All of the original contributors to the auto splitter have agreed for this project to take on some of their code.

**The original contributors:**

[TurtleMan64](https://github.com/TurtleMan64/)  
[Ngolinvaux](https://github.com/Ngolinvaux)  
[IDGeek121](https://github.com/IDGeek121)  
[Sora](https://github.com/Sora-yx)  
[skoobmaster](https://github.com/skewbmaster/)

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-wasi --toolchain stable
```

The auto splitter can now be compiled:
```sh
cargo b
```

The auto splitter is then available at:
```
target/wasm32-wasi/release/sadx_auto_splitter.wasm
```

Make sure too look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

You can use the [debugger](https://github.com/CryZe/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory and more.
