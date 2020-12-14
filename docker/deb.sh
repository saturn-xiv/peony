#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(x64, win, arm) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD

export BUILD_ROOT=$WORKSPACE/build/$2-$1
rm -rfv $BUILD_ROOT
mkdir -pv $BUILD_ROOT
cd $BUILD_ROOT

if [ $1 = "arm" ]
then
    sudo apt install -y 
elif [ $1 = "win" ]
then
    sudo apt install -y 
elif [ $1 = "x64" ]
then
    sudo apt install -y 
else
    echo "Unknown arch $1"
    exit 1
fi

export TARGET=$WORKSPACE/ubuntu
rm -rfv $TARGET/usr
mkdir -pv $TARGET/usr/bin 
cp -av bin/ashoka $TARGET/usr/bin/
strip -s $TARGET/usr/bin/ashoka
mkdir -pv $TARGET/usr/share/ashoka
cp -av $WORKSPACE/db $TARGET/usr/share/ashoka/

rm -rfv $TARGET/etc
mkdir -pv $TARGET/etc/ashoka
cp $WORKSPACE/LICENSE $WORKSPACE/config.toml $TARGET/etc/ashoka/

cd $WORKSPACE
dpkg -b ubuntu $2-$1-$(lsb_release -cs)-$(git describe --tags --always --dirty --first-parent).deb

echo 'done'

exit 0