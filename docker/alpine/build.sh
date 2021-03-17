#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")
export CODE=peony-alpine

buildah pull alpine:latest

declare -a features=(
    "amd64"
    # "armhf"
)

for i in "${features[@]}"
do
    buildah bud --layers -t $CODE-$i $i.dockerfile
    podman save -o $CODE-$i-$VERSION.tar $CODE-$i
done


echo 'done.'

exit 0