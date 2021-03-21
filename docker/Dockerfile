FROM ubuntu:bionic
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y install apt-transport-https software-properties-common curl gnupg

# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/test
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test

# https://docs.bazel.build/versions/master/install-ubuntu.html
# RUN curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor > /etc/apt/trusted.gpg.d/bazel.gpg
# RUN echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | tee /etc/apt/sources.list.d/bazel.list

# https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa
RUN add-apt-repository -y ppa:deadsnakes/ppa

RUN apt update
RUN apt -y upgrade
RUN apt -y install zsh git locales rsync openssh-client \
    vim sudo tzdata pwgen curl zip unzip wget yasm tree \
    meson nasm ninja-build \
    build-essential pkg-config libtool automake autoconf binutils cmake debhelper \
    g++-10 clang-10 llvm bison flex \
    python3.10 python3.10-distutils python3.10-dev


RUN apt update
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
RUN python3.10 $HOME/downloads/get-pip.py --user
RUN sh -c ". $HOME/.profile \
    && pip install --user cmake \
    && pip install --user conan \
    && pip install --user ansible paramiko"

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"
RUN sh -c ". $HOME/.profile \
    && npm i yarn -g"

ENV ANDROID_SDK_VERSION 6858069
RUN wget -P $HOME/downloads https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip
RUN mkdir -pv $HOME/local/android-sdk
RUN unzip $HOME/downloads/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip -d $HOME/local/android-sdk
# RUN yes | $HOME/local/android-sdk/cmdline-tools/tools/bin/sdkmanager --licenses

RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java 15.0.2-open \
    && sdk install maven \
    && sdk install gradle"

COPY conan toolchains peony.cmake armhf.cmake /opt/

RUN tar -xf /opt/toolchains/gcc-linaro-6.5.0-2018.12-x86_64_arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/toolchains/runtime-gcc-linaro-6.5.0-2018.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/toolchains/sysroot-glibc-linaro-2.23-2018.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains

RUN tar -xf /opt/toolchains/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/toolchains/runtime-gcc-linaro-7.5.0-2019.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains
RUN tar -xf /opt/toolchains/sysroot-glibc-linaro-2.25-2019.12-arm-linux-gnueabihf.tar.xz -C /opt/toolchains

RUN zsh -c "source $HOME/.profile \
    && mkdir -p $HOME/amd64-bionic \
    && cd $HOME/amd64-bionic \
    && conan install /opt/conan --profile=/opt/conan/profiles/amd64-bionic --build=missing"

# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/armhf-buster \
#     && cd $HOME/armhf-buster \
#     && conan install /opt/conan --profile=/opt/conan/profiles/armhf-buster --build=missing"

# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/armhf-xenial \
#     && cd $HOME/armhf-xenial \
#     && conan install /opt/conan --profile=/opt/conan/profiles/armhf-xenial --build=missing"

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.profile
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
