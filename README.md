# Hello world :: Spin

Spin is a way to deploy serverless WebAssembly (WASM) apps

The purpose of this repo is to try out [Fermyon Spin](https://www.fermyon.com/spin) while building a simple Rust app.

Since wasm/wasi/wati are all evolving standards, there are some complications, but theoretically any language that compiles to WASM could be used.

## Installation

### Install Spin

curl https://developer.fermyon.com/downloads/install.sh | bash

### Build and serve app locally

```sh
spin up
```

And we'll see:

```text
Logging component stdio to ".spin/logs/"

Serving http://127.0.0.1:3000
Available Routes:
  hello-rust: http://127.0.0.1:3000 (wildcard)
```
