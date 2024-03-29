FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y install apt-transport-https software-properties-common curl gnupg

# https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa
RUN add-apt-repository -y ppa:deadsnakes/ppa

RUN apt update
RUN apt -y upgrade
RUN apt -y install zsh git locales rsync openssh-client \
    vim sudo tzdata pwgen curl zip unzip wget yasm tree \
    telnet net-tools iputils-arping iputils-ping dnsutils \
    meson nasm ninja-build \
    build-essential pkg-config libtool automake autoconf binutils cmake debhelper \
    clang llvm bison flex \
    python3.10 python3.10-distutils python3.10-dev \
    fontconfig ttf-dejavu-extra fonts-dejavu-extra \
    tmux

RUN apt update
RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN echo "zh_CN.UTF-8 UTF-8" >> /etc/locale.gen
RUN echo "zh_TW.UTF-8 UTF-8" >> /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=zh_CN.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

# deploy
RUN useradd -m deploy -s /bin/zsh
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy

USER deploy

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

RUN mkdir -p $HOME/downloads

# https://pip.pypa.io/en/stable/installing/
RUN curl https://bootstrap.pypa.io/get-pip.py -o $HOME/downloads/get-pip.py
RUN python3.10 $HOME/downloads/get-pip.py --user
RUN sh -c ". $HOME/.profile \
    && pip install --user cmake"

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
# https://nodejs.org/en/about/releases/
# FIXME: element-ui need LTS
RUN sh -c ". $HOME/.profile \
    && nvm install 'lts/*' "
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

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN echo 'export LC_ALL=zh_CN.UTF-8' >> $HOME/.zshrc
RUN echo 'export LANG=zh_CN.UTF-8' >> $HOME/.zshrc

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
