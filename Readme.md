====

Notes
----

I'm trying to coss-compile rust app for Linux on ARMv7 ([RK3288](http://opensource.rock-chips.com/wiki_RK3288)) which is built with [Yocto](https://www.yoctoproject.org/)

Two "Hello world" samples:

1. [hello world](hello_console/Readme.md) for console (print 'Hello World' in console)
2. [hello world](hello_rocket/Readme.md) for [Rocket.rs](https://rocket.rs/)

setup .cargo/config inside project or globally (~/.cargo/config) as:

```
[target.armv7-unknown-linux-gnueabihf]
linker = "/home/hongbo/Developer/build-disk/rk-none/sysroots/x86_64-rksdk-linux/usr/bin/arm-rk-linux-gnueabi/arm-rk-linux-gnueabi-gcc"
rustflags = [
    "-C", """link-args=--sysroot=/home/hongbo/Developer/build-disk/rk-none/sysroots/cortexa17hf-neon-vfpv4-rk-linux-gnueabi
    -Wl,--dynamic-linker=/lib/ld-linux-armhf.so.3
    -I/home/hongbo/Developer/build-disk/rk-none/sysroots/cortexa17hf-neon-vfpv4-rk-linux-gnueabi/usr/include""",
    ]

```

Need to use `--sysroot=` flag to set correct toolchain

On yocto-built system, the dynamic loader path is `/lib/ld-linux-armhf.so.3`, but gcc use `/lib/ld-linux.so.3` by default, so need use `-Wl,--dynamic-linker=/lib/ld-linux-armhf.so.3` to run properly [[link](https://stackoverflow.com/questions/847179/multiple-glibc-libraries-on-a-single-host)]

