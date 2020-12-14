#!/bin/bash

set -e

declare -a features=(
    "focal"
    "bionic"
    "xenial"
)


for i in "${features[@]}"
do
    docker pull ubuntu:$i
    docker build -t peony-$i -f $i.Dockerfile
done

echo 'done.'
exit 0