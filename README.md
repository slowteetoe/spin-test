# Hello world :: Spin

Spin is a way to deploy serverless WebAssembly (WASM) apps

The purpose of this repo is to try out [Fermyon Spin](https://www.fermyon.com/spin) while building a simple Rust app.

Since wasm/wasi/wati are all evolving standards, there are some complications, but theoretically any language that compiles to WASM could be used.

## Installation

### Install Spin

```sh
curl https://developer.fermyon.com/downloads/install.sh | bash
```

### Build and serve app locally

Export an ENV variable for your mysql connection

```sh
export SPIN_CONFIG_DB_URL="your conn url"
```

Start up the app in the same terminal

```sh
spin build --up
```

And we'll see:

```text
Logging component stdio to ".spin/logs/"

Serving http://127.0.0.1:3000
Available Routes:
  hello-rust: http://127.0.0.1:3000 (wildcard)
```

## Issues

* Spin doesn't support TLS for mysql connection? But Planetscale requires TLS...  still looking into this one
* If you try to connect to a mysql url with the parameter `tls`, you'll see 5(?!) errors per request

    ```text
    thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: UnknownParameter { param: "tls" }', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/mysql_async-0.30.0/src/opts.rs:843:41
    ```

## Interesting references

* [https://www.thorsten-hans.com/master-configuration-data-in-fermyon-spin/]
