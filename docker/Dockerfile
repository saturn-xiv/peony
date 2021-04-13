FROM ubuntu:groovy
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install apt-transport-https software-properties-common curl gnupg
RUN apt -y install zsh git locales rsync openssh-client screen tmux \
    vim sudo tzdata pwgen curl zip unzip wget yasm tree cpio bc \
    build-essential pkg-config cmake libtool automake autoconf binutils debhelper libncurses5-dev \
    clang llvm bison flex meson \
    crossbuild-essential-armhf g++-arm-linux-gnueabihf pkg-config-arm-linux-gnueabihf binutils-arm-linux-gnueabihf \
    python3 python3-distutils python3-dev

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
RUN python3 $HOME/downloads/get-pip.py --user
RUN sh -c ". $HOME/.profile \
    && pip install --user cmake \
    && pip install --user conan \
    && pip install --user ansible paramiko"
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile


# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"
RUN sh -c ". $HOME/.profile \
    && npm i npm -g \
    && npm i yarn -g"

RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java 15.0.2-open \
    && sdk install maven \
    && sdk install gradle"

RUN git clone https://github.com/microsoft/vcpkg $HOME/local/vcpkg
RUN zsh -c "source $HOME/.profile \
    && $HOME/local/vcpkg/bootstrap-vcpkg.sh"
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.profile
COPY vcpkg /opt/
# RUN zsh /opt/vcpkg.sh

COPY conan /opt/conan/
# RUN zsh -c "source $HOME/.profile \
#     && conan remote update conan-center https://api.bintray.com/conan/conan/conan-center"
RUN zsh -c "source $HOME/.profile \
    && mkdir $HOME/amd64 \
    && cd $HOME/amd64 \
    && conan install /opt/conan --build=missing --profile=/opt/conan/amd64"
RUN zsh -c "source $HOME/.profile \
    && mkdir $HOME/armhf \
    && cd $HOME/armhf \
    && conan install /opt/conan --build=missing --profile:host=/opt/conan/armhf --profile:build=/opt/conan/amd64"

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.profile

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
