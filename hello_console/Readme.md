Rust Cross Compile app for ARMv7 (RK3288)
====

Notes
----

Need to use `--sysroot=` flag to set correct toolchain

On yocto-built system, the dynamic loader path is `/lib/ld-linux-armhf.so.3`, but gcc use default `/lib/ld-linux.so.3`, so need use `-Wl,--dynamic-linker=/lib/ld-linux-armhf.so.3` to run properly [[link](https://stackoverflow.com/questions/847179/multiple-glibc-libraries-on-a-single-host)]


