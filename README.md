This is a project template for Rust WebAssembly programs which don't use any
JavaScript modules.

Prerequisites:
- Wasm toolchain: `rustup target add wasm32-unknown-unknown`
- [wasm-bindgen]: `cargo install wasm-bindgen-cli`
- [cargo-generate]: `cargo install cargo-generate`
- [wasm-server-runner]: `cargo install wasm-server-runner`

[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[wasm-server-runner]: https://github.com/jakobhellermann/wasm-server-runner

## Steps

1. Generate project

`cargo generate https://github.com/DougLau/simple-wasm-template --name {project name}`

2. Run project

`cargo run`
