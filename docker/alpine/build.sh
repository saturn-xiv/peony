#!/bin/sh

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

buildah pull alpine:latest
buildah bud --layers -t peony-alpine-amd64 .
podman save > peony-$i-$VERSION.tar peony-alpine-amd64

echo 'done.'
exit 0