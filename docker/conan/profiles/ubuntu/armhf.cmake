SET(CMAKE_SYSTEM_NAME Linux)
SET(CMAKE_SYSTEM_PROCESSOR arm)

SET(CROSSTOOL_PREFIX /opt/gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf/bin/arm-none-linux-gnueabihf)
# SET(CROSSTOOL_PREFIX /opt/x-tools/arm-peony-linux-gnueabi/bin/arm-peony-linux-gnueabi)

SET(CMAKE_C_COMPILER   ${CROSSTOOL_PREFIX}-gcc)
SET(CMAKE_CXX_COMPILER ${CROSSTOOL_PREFIX}-g++)


# SET(CMAKE_SYSROOT /opt/toolchains/sysroot-glibc-linaro-2.23-2018.12-arm-linux-gnueabihf)
# set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
# set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
# set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
# set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)