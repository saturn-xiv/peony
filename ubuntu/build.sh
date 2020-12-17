#!/bin/sh

set -e

if [ $# -ne 2 ] ; then
    echo 'Please specify your arch(amd64, armhf) AND domain name'
    exit 1
fi

export WORKSPACE=$PWD

export BUILD_ROOT=$WORKSPACE/build/$1
# rm -rf $BUILD_ROOT
mkdir -pv $BUILD_ROOT
cd $BUILD_ROOT

# -----------------------------
conan install $WORKSPACE --profile=$WORKSPACE/conan/profiles/$1 --build=missing
if [ $1 = "armhf" ]
then
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/armhf.cmake -DCMAKE_BUILD_TYPE=Release $WORKSPACE
elif [ $1 = "amd64" ]
then
    cmake -DPEONY_BUILD_STATIC=ON -DCMAKE_BUILD_TYPE=Release $WORKSPACE
else
    echo "Unknown arch $1"
    exit 1
fi

make peony

# -----------------------------

cd $WORKSPACE
if [ ! -d node_modules ]
then
    npm install
fi

if [ ! -d dashboard/node_modules ]
then
    cd dashboard
    npm install
fi
cd $WORKSPACE
REACT_GRPC_HOST=$2 npm run build

# -----------------------------

export TARGET=$WORKSPACE/docker/ubuntu
rm -rf $TARGET/usr
mkdir -pv $TARGET/usr/bin 
cp -av $BUILD_ROOT/bin/peony $TARGET/usr/bin/
mkdir -pv $TARGET/usr/share/peony
cp -r $WORKSPACE/node_modules $WORKSPACE/package.json $TARGET/usr/share/peony/
cp -r $WORKSPACE/dashboard/build  $TARGET/var/usr/share/dashboard

rm -rf $TARGET/etc
mkdir -pv $TARGET/etc/peony
cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/etc/* $TARGET/etc/peony/
echo "$(git describe --tags --always --dirty --first-parent) $(date -R)" > $TARGET/etc/peony/VERSION
echo "$1 $(lsb_release -cs) $2" >> $TARGET/etc/peony/VERSION

rm -rf $TARGET/var
mkdir -pv $TARGET/var/lib/peony

cd $TARGET
dpkg-buildpackage -us -uc -b --host-arch $1

echo 'done'

exit 0