#!/bin/sh

set -e

export WORKSPACE=$PWD

if [ -d $WORKSPACE/build ]
then
    rm -rv $WORKSPACE/build
fi

mkdir -pv $WORKSPACE/build/amd64-release
cd $WORKSPACE/build/amd64-release
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release -DCMAKE_TOOLCHAIN_FILE=$HOME/local/vcpkg/scripts/buildsystems/vcpkg.cmake
make -j

# mkdir -p $WORKSPACE/build/armhf-release
# cd $WORKSPACE/build/armhf-release
# cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release

echo 'done.'
exit 0
