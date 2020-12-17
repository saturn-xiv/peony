#!/bin/sh

set -e

$HOME/.local/bin/protoc --version
$HOME/.local/bin/protoc -I etc/protos --cpp_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` etc/protos/*.proto

echo 'done.'

exit 0