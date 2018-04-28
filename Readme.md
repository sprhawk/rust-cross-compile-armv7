====

Notes
----

I'm trying to coss-compile rust app for Linux on ARMv7 ([RK3288](http://opensource.rock-chips.com/wiki_RK3288)) which is built with [Yocto](https://www.yoctoproject.org/)

Three "Hello world" samples:

1. [hello world](hello_console/) for console (print 'Hello World' in console)
2. [hello rocket](hello_rocket/) for [Rocket.rs](https://rocket.rs/)
3. [hello vulkan](hello_vulkan/) for [Vulkano](https://vulkano.rs/)

First you need to add target:

```
rustup target add armv7-unknown-linux-gnueabihf
```

Then setup .cargo/config inside project or globally (~/.cargo/config) as:

```
[target.armv7-unknown-linux-gnueabihf]
linker = "/home/hongbo/Developer/build-disk/rk-none/sysroots/x86_64-rksdk-linux/usr/bin/arm-rk-linux-gnueabi/arm-rk-linux-gnueabi-gcc"
rustflags = [
    "-C", """link-args=--sysroot=/home/hongbo/Developer/build-disk/rk-none/sysroots/cortexa17hf-neon-vfpv4-rk-linux-gnueabi
    -Wl,--dynamic-linker=/lib/ld-linux-armhf.so.3
    -march=armv7ve
    -mfpu=neon-vfpv4
    -mfloat-abi=hard
    -mcpu=cortex-a17
    -I/home/hongbo/Developer/build-disk/rk-none/sysroots/cortexa17hf-neon-vfpv4-rk-linux-gnueabi/usr/include""",
    ]

```

Need to use `--sysroot=` flag to set correct toolchain

On yocto-built system, the dynamic loader path is `/lib/ld-linux-armhf.so.3`, but gcc use `/lib/ld-linux.so.3` by default, so need use `-Wl,--dynamic-linker=/lib/ld-linux-armhf.so.3` to run properly [[link](https://stackoverflow.com/questions/847179/multiple-glibc-libraries-on-a-single-host)]

