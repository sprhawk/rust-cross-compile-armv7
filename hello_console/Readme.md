Rust Cross Compile app for ARMv7 (RK3288)
====

Notes
----

```
rustup target add armv7-unknown-linux-gnueabihf
```

setup [.cargo/config](../Readme.md).

```
cd hello_console
cargo build --target armv7-unknown-linux-gnueabihf
```
