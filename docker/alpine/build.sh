#!/bin/sh

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")
export CODE=peony-alpine

buildah pull alpine:latest
buildah bud --layers -t $CODE-amd64 .
# podman save > peony-$i-$VERSION.tar $CODE-amd64

echo 'done.'
exit 0