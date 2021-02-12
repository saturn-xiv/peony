#!/bin/sh

set -e

export WORKSPACE=$HOME/build/grpc
export GRPC_VERSION="v1.35.0"

if [ -d $WORKSPACE ]
then
    cd $WORKSPACE
    git checkout --recurse-submodules -b $GRPC_VERSION
else
    git clone --recurse-submodules -b $GRPC_VERSION https://github.com/grpc/grpc.git $WORKSPACE
fi

mkdir -p $WORKSPACE/build
cd $WORKSPACE/build

cmake -Dprotobuf_BUILD_PROTOC_BINARIES=ON -DgRPC_SSL_PROVIDER="package" -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DCMAKE_INSTALL_PREFIX=$HOME/.local $WORKSPACE
make clean
make
make install

echo 'Done.'