FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade
RUN apk add alpine-sdk build-base \
    sudo zsh vim zip unzip curl wget pwgen \
    nodejs npm yarn \
    openssl-dev

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

# RUN echo 'RUSTFLAGS="-C target-feature=+crt-static"' >> $HOME/.profile
RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]