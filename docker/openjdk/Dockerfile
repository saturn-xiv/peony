FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade
RUN apk add busybox-extras \
    git sudo zsh vim zip unzip curl wget pwgen rsync openssh-client tzdata \
    screen tmux ttf-dejavu \
    nodejs npm yarn

# deploy user
RUN adduser -s /bin/bash -D deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

RUN mkdir -p $HOME/downloads $HOME/local

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

# https://jdk.java.net/17/
RUN wget -P $HOME/downloads https://download.java.net/java/early_access/alpine/10/binaries/openjdk-17-ea+10_linux-x64-musl_bin.tar.gz
RUN tar -xf $HOME/downloads/openjdk-17-ea+10_linux-x64-musl_bin.tar.gz -C $HOME/local

# https://gradle.org/install/#manually
ENV GRADLE_VERSION="6.8.3"
RUN wget -P $HOME/downloads https://downloads.gradle-dn.com/distributions/gradle-${GRADLE_VERSION}-bin.zip
RUN unzip $HOME/downloads/gradle-${GRADLE_VERSION}-bin.zip -d $HOME/local

# https://maven.apache.org/download.cgi
ENV MAVEN_VERSION="3.6.3"
RUN wget -P $HOME/downloads https://mirror.jframeworks.com/apache/maven/maven-3/${MAVEN_VERSION}/binaries/apache-maven-${MAVEN_VERSION}-bin.tar.gz
RUN tar -xf $HOME/downloads/apache-maven-${MAVEN_VERSION}-bin.tar.gz -C $HOME/local

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc

# setup java
RUN echo "JAVA_HOME=\$HOME/local/jdk-17" >> $HOME/.profile
RUN echo "MAVEN_HOME=\$HOME/local/apache-maven-${MAVEN_VERSION}" >> $HOME/.profile
RUN echo "GRADLE_HOME=\$HOME/local/gradle-${GRADLE_VERSION}" >> $HOME/.profile
RUN echo "PATH=\$JAVA_HOME/bin:\$MAVEN_HOME/bin:\$GRADLE_HOME/bin:\$PATH" >> $HOME/.profile
RUN echo "export JAVA_HOME MAVEN_HOME GRADLE_HOME PATH" >> $HOME/.profile

RUN git config --global pull.rebase false

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]