#!/bin/bash

set -e

buildah pull ubuntu:latest

# https://wiki.ubuntu.com/Releases
declare -a features=(
    "focal"
    "bionic"
    "xenial"
)

for i in "${features[@]}"
do
    buildah pull ubuntu:$i
    buildah bud --build-arg CODE_VERSION=$i -t peony-$i .
done

echo 'done.'

exit 0