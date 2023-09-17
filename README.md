# sadx-auto-splitter

An auto splitter for Sonic.

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
