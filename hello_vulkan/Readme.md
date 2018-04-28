Rust Cross Compile [Vulkan] app for ARMv7 (RK3288)
====

Notes
----


setup [.cargo/config](../Readme.md).

```
./build-armv7.sh
```

You need to `cd /usr/lib && ln -s libMali.so libvulkan.so.1`
