Rust Cross Compile [Rocket.rs](https://rocket.rs/) app for ARMv7 (RK3288)
====

Notes
----

```
# Rocket requires features in nightly built Rust
rustup target add --toolchain nightly armv7-unknown-linux-gnueabihf
```

setup [.cargo/config](../Readme.md).

```
rustup run nightly cargo build --target armv7-unknown-linux-gnueabihf
```