FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade
RUN apk add busybox-extras build-base \
    python3 python3-dev libffi-dev openssl-dev \
    sudo curl git zsh pwgen openssl pwgen rsync openssh-client tree

# deploy user
RUN adduser -s /bin/bash -D deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN mkdir $HOME/downloads
RUN curl https://bootstrap.pypa.io/get-pip.py -o $HOME/downloads/get-pip.py
RUN python3 $HOME/downloads/get-pip.py --user
RUN source $HOME/.profile && python3 -m pip install --user ansible paramiko

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.profile
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
# RUN echo 'export ANSIBLE_LOG_PATH=$HOME/logs' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]