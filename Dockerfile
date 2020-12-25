FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install software-properties-common curl gnupg
RUN add-apt-repository ppa:ubuntu-toolchain-r/test -y
# https://docs.bazel.build/versions/master/install-ubuntu.html
RUN curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor > /etc/apt/trusted.gpg.d/bazel.gpg
RUN echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | tee /etc/apt/sources.list.d/bazel.list
RUN apt update
RUN apt upgrade
RUN apt -y install zsh git locales rsync openssh-client \
    vim sudo tzdata pwgen curl zip unzip wget yasm \
    meson nasm bazel ninja-build \
    build-essential pkg-config libtool automake autoconf binutils cmake clang llvm debhelper \
    binutils-multiarch \
    g++-arm-linux-gnueabihf pkg-config-arm-linux-gnueabihf binutils-arm-linux-gnueabihf \
    mingw-w64 mingw-w64-tools binutils-mingw-w64 \
    python3 python3-pip python3-distutils python3-dev \
    bazel

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

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"

ENV ANDROID_SDK_VERSION 6858069
RUN wget -P $HOME/downloads https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip
RUN mkdir -pv $HOME/local/android-sdk
RUN unzip $HOME/downloads/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip -d $HOME/local/android-sdk
# RUN yes | $HOME/local/android-sdk/cmdline-tools/tools/bin/sdkmanager --licenses

RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java 15.0.1-open \
    && sdk install maven \
    && sdk install gradle"

RUN pip3 install --user cmake
RUN pip3 install --user conan

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN zsh -c "source $HOME/.cargo/env \
    && rustup target add armv7-unknown-linux-gnueabihf"

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN sh -c "git clone https://github.com/saturn-xiv/peony.git $HOME/peony"
RUN zsh -c "source $HOME/.profile && $HOME/peony/grpc.sh"

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
