#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(amd64, armhf) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD
export TARGET=$WORKSPACE/ubuntu

# -----------------------------
rm -rf $TARGET/usr
mkdir -pv $TARGET/usr/bin 

# https://github.com/rust-lang/rust/issues/79206
export RUSTFLAGS='-C target-feature=+crt-static'
# https://doc.rust-lang.org/nightly/rustc/platform-support.html
if [ $1 = "armhf" ]
then
    PKG_CONFIG_ALLOW_CROSS=1
    PKG_CONFIG_DIR=
    PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
    export PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR PKG_CONFIG_LIBDIR
    cargo build --target armv7-unknown-linux-gnueabihf --release
    cp -av $BUILD_ROOT/target/armv7-unknown-linux-gnueabihf/release/peony $TARGET/usr/bin/
    arm-linux-gnueabihf-strip -s $TARGET/usr/bin/peony
elif [ $1 = "amd64" ]
then
    cargo build --target x86_64-unknown-linux-gnu --release
    cp -av $BUILD_ROOT/target/x86_64-unknown-linux-gnu/release/peony $TARGET/usr/bin/
    strip -s $TARGET/usr/bin/peony
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
REACT_GRPC_HOST=$2 npm run build

# -----------------------------
mkdir -pv $TARGET/usr/share/peony
cp -r $WORKSPACE/node_modules $WORKSPACE/package.json $TARGET/usr/share/peony/
cp -r $WORKSPACE/dashboard/build $TARGET/usr/share/peony/dashboard

rm -rf $TARGET/etc
mkdir -pv $TARGET/etc/peony
cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/etc/* $TARGET/etc/peony/
echo "$(git describe --tags --always --dirty --first-parent) $(date -R)" > $TARGET/etc/peony/VERSION
echo "$1 $(lsb_release -cs) $2" >> $TARGET/etc/peony/VERSION

rm -rf $TARGET/var
mkdir -pv $TARGET/var/lib/peony

cd $TARGET
dpkg-buildpackage -us -uc -b --host-arch $1

echo 'done'

exit 0