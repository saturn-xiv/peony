#!/bin/bash

set -e

# https://wiki.ubuntu.com/Releases
declare -a features=(
    "focal"
    "bionic"
    "xenial"
)

for i in "${features[@]}"
do
    docker pull ubuntu:$i
    docker build --build-arg CODE_VERSION=$i -t peony-$i .
done

echo 'done.'

exit 0