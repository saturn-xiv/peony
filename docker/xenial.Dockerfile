FROM ubuntu:xenial
LABEL maintainer="Jeremy Zheng"

ADD scripts/root.sh /opt/root.sh
RUN sh /opt/root.sh
USER deploy
ADD scripts/deploy.sh /opt/deploy.sh
RUN sh /opt/deploy.sh

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
