#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

buildah pull ubuntu:latest
buildah bud --layers -t peony-buildroot .
podman save -o peony-buildroot-$VERSION.tar peony-buildroot

echo 'done.'

exit 0
