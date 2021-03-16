FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade
RUN apk add busybox-extras alpine-sdk build-base linux-headers \
    sudo git zsh vim zip unzip curl wget pwgen cmake rsync openssh-client tzdata tree \
    nodejs npm yarn perl \
    openssl-dev \
    mariadb-dev postgresql-dev sqlite-dev

# deploy user
RUN adduser -s /bin/bash -D deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# https://github.com/nvm-sh/nvm
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
# RUN sh -c ". $HOME/.profile \
#     && nvm install node"
# RUN sh -c ". $HOME/.nvm/nvm.sh \
#     && npm install -g yarn"

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]