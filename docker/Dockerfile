FROM ubuntu:xenial
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive
ENV PYTHON_VERSION "3.9"

RUN apt update
RUN apt -y install apt-transport-https software-properties-common curl gnupg

# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/test
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test
# https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa
RUN add-apt-repository -y ppa:deadsnakes/ppa

# https://wiki.ubuntu.com/ToolChain
RUN dpkg --add-architecture armhf
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs) main restricted universe multiverse" > /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs) main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list

RUN apt update
RUN apt -y upgrade

RUN apt -y install zsh git locales rsync openssh-client \
    vim sudo tzdata pwgen curl zip unzip wget yasm tree \
    meson nasm ninja-build \
    build-essential pkg-config libtool automake autoconf binutils cmake debhelper \
    g++-9 clang llvm bison flex texinfo help2man libtool-bin \
    g++-arm-linux-gnueabihf pkg-config-arm-linux-gnueabihf binutils-arm-linux-gnueabihf \
    telnet net-tools iputils-arping iputils-ping dnsutils \
    python${PYTHON_VERSION} python${PYTHON_VERSION}-distutils python${PYTHON_VERSION}-dev

RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

# deploy
RUN useradd -m deploy -s /bin/zsh
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy

USER deploy

RUN mkdir -p $HOME/downloads

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

# https://pip.pypa.io/en/stable/installing/
RUN curl https://bootstrap.pypa.io/get-pip.py -o $HOME/downloads/get-pip.py
RUN python${PYTHON_VERSION} $HOME/downloads/get-pip.py --user
RUN sh -c ". $HOME/.profile && python${PYTHON_VERSION} -m pip install --user cmake"
RUN sh -c ". $HOME/.profile && python${PYTHON_VERSION} -m pip install --user conan"
RUN sh -c ". $HOME/.profile && python${PYTHON_VERSION} -m pip install --user ansible"
RUN sh -c ". $HOME/.profile && python${PYTHON_VERSION} -m pip install --user paramiko"

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"
RUN sh -c ". $HOME/.profile \
    && npm i yarn -g"

# ENV ANDROID_SDK_VERSION 6858069
# RUN wget -P $HOME/downloads https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip
# RUN mkdir -pv $HOME/local/android-sdk
# RUN unzip $HOME/downloads/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip -d $HOME/local/android-sdk
# RUN yes | $HOME/local/android-sdk/cmdline-tools/tools/bin/sdkmanager --licenses

RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java 15.0.2-open \
    && sdk install maven \
    && sdk install gradle"

COPY toolchains /opt/packages
RUN sudo mkdir /opt/toolchains
RUN sudo chown deploy:deploy /opt/toolchains
RUN tar -xf /opt/packages/gcc-linaro-6.5.0-2018.12-x86_64_arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/packages/runtime-gcc-linaro-6.5.0-2018.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/packages/sysroot-glibc-linaro-2.23-2018.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains

RUN tar -xf /opt/packages/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/packages/runtime-gcc-linaro-7.5.0-2019.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/packages/sysroot-glibc-linaro-2.25-2019.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains

RUN tar -xf /opt/packages/x-tools.tar.xz -C /opt/toolchains

# BUILDROOT & CROSSTOOL-ng
# RUN git clone --depth 1 https://github.com/buildroot/buildroot.git $HOME/build/buildroot
# RUN git clone --depth 1 --branch 2021.02 https://github.com/crosstool-ng/crosstool-ng $HOME/build/crosstool-ng

# build crosstool-ng
# RUN mkdir $HOME/src
# RUN zsh -c "source $HOME/.zshrc \
#     && cd $HOME/build/crosstool-ng \
#     && ./configure --prefix=$HOME/.local \
#     && make \
#     && make install"
# COPY crosstool-ng /opt/crosstool-ng
# RUN zsh -c "source $HOME/.zshrc \
#     && mkdir -p $HOME/build/amd64 \
#     && cd $HOME/build/amd64 \
#     && cp /opt/crosstool-ng/amd64 .config \
#     && ct-ng build"
# RUN zsh -c "source $HOME/.zshrc \
#     && mkdir -p $HOME/build/armhf \
#     && cd $HOME/build/armhf \
#     && cp /opt/crosstool-ng/armhf .config \
#     && ct-ng build"

# COPY build.sh /opt/
# COPY conan /opt/conan
# COPY cmake /opt/cmake


# RUN sudo apt install -y libgl1-mesa-dev libglu1-mesa-dev \
#     libgl1-mesa-dev:armhf libglu1-mesa-dev:armhf
# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/amd64-bionic \
#     && cd $HOME/amd64-bionic \
#     && conan install /opt/conan --profile=/opt/conan/profiles/amd64-bionic --build=missing"

# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/armhf-buster \
#     && cd $HOME/armhf-buster \
#     && conan install /opt/conan --profile=/opt/conan/profiles/armhf-buster --build=missing"

# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/armhf-xenial \
#     && cd $HOME/armhf-xenial \
#     && conan install /opt/conan --profile=/opt/conan/profiles/armhf-xenial --build=missing"












RUN echo 'source $HOME/.profile' >> $HOME/.zshrc

RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
