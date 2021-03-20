#!/bin/sh

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

buildah pull ubuntu:xenial
buildah bud --layers --build-arg -t peony .
podman save -o peony-$VERSION.tar peony

echo 'done.'

exit 0
