# gateway-demo

Demonstrates a project layout and the code required to run on the Cuprous Secured Edge Gateway running
Cuprux, our own build of Linux targetting edge-based applications with security in mind.

The project is split into a `backend` and `frontend`. The `backend` will serve the `frontend` project
as a web application. The `frontend` will then connect to the `backend` and receive events per the
gateway's tamper switch being opened/closed.

## Releasing for Linux armv7 (The Cuprous Secured Edge Gateway with Cuprux)

Install the toolchain (Linux):

```
apt-get install gcc-arm-linux-musleabihf
```

...or for macOS:

```
brew install arm-linux-gnueabihf-binutils 
```

...and then build:

> If not already done, install the Rust target: `rustup target add armv7-unknown-linux-musleabihf`.

```
cargo build --bin gateway-demo --release --target=armv7-unknown-linux-musleabihf
```

The next step is to build the frontend in release form:

> If not already done, following the instructions for WASM here: https://yew.rs/docs/getting-started/introduction

```
(cd frontend && trunk build --release)
```

With all binaries built, we can now package:

```
sudo ./package-build-artefacts-linux.sh
```

This package will be built as a tar.gz file and the script will output its location. The archive
can be copied on to a gateway, decompressed and then its `install` script run.

## Contribution policy

Contributions via GitHub pull requests are gladly accepted from their original author. Along with any pull requests, please state that the contribution is your original work and that you license the work to the project under the project's open source license. Whether or not you state this explicitly, by submitting any copyrighted material via pull request, email, or other means you agree to license the material under the project's open source license and warrant that you have the legal authority to do so.

## License

This code is open source software licensed under the [Apache-2.0 license](./LICENSE).

© Copyright [Cuprous P/L](https://www.cuprous.com.au/), 2024
