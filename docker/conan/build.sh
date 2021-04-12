#!/bin/sh

set -e 

export WORKSPACE=$PWD
export CONAN_HOME=$WORKSPACE/docker/conan

if [ -d $WORKSPACE/build ]
then
    rm -r $WORKSPACE/build
fi

mkdir -p $WORKSPACE/build/amd64-release
cd $WORKSPACE/build/amd64-release
conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/amd64
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release
# make -j

mkdir -p $WORKSPACE/build/armhf-release
cd $WORKSPACE/build/armhf-release
conan install $CONAN_HOME --build=missing --profile:host=$CONAN_HOME/armhf --profile:build=$CONAN_HOME/amd64
cmake $WORKSPACE -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/armhf.cmake -DCMAKE_BUILD_TYPE=Release
# make -j



echo 'done.'

exit 0
