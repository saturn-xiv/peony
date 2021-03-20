#!/bin/bash

set -e 

export WORKSPACE=$PWD

if [ -d $WORKSPACE/build ]
then
    rm -r $WORKSPACE/build
fi

mkdir -p $WORKSPACE/build/amd64-debug
cd $WORKSPACE/build/amd64-debug
conan install $WORKSPACE --build=missing --profile=$WORKSPACE/profiles/amd64
cmake $WORKSPACE
make 

mkdir -p $WORKSPACE/build/amd64-release
cd $WORKSPACE/build/amd64-release
conan install $WORKSPACE --build=missing --profile=$WORKSPACE/profiles/amd64
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release
make 

mkdir -p $WORKSPACE/build/armhf-debug
cd $WORKSPACE/build/armhf-debug
conan install $WORKSPACE --build=missing --profile=$WORKSPACE/profiles/armhf
cmake $WORKSPACE -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake
make 

mkdir -p $WORKSPACE/build/armhf-release
cd $WORKSPACE/build/armhf-release
conan install $WORKSPACE --build=missing --profile=$WORKSPACE/profiles/armhf
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake
make 

echo 'done.'

exit 0
