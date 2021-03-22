#!/bin/sh

set -e 

export WORKSPACE=$PWD
export CONAN_HOME=$WORKSPACE/docker/conan

if [ -d $WORKSPACE/build ]
then
    rm -r $WORKSPACE/build
fi

# mkdir -p $WORKSPACE/build/amd64-xenial-debug
# cd $WORKSPACE/build/amd64-xenial-debug
# conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/profiles/xenial/amd64
# CC=gcc-9 CXX=g++-9 cmake $WORKSPACE
# make -j

mkdir -p $WORKSPACE/build/amd64-xenial-release
cd $WORKSPACE/build/amd64-xenial-release
conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/profiles/xenial/amd64
CC=gcc-9 CXX=g++-9 cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release
make -j

# mkdir -p $WORKSPACE/build/armhf-xenial-debug
# cd $WORKSPACE/build/armhf-xenial-debug
# conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/profiles/xenial/armhf
# cmake $WORKSPACE -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/profiles/xenial/armhf.cmake
# make -j

mkdir -p $WORKSPACE/build/armhf-xenial-release
cd $WORKSPACE/build/armhf-xenial-release
conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/profiles/xenial/armhf
cmake $WORKSPACE -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/profiles/xenial/armhf.cmake -DCMAKE_BUILD_TYPE=Release
make -j



echo 'done.'

exit 0
