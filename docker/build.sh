#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(amd64, armhf) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD

export BUILD_ROOT=$WORKSPACE/build/$2-$1
rm -rf $BUILD_ROOT
mkdir -pv $BUILD_ROOT
cd $BUILD_ROOT

conan install $WORKSPACE --profile=$WORKSPACE/docker/conan/profiles/$1 --build=missing
if [ $1 = "arm" ]
then
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake -DCMAKE_BUILD_TYPE=Release $WORKSPACE
elif [ $1 = "x64" ]
then
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_BUILD_TYPE=Release $WORKSPACE
else
    echo "Unknown arch $1"
    exit 1
fi

make peony

export TARGET=$WORKSPACE/docker/ubuntu
rm -rf $TARGET/usr
mkdir -pv $TARGET/usr/bin 
cp -av $BUILD_ROOT/bin/peony $TARGET/usr/bin/
mkdir -pv $TARGET/usr/share/peony

rm -rf $TARGET/etc
mkdir -pv $TARGET/etc/peony
cp $WORKSPACE/LICENSE $WORKSPACE/README.md $TARGET/etc/peony/

rm -rf $TARGET/var
mkdir -pv $TARGET/var/lib/peony
# TODO dashboard

cd $WORKSPACE
# $(lsb_release -cs)-
dpkg -b docker/ubuntu $2-$1-$(git describe --tags --always --dirty --first-parent).deb

echo 'done'

exit 0