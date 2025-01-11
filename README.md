# Minimal project to test hello-world app (contract) on NEAR Protocol

It is a good starting point for checking an existing Wasm file on NEAR Protocol.

## Prerequisites

Install Rust and Cargo: https://www.rust-lang.org/tools/install

## Basic Usage

1. Put the Wasm file you want to test into the root of this project and name it `contract.wasm`
2. Run `cargo test`

## Advanced Usage

If you want to test the contract with the latest version of NEAR Protocol, you can build the NEAR Protocol from the source code.

1. Clone nearcore from `debug/wasm-logs` branch: https://github.com/near/nearcore/compare/debug/wasm-logs
2. `make neard-debug`
3. Set the `NEAR_SANDBOX_BIN_PATH` env variable to a full path pointing to the `neard` in `./target/debug` folder
4. and run `cargo test`

Example one-liner:

```shell
env NEAR_SANDBOX_BIN_PATH=/home/frol/nearcore/target/debug/neard cargo test
```
