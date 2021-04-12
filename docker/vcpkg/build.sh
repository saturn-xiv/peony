#!/bin/sh

set -e

export WORKSPACE=$PWD

# if [ -d $WORKSPACE/build ]
# then
#     rm -rfv $WORKSPACE/build
# fi

mkdir -pv $WORKSPACE/build/x64-release
cd $WORKSPACE/build/x64-release
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release
time make -j

mkdir -pv $WORKSPACE/build/arm-release
cd $WORKSPACE/build/arm-release
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake
time make -j


echo 'done.'
exit 0
