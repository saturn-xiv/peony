#!/bin/sh

set -e

export WORKSPACE=$PWD
export VERSION=$(git describe --tags --always --dirty --first-parent)
export TARGET=$WORKSPACE/tmp/$1-$VERSION/target

if [ -d $TARGET ]
then
    rm -rf $TARGET
fi
mkdir -pv $TARGET

export PKG_CONFIG_ALL_STATIC=1

# https://doc.rust-lang.org/nightly/rustc/platform-support.html
if [ $1 = "armhf" ]
then
    # FIXME
    cargo build --release --target armv7-unknown-linux-musleabihf
    cp -av $WORKSPACE/target/armv7-unknown-linux-musleabihf/release/peony $TARGET/
    arm-linux-gnueabihf-strip -s $TARGET/peony
elif [ $1 = "amd64" ]
then
    cargo build --release --target x86_64-unknown-linux-musl
    cp -av $WORKSPACE/target/x86_64-unknown-linux-musl/release/peony $TARGET/usr/bin/
    strip -s $TARGET/peony
else
    echo "Unknown arch $1"
    exit 1
fi

# -----------------------------

if [ ! -d $WORKSPACE/node_modules ]
then
    cd $WORKSPACE
    yarn
fi

if [ ! -d $WORKSPACE/dashboard/node_modules ]
then
    cd $WORKSPACE/dashboard
    yarn
fi
cd $WORKSPACE/dashboard
yarn build
cp -r $WORKSPACE/node_modules $TARGET/
cp -r $WORKSPACE/dashboard/dist $TARGET/dashboard

# -----------------------------
cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/package.json $TARGET/

echo "$VERSION $(date -R) $1" > $TARGET/etc/peony/VERSION

echo 'done.'
exit 0