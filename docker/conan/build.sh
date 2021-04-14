#!/bin/sh

set -e 

export WORKSPACE=$PWD
export CONAN_HOME=$WORKSPACE/docker/conan
export CLANG_HOME="$HOME/local/clang+llvm-11.1.0-x86_64-linux-gnu-ubuntu-16.04"
export PATH=$CLANG_HOME/bin:$PATH
export CC=clang
export CXX=clang++

if [ -d $WORKSPACE/build ]
then
    rm -r $WORKSPACE/build
fi

mkdir -p $WORKSPACE/build/amd64-debug
cd $WORKSPACE/build/amd64-debug
conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/amd64
# CC=gcc CXX=g++-9
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Debug
# make -j

mkdir -p $WORKSPACE/build/amd64-release
cd $WORKSPACE/build/amd64-release
conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/amd64
cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release
# # make -j

# mkdir -p $WORKSPACE/build/armhf-release
# cd $WORKSPACE/build/armhf-release
# conan install $CONAN_HOME --build=missing --profile:host=$CONAN_HOME/armhf --profile:build=$CONAN_HOME/amd64
# cmake $WORKSPACE -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/armhf.cmake -DCMAKE_BUILD_TYPE=Release
# # make -j



echo 'done.'

exit 0
