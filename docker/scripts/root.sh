#!/bin/sh

set -e

export DEBIAN_FRONTEND=noninteractive
apt update

# apt -y install software-properties-common
# add-apt-repository ppa:ubuntu-toolchain-r/test -y
# apt update

apt -y upgrade
apt -y install zsh git locales rsync openssh-client net-tools \
    lsb-release vim sudo tzdata pwgen curl zip unzip wget yasm \
    build-essential pkg-config libtool automake autoconf binutils cmake debhelper \
    python3 python3-pip \
    binutils-multiarch \
    g++-arm-linux-gnueabihf pkg-config-arm-linux-gnueabihf binutils-arm-linux-gnueabihf

# https://wiki.ubuntu.com/ToolChain
dpkg --add-architecture armhf
echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs) main restricted universe multiverse" > /etc/apt/sources.list
echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list
echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs) main restricted universe multiverse" >> /etc/apt/sources.list
echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list

apt update
apt -y autoremove
apt -y clean

echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
update-locale LANG=en_US.UTF-8
update-alternatives --set editor /usr/bin/vim.basic

# https://www.linaro.org/downloads/
# tar xf /opt/packages/linaro/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar -C /opt
# tar xf /opt/packages/linaro/sysroot-glibc-linaro-2.25-2019.12-arm-linux-gnueabihf.tar -C /opt
# tar xf /opt/packages/linaro/runtime-gcc-linaro-7.5.0-2019.12-arm-linux-gnueabihf.tar -C /opt

# deploy user
useradd -m deploy -s /bin/zsh
passwd -l deploy
echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy

exit 0