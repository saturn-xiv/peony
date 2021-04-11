#!/bin/sh

set -e

export WORKSPACE=$PWD

if [ -d $WORKSPACE/build ]
then
    rm -rv $WORKSPACE/build
fi

mkdir -pv $WORKSPACE/build/amd64-release
cd $WORKSPACE/build/amd64-release
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release
time make -j

mkdir -pv $WORKSPACE/build/armhf-release
cd $WORKSPACE/build/armhf-release
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake
time make -j


echo 'done.'
exit 0
