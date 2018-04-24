SDKROOT="/home/hongbo/Developer/build-disk/rk-none/sysroots"
SYSROOT="$SDKROOT/cortexa17hf-neon-vfpv4-rk-linux-gnueabi"

export CC="$SDKROOT/x86_64-rksdk-linux/usr/bin/arm-rk-linux/arm-rk-linux-gcc"
export CFLAGS="-mfloat-abi=hard --sysroot=$SYSROOT -I$SYSROOT/usr/include"

rustup run nightly cargo build  --target=armv7-unknown-linux-gnueabihf
