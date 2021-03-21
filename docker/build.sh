#!/bin/sh

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

buildah pull ubuntu:xenial
buildah bud --layers -t peony .
podman save -o peony-$VERSION.tar peony

echo 'done.'

exit 0
