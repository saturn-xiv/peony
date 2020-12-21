#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(amd64, armhf) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD

export BUILD_ROOT=$WORKSPACE/build/$1
# rm -rf $BUILD_ROOT
mkdir -pv $BUILD_ROOT
cd $BUILD_ROOT

# -----------------------------
conan install $WORKSPACE --profile=$WORKSPACE/conan/profiles/$1 --build=missing

if [ $1 = "armhf" ]
then
    sudo apt -y install libglfw3-dev:armhf
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake -DCMAKE_BUILD_TYPE=Release $WORKSPACE
    # dpkg-architecture: warning: specified GNU system type arm-linux-gnueabihf
    CC=arm-linux-gnueabihf-gcc
    CXX=arm-linux-gnueabihf-g++
    export CC CXX
elif [ $1 = "amd64" ]
then
    sudo apt -y install libglfw3-dev
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_BUILD_TYPE=Release $WORKSPACE
else
    echo "Unknown arch $1"
    exit 1
fi

make peony
make edelweiss

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

export TARGET=$WORKSPACE/ubuntu
rm -rf $TARGET/usr
mkdir -pv $TARGET/usr/bin 
cp -av $BUILD_ROOT/bin/peony $BUILD_ROOT/bin/edelweiss $TARGET/usr/bin/
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