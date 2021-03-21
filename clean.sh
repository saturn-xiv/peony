#!/bin/sh

set -e 

export WORKSPACE=$PWD
export CONAN_HOME=$WORKSPACE/docker/conan

if [ -d $WORKSPACE/build ]
then
    rm -r $WORKSPACE/build
fi

mkdir -p $WORKSPACE/build/amd64-xenial-debug
cd $WORKSPACE/build/amd64-xenial-debug
conan install $CONAN_HOME --build=missing --profile=$CONAN_HOME/profiles/xenial/amd64
CC=gcc-9 CXX=g++-9 cmake $WORKSPACE
make 


# mkdir -p $WORKSPACE/build/amd64-bionic-release
# cd $WORKSPACE/build/amd64-bionic-release
# conan install $PROFILE_HOME/conan --build=missing --profile=$PROFILE_HOME/conan/profiles/amd64-bionic
# CC=gcc-10 CXX=g++-10 cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release


# mkdir -p $WORKSPACE/build/armhf-buster-debug
# cd $WORKSPACE/build/armhf-buster-debug
# conan install $PROFILE_HOME/conan --build=missing --profile=$PROFILE_HOME/conan/profiles/armhf-buster
# CC=gcc-10 CXX=g++-10 cmake $WORKSPACE

# mkdir -p $WORKSPACE/build/armhf-debug
# cd $WORKSPACE/build/armhf-debug
# conan install $WORKSPACE --build=missing --profile=/opt/conan/profiles/armhf
# cmake $WORKSPACE -DCMAKE_TOOLCHAIN_FILE=/opt/armhf.cmake

# mkdir -p $WORKSPACE/build/armhf-release
# cd $WORKSPACE/build/armhf-release
# conan install $WORKSPACE --build=missing --profile=/opt/conan/profiles/armhf
# cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release -DCMAKE_TOOLCHAIN_FILE=/opt/armhf.cmake


echo 'done.'

exit 0
