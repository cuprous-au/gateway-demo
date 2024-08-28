gateway-demo
====

Demonstrates a project layout and the code required to run on the Cuprous Secured Edge Gateway running
Cuprux, our own build of Linux targetting edge-based applications with security in mind.

The project is split into a `backend` and `frontend`. The `backend` will serve the `frontend` project
as a web application. The `frontend` will then connect to the `backend` and receive events per the
gateway's tamper switch being opened/closed.

Releasing for Linux armv7 (The Cuprous Secured Edge Gateway with Cuprux)
---

Assuming a hardware floating point target (`hf`), install the toolchain (Linux):

```
apt-get install gcc-arm-linux-musleabihf
```

...or for macOS:

```
brew install arm-linux-gnueabihf-binutils 
```

...and then build:

```
cargo build --bin gateway-demo --release --target=armv7-unknown-linux-musleabihf
```
