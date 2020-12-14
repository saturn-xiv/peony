#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(armv6, x86_64) AND domain name'
    exit 1
fi

# -----------------------------

export WORKSPACE=$PWD
export VERSION=$(git describe --tags --always --dirty)

# -----------------------------

# https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg
# export RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-static"
# https://github.com/sfackler/rust-openssl/issues/604
# OPENSSL_INCLUDE_DIR=/usr/lib
# OPENSSL_LIB_DIR=/usr/include
# OPENSSL_STATIC=1
# export OPENSSL_INCLUDE_DIR OPENSSL_LIB_DIR OPENSSL_STATIC
# export PQ_LIB_STATIC=1
# export SQLITE3_STATIC=1
export PKG_CONFIG_ALL_STATIC=1

rm -rf ubuntu/usr/bin/
mkdir -pv ubuntu/usr/bin/

if [ $1 = "armv7" ]
then
    sudo apt -y install libssl-dev:armhf libudev-dev:armhf \
        libsqlite3-dev:armhf libpq-dev:armhf libmysqlclient-dev:armhf
    PKG_CONFIG_ALLOW_CROSS=1
    PKG_CONFIG_DIR=
    PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
    export PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR PKG_CONFIG_LIBDIR
    cargo build --target armv7-unknown-linux-gnueabihf --release
    cp -v target/armv7-unknown-linux-gnueabihf/release/peony ubuntu/usr/bin/
    arm-linux-gnueabihf-strip -s ubuntu/usr/bin/peony
elif [ $1 = "x86_64" ]
then
    sudo apt -y install libssl-dev \
        libsqlite3-dev libpq-dev libmysqlclient-dev 
    cargo build --release
    cp -v target/release/peony ubuntu/usr/bin/
    strip -s ubuntu/usr/bin/peony
else
    echo "Unknown arch $1"
    exit 1
fi

# -----------------------------
if [ ! -d node_modules ]
then
    npm install
fi
# -----------------------------
cd $WORKSPACE/dashboard
if [ ! -d node_modules ]
then
    npm install
fi
REACT_GRPC_HOST=$2 npm run build
# -----------------------------

export TARGET=$WORKSPACE/ubuntu
rm -rfv $TARGET/usr
mkdir -pv $TARGET/usr/bin 
cp -av bin/ashoka $TARGET/usr/bin/
strip -s $TARGET/usr/bin/ashoka
mkdir -pv $TARGET/usr/share/ashoka
cp -av $WORKSPACE/db $TARGET/usr/share/ashoka/

rm -rfv $TARGET/etc
mkdir -pv $TARGET/etc/ashoka
cp $WORKSPACE/LICENSE $WORKSPACE/config.toml $TARGET/etc/ashoka/

cd $WORKSPACE
dpkg -b ubuntu $2-$1-$(lsb_release -cs)-$(git describe --tags --always --dirty --first-parent).deb

echo 'done'

exit 0