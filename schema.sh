#!/bin/sh

set -e

$HOME/.local/bin/protoc --version

$HOME/.local/bin/protoc -I etc/protos --cpp_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` etc/protos/*.proto

export PHP_OUT=tmp/php
rm -r $PHP_OUT
mkdir -p $PHP_OUT
$HOME/.local/bin/protoc -I etc/protos --php_out=$PHP_OUT --grpc_out=$PHP_OUT --plugin=protoc-gen-grpc=`which grpc_php_plugin` etc/protos/*.proto

echo 'done.'

exit 0