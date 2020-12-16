#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(x64, win, arm) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD

export BUILD_ROOT=$WORKSPACE/build/$2-$1
rm -rfv $BUILD_ROOT
mkdir -pv $BUILD_ROOT
cd $BUILD_ROOT

if [ $1 = "arm" ]
then
    cmake -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/linaro.cmake -DCMAKE_BUILD_TYPE=Release $WORKSPACE
elif [ $1 = "win" ]
then
    cmake -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/mingw.cmake -DCMAKE_BUILD_TYPE=Release $WORKSPACE 
elif [ $1 = "x64" ]
then    
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_BUILD_TYPE=Release $WORKSPACE
    strip -s $BUILD_ROOT/peony
else
    echo "Unknown arch $1"
    exit 1
fi

make peony

export TARGET=$WORKSPACE/docker/ubuntu
rm -rfv $TARGET/usr
mkdir -pv $TARGET/usr/bin 
cp -av $BUILD_ROOT/peony $TARGET/usr/bin/
mkdir -pv $TARGET/usr/share/peony

rm -rfv $TARGET/etc
mkdir -pv $TARGET/etc/peony
cp $WORKSPACE/LICENSE $WORKSPACE/README.md $TARGET/etc/peony/

rm -rfv $TARGET/var
mkdir -pv $TARGET/var/lib/peony

cd $WORKSPACE
# $(lsb_release -cs)-
dpkg -b ubuntu $2-$1-$(git describe --tags --always --dirty --first-parent).deb

echo 'done'

exit 0