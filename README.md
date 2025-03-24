# `hyperlight-wasm` sockets example

This is a minimal example of a
[Hyperlight-Wasm](https://github.com/hyperlight-dev/hyperlight-wasm)
host application. It implements just enough of the `wasi:sockets` UDP
api (in an unfortunately blocking fashion) to run the [echo sample
server](https://github.com/hyperlight-dev/wasm-udp-echo-sample).

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install), including the `x86_64-unknown-none` target (which may be installed via e.g. `rustup target add x86_64-unknown-none`)
2. A C compiler
3. [wasm-tools](https://github.com/bytecodealliance/wasm-tools)
4. If you are fetching the sample binary from an OCI registry,
   [wkg](https://crates.io/crates/wkg/0.10.0).

## Building

Compile the WIT and set the environment variables used when building
(both the host and the guest):

```sh
wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm
export HYPERLIGHT_WASM_WORLD=$PWD/hyperlight-world.wasm
```

Build:
```
cargo build
```

## Running

Get an `echo.wasm` from [the sample
repo](https://github.com/hyperlight-dev/wasm-udp-echo-sample), either
by building it or by fetching it from the OCI registry (`wkg oci pull
ghcr.io/hyperlight-dev/wasm-udp-echo-sample/udp-echo-server:latest -o echo.wasm`).

AOT compile it:

```sh
cargo install --git https://github.com/hyperlight-dev/hyperlight-wasm hyperlight-wasm-aot
hyperlight-wasm-aot compile --component echo.wasm echo.bin
```

You can then run the server:

```sh
cargo run # or target/debug/echo
```

This will not produce any output on stdout, and should wait forever to
receive UDP packets on `127.0.0.1:8080`.

## Testing

In another shell:

```sh
nc -u 127.0.0.1 8080
```

When using interactively, due to line-buffering, you will likely need
to send an entire line to `nc` before receiving a response.
