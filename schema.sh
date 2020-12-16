#!/bin/sh

set -e

protoc -I protos --cpp_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` protos/*.proto

exit 0