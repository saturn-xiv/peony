#!/bin/sh

set -e

# rm -rf build
mkdir -pv build
cd build

conan install .. --profile=../conan/profiles/arch --build=missing
CC=clang CXX=clang++ cmake -DCMAKE_BUILD_TYPE=Release ..

echo 'done.'

exit 0
