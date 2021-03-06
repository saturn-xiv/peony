ARG  CODE_VERSION=latest

FROM ubuntu:${CODE_VERSION}
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
    vim sudo tzdata pwgen curl zip unzip wget yasm \
    meson nasm ninja-build \
    build-essential pkg-config libtool automake autoconf binutils cmake debhelper \
    clang llvm bison flex \
    g++-arm-linux-gnueabihf pkg-config-arm-linux-gnueabihf binutils-arm-linux-gnueabihf \
    python3.10 python3.10-distutils python3.10-dev

# https://wiki.ubuntu.com/ToolChain
RUN dpkg --add-architecture armhf
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs) main restricted universe multiverse" > /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs) main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list

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
# https://github.com/ohmyzsh/ohmyzsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
RUN mkdir -p $HOME/downloads

# https://pip.pypa.io/en/stable/installing/
RUN curl https://bootstrap.pypa.io/get-pip.py -o $HOME/downloads/get-pip.py
RUN python3.10 $HOME/downloads/get-pip.py --user
RUN sh -c ". $HOME/.profile \
    && pip install --user cmake \
    && pip install --user conan"

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


# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN zsh -c "source $HOME/.cargo/env \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add armv7-unknown-linux-gnueabihf \
    && rustup target add armv7-unknown-linux-musleabihf"

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc

RUN sh -c "git clone https://github.com/microsoft/vcpkg $HOME/vcpkg"
# RUN sh -c "source $HOME/.profile \
#     && $HOME/vcpkg/bootstrap-vcpkg.sh"
# $HOME/vcpkg/vcpkg upgrade libmariadb libmysql
# RUN sh -c "source $HOME/.profile \
#     && $HOME/vcpkg/vcpkg install \
#     libpq:x64-linux libmysql:x64-linux sqlite3:x64-linux czmq:x64-linux \
# RUN sh -c "source $HOME/.profile \
#     && $HOME/vcpkg/vcpkg install \
#     libpq:arm-linux libmariadb:arm-linux sqlite3:arm-linux czmq:arm-linux"

# COPY conan /opt/conan
# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/amd64 \
#     && cd $HOME/amd64 \
#     && conan install /opt/conan --profile=/opt/conan/profiles/focal/amd64 --build=missing"
# RUN zsh -c "source $HOME/.profile \
#     && mkdir -p $HOME/armhf \
#     && cd $HOME/armhf \
#     && conan install /opt/conan --profile=/opt/conan/profiles/focal/armhf --build=missing" 

# RUN sh -c "git clone https://github.com/saturn-xiv/peony.git $HOME/peony"
# RUN zsh -c "source $HOME/.profile \
#     && $HOME/peony/grpc.sh"

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
