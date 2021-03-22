SET(CMAKE_SYSTEM_NAME Linux)
SET(CMAKE_SYSTEM_PROCESSOR arm)

SET(CMAKE_SYSROOT /opt/toolchains/sysroot-glibc-linaro-2.23-2018.12-arm-linux-gnueabihf)
SET(CROSSTOOL_PREFIX /opt/toolchains/gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf/bin/arm-none-linux-gnueabihf)


# SET(PKG_CONFIG_EXECUTABLE arm-linux-gnueabihf-pkg-config)
# SET(ENV{PKG_CONFIG_ALLOW_CROSS} 1)
# SET(ENV{PKG_CONFIG_DIR} "")
# SET(ENV{PKG_CONFIG_LIBDIR} /usr/lib/arm-linux-gnueabihf/pkgconfig)
# SET(CMAKE_PREFIX_PATH /usr/lib/arm-linux-gnueabihf/pkgconfig)

SET(CMAKE_C_COMPILER   ${CROSSTOOL_PREFIX}-gcc)
SET(CMAKE_CXX_COMPILER ${CROSSTOOL_PREFIX}-g++)

# SET(ENV{PKG_CONFIG_LIBDIR} /usr/lib/arm-linux-gnueabihf/pkgconfig/)  


# SET(CMAKE_FIND_ROOT_PATH ${CMAKE_FIND_ROOT_PATH} /opt/toolchains/sysroot-glibc-linaro-2.23-2018.12-arm-linux-gnueabihf) 

# set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
# set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
# set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
# set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

# https://autotools.io/pkgconfig/cross-compiling.html
# include(FindPkgConfig)
# SET(CMAKE_PREFIX_PATH /usr/lib/arm-linux-gnueabihf/pkgconfig)
# SET(PKG_CONFIG_EXECUTABLE /tmp/pkg-config)