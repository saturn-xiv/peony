#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

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
    podman save > peony-$VERSION.tar peony
done

echo 'done.'

exit 0