#!/bin/sh

set -e

export WORKSPACE=$HOME/build/grpc
export GRPC_VERSION="v1.34.0"

if [ ! -d $WORKSPACE ]
then
    git clone https://github.com/grpc/grpc.git $WORKSPACE
fi

cd $WORKSPACE
git checkout $GRPC_VERSION
git submodule update --recursive

mkdir -p $WORKSPACE/build/amd64
cd $WORKSPACE/build/amd64
cmake -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DCMAKE_INSTALL_PREFIX=$HOME/.local-amd64 $WORKSPACE
make
make install

if [ "$(lsb_release -is)" == "Ubuntu" ]
then
    mkdir -p $WORKSPACE/build/armhf
    cd $WORKSPACE/build/armhf
    cmake -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DCMAKE_INSTALL_PREFIX=$HOME/.local-armhf -DCMAKE_TOOLCHAIN_FILE=$HOME/peony/armhf.cmake $WORKSPACE
    make
    make install
fi
# make install
