Rust Cross Compile [Vulkan] app for ARMv7 (RK3288)
====

Notes
----

you need proper Mali Driver on the device, I used [malit76xr12p004rel0linux1fbdevtar.gz](https://developer.arm.com/products/software/mali-drivers/user-space).

Overwrite /usr/lib/libMali.so with it.

```
cd /usr/lib && ln -s libMali.so libvulkan.so.1
```

setup [.cargo/config](../Readme.md).

```
./build-armv7.sh
```
