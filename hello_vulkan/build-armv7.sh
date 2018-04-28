SDKROOT="/home/hongbo/Developer/build-disk/rk-none/sysroots"
SYSROOT="$SDKROOT/cortexa17hf-neon-vfpv4-rk-linux-gnueabi"

export CC_armv7_unknown_linux_gnueabihf="$SDKROOT/x86_64-rksdk-linux/usr/bin/arm-rk-linux/arm-rk-linux-gcc"
export CFLAGS_armv7_unknown_linux_gnueabihf="-mcpu=cortex-a17 -mfloat-abi=hard -mfpu=neon-vfpv4 -march=armv7ve --sysroot=$SYSROOT -I$SYSROOT/usr/include"
export CARGO_TARGET="armv7-unknown-linux-gnueabihf"
export CARGO_HOST="x86_64-unknown-linux-gnueabi"

rustup run nightly cargo build --features="fbdev" ---target armv7-unknown-linux-gnueabihf
