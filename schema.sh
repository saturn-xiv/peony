#!/bin/sh

set -e

$HOME/.local/bin/protoc -I protos --cpp_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` protos/*.proto

exit 0