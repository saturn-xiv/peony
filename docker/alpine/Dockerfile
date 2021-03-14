FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apt update

# RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]