#!/bin/sh

set -e

export WORKSPACE=$HOME/build/grpc
export GRPC_VERSION="v1.34.0"

if [ ! -d $WORKSPACE ]
then
    git clone --recurse-submodules -b $GRPC_VERSION https://github.com/grpc/grpc.git $WORKSPACE
fi

sudo apt install -y libssl-dev
mkdir -p $WORKSPACE/build
cd $WORKSPACE/build

cmake -Dprotobuf_BUILD_PROTOC_BINARIES=ON -DgRPC_SSL_PROVIDER="package" -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DCMAKE_INSTALL_PREFIX=$HOME/.local $WORKSPACE
make
make install
make clean

sudo apt remove -y libssl-dev