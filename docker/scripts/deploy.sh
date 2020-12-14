#!/bin/sh

set -e

# https://github.com/ohmyzsh/ohmyzsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended

# https://github.com/nvm-sh/nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | sh
. $HOME/.profile
nvm install node

# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

. $HOME/.cargo/env
rustup target add armv7-unknown-linux-gnueabihf

mkdir $HOME/downloads

ENV ANDROID_SDK_VERSION 6858069
wget -P $HOME/downloads https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip
mkdir -pv $HOME/local/android-sdk
unzip $HOME/downloads/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip -d $HOME/local/android-sdk
# yes | $HOME/local/android-sdk/cmdline-tools/tools/bin/sdkmanager --licenses

curl -s "https://get.sdkman.io" | zsh
sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
zsh -c "source $HOME/.sdkman/bin/sdkman-init.sh \
    && sdk install java 15.0.1-open \
    && sdk install maven \
    && sdk install gradle"

echo 'source $HOME/.profile' >> $HOME/.zshrc

git clone https://github.com/saturn-xiv/peony.git $HOME/peony

exit 0