This is a project template for Rust WebAssembly programs which don't use any
JavaScript modules.

Prerequisites:
- Wasm toolchain: `rustup target add wasm32-unknown-unknown`
- wasm-bindgen: `cargo install wasm-bindgen-cli`
- cargo-generate: `cargo install cargo-generate`

Optional:
- basic-http-server: `cargo install basic-http-server`

## Steps

1. Generate project

`cargo generate https://github.com/DougLau/simple-wasm-template --name {project name}`

2. Build project

In new project directory:
`./build.sh`

3. Serve for testing

Run web server:
`basic-http-server`
