# https://openwrt.org/docs/guide-developer/build-system/install-buildsystem#debianubuntu
FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive
ENV OPENWRT_VERSION v19.07.7

RUN apt update
RUN apt -y upgrade
RUN apt -y install build-essential ccache ecj fastjar file g++ gawk \
    gettext git java-propose-classpath libelf-dev libncurses5-dev \
    libncursesw5-dev libssl-dev python python2.7-dev python3 unzip wget \
    python3-distutils python3-setuptools rsync subversion swig time \
    xsltproc zlib1g-dev 
RUN apt -y install sudo vim locales zsh curl cmake

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
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

# https://openwrt.org/docs/guide-developer/build-system/use-buildsystem
RUN git clone https://github.com/openwrt/openwrt.git $HOME/openwrt
RUN cd $HOME/openwrt \
    && git checkout $OPENWRT_VERSION
RUN cd $HOME/openwrt \
    && ./scripts/feeds update -a \
    && ./scripts/feeds install -a

# https://www.rust-lang.org/tools/install
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# RUN zsh -c "source $HOME/.cargo/env \
#     && rustup target add x86_64-unknown-linux-musl \
#     && rustup target add armv7-unknown-linux-gnueabihf \
#     && rustup target add armv7-unknown-linux-musleabihf"

# https://github.com/nvm-sh/nvm
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
# RUN sh -c ". $HOME/.profile \
#     && nvm install node"
# RUN sh -c ". $HOME/.profile \
#     && npm i yarn -g"

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]