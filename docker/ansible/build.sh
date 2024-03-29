#!/bin/sh

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")
export CODE=peony-ansible

buildah pull alpine:latest

buildah bud --layers -t $CODE .
podman save -o $CODE-$VERSION.tar $CODE

echo 'done.'
exit 0