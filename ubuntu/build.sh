#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(amd64, armhf) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD
export VERSION=$(git describe --tags --always --dirty --first-parent)
export TARGET=$WORKSPACE/tmp/$(lsb_release -cs)-$VERSION/target
export CONAN_HOME=$HOME/.conan/data
# -----------------------------
if [ -d $TARGET ]
then
    rm -rf $TARGET
fi
mkdir -pv $TARGET/usr/bin
cp -r $WORKSPACE/ubuntu/debian $TARGET/


# FIXME static link
# export PQ_LIB_STATIC=1
export PKG_CONFIG_ALL_STATIC=1

# https://doc.rust-lang.org/nightly/rustc/platform-support.html
if [ $1 = "armhf" ]
then
    # sudo apt -y install libc6-dev-i386 g++-arm-linux-gnueabihf libc6-dev:armhf \
    #     libssl-dev:armhf libzmq3-dev:armhf \
    #     libpq-dev:armhf libmysqlclient-dev:armhf libsqlite3-dev:armhf
    PKG_CONFIG_ALLOW_CROSS=1
    PKG_CONFIG_DIR=
    PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
    export PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR PKG_CONFIG_LIBDIR    
    
    cargo build --target armv7-unknown-linux-gnueabihf --release
    # export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld
    # cargo build --target=armv7-unknown-linux-musleabihf --release
    
    cp -av $WORKSPACE/target/armv7-unknown-linux-gnueabihf/release/peony $TARGET/usr/bin/
    arm-linux-gnueabihf-strip -s $TARGET/usr/bin/peony
    # arm-linux-gnueabihf-readelf -a $TARGET/usr/bin/peony | grep "Shared library:"
    arm-linux-gnueabihf-objdump -x $TARGET/usr/bin/peony | grep "NEEDED"
    # fix in dpkg-architecture
    CC=arm-linux-gnueabihf-gcc
    CXX=arm-linux-gnueabihf-g++
    export CC CXX
elif [ $1 = "amd64" ]
then
    # sudo apt -y install libssl-dev \
    #     libzmq3-dev \
    #     libpq-dev libmysqlclient-dev libsqlite3-dev
    # export OPENSSL_DIR=
    export PQ_LIB_DIR=$CONAN_HOME/libpq/13.1/_/_/package/8e0939db49a1d312829524beb4d0b6824e47691d
    cargo build --target x86_64-unknown-linux-gnu --release
    
    # MUSL
    # cargo build --target x86_64-unknown-linux-musl --release

    cp -av $WORKSPACE/target/x86_64-unknown-linux-gnu/release/peony $TARGET/usr/bin/
    strip -s $TARGET/usr/bin/peony
    objdump -x $TARGET/usr/bin/peony | grep "NEEDED"
else
    echo "Unknown arch $1"
    exit 1
fi

# -----------------------------

cd $WORKSPACE
if [ ! -d node_modules ]
then
    npm install
fi

if [ ! -d dashboard/node_modules ]
then
    cd dashboard
    npm install
fi
cd $WORKSPACE/dashboard
# %REACT_APP_WEBSITE_NAME%
REACT_API_HOST=api.$2 REACT_GRPC_HOST=rpc.$2 npm run build

# -----------------------------
mkdir -pv $TARGET/usr/share/peony
cp -r $WORKSPACE/node_modules $TARGET/usr/share/peony/
cp -r $WORKSPACE/dashboard/build $TARGET/usr/share/peony/dashboard

rm -rf $TARGET/etc
mkdir -pv $TARGET/etc/peony
cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/package.json $TARGET/etc/peony/
echo "$VERSION $(date -R)" > $TARGET/etc/peony/VERSION
echo "$1 $(lsb_release -cs) $2" >> $TARGET/etc/peony/VERSION

rm -rf $TARGET/var
mkdir -pv $TARGET/var/lib/peony

cd $TARGET
dpkg-buildpackage -us -uc -b --host-arch $1

echo "Done($TARGET)."

exit 0