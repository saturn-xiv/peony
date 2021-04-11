#!/bin/bash

set -e

export PATH=$HOME/local/gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf/bin:$PATH

declare -a packages=(
    # "poco"
    # "poco[sqlite3]"
    # "poco[mariadb]"
    # "poco[postgresql]"
    # "poco[pdf]"

    "boost-log"    
    "boost-date-time"
    "boost-property-tree"
    "boost-algorithm"
    "boost-program-options"
    "boost-format"
    "boost-test"

    "soci"
    "soci[boost]"
    "soci[sqlite3]"
    "soci[postgresql]"

    # FIXME arm-linux
    # "soci[mysql]"

    "openssl"
    "libsodium"
    # "libpq"
    # "libmariadb"
    # "sqlite3"
    "jwt-cpp"
    "toml11"

    "inja"
    "nlohmann-json"
    "cpp-httplib"
    "curl"
    
    "hiredis"
    "cppzmq"
    "librabbitmq"
    "paho-mqtt"
)

declare -a triplets=(
    "x64-linux"
    "arm-linux"
)

for p in "${packages[@]}"
do
    for t in "${triplets[@]}"
    do
        $HOME/local/vcpkg/vcpkg install $p:$t --recurse
    done
done

echo 'done.'

exit 0
