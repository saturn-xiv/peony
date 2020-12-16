#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(amd64, armhf) AND domain name'
    exit 1
fi

# -----------------------------

export DEBIAN_FRONTEND=noninteractive

export WORKSPACE=$PWD
export VERSION=$(git describe --tags --always --dirty)
export TARGET=$WORKSPACE/docker/debian

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

rm -rfv $TARGET/usr/bin/
mkdir -pv $TARGET/usr/bin/

if [ $1 = "armhf" ]
then
    sudo apt -y install libssl-dev:armhf \
        libsqlite3-dev:armhf libpq-dev:armhf libmysqlclient-dev:armhf \
        libudev-dev:armhf
    PKG_CONFIG_ALLOW_CROSS=1
    PKG_CONFIG_DIR=
    PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
    export PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR PKG_CONFIG_LIBDIR
    cargo build --target armv7-unknown-linux-gnueabihf --release
    cp -v target/armv7-unknown-linux-gnueabihf/release/peony $TARGET/usr/bin/
    arm-linux-gnueabihf-strip -s $TARGET/usr/bin/peony
elif [ $1 = "amd64" ]
then
    sudo apt -y install libssl-dev \
        libsqlite3-dev libpq-dev libmysqlclient-dev 
    cargo build --release
    cp -v target/release/peony $TARGET/usr/bin/
    strip -s $TARGET/usr/bin/peony
else
    echo "Unknown arch $1"
    exit 1
fi

# -----------------------------
if [ ! -d node_modules ]
then
    npm install
fi

cd $WORKSPACE/dashboard
if [ ! -d node_modules ]
then
    npm install
fi
REACT_GRPC_HOST=$2 npm run build

rm -rfv $TARGET/var
mkdir -pv $TARGET/var/lib/peony

cp -r $WORKSPACE/node_modules $TARGET/var/lib/peony/
cp -r $WORKSPACE/dashboard/build $TARGET/var/lib/peony/dashboard
# -----------------------------
rm -rfv $TARGET/etc
mkdir -pv $TARGET/etc/peony
cp $WORKSPACE/LICENSE $WORKSPACE/README.md $TARGET/etc/peony/

echo "$(git describe --tags --always --dirty --first-parent) $(date -R)" > $TARGET/etc/peony/VERSION
echo "$1 $(lsb_release -cs) $2" >> $TARGET/etc/peony/VERSION

cd $WORKSPACE
dpkg-buildpackage -us -uc -b --host-arch $1

echo 'done'

exit 0