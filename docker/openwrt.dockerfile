# https://openwrt.org/docs/guide-developer/build-system/install-buildsystem#debianubuntu
FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive
ENV OPENWRT_VERSION v19.07.6

RUN apt update
RUN apt -y upgrade
RUN apt -y install build-essential ccache ecj fastjar file g++ gawk \
    gettext git java-propose-classpath libelf-dev libncurses5-dev \
    libncursesw5-dev libssl-dev python python2.7-dev python3 unzip wget \
    python3-distutils python3-setuptools rsync subversion swig time \
    xsltproc zlib1g-dev 
RUN apt -y install sudo vim locales zsh curl


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

# https://openwrt.org/docs/guide-developer/build-system/use-buildsystem
RUN git clone https://github.com/openwrt/openwrt.git $HOME/openwrt
RUN cd $HOME/openwrt \
    && git checkout $OPENWRT_VERSION
RUN cd $HOME/openwrt \
    && ./scripts/feeds update -a \
    && ./scripts/feeds install -a

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]