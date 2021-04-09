cmake_minimum_required(VERSION 3.18)

set(CMAKE_TOOLCHAIN_FILE $ENV{HOME}/local/vcpkg/scripts/buildsystems/vcpkg.cmake)

project(peony 
    VERSION 2021.4.8
    DESCRIPTION "A total free education & translation & ops solution."
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
set(CMAKE_EXE_LINKER_FLAGS "-static")

find_package(Boost REQUIRED COMPONENTS log date_time program_options)
find_package(OpenSSL REQUIRED)
# find_package(libpq CONFIG REQUIRED)
find_package(unofficial-libmariadb CONFIG REQUIRED)
find_package(unofficial-sqlite3 CONFIG REQUIRED) 
find_package(hiredis CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(toml11 CONFIG REQUIRED)
find_package(czmq CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(eclipse-paho-mqtt-c CONFIG REQUIRED)

include(CTest)


execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
configure_file(include/peony/version.h.in version.h)

add_subdirectory(src)
add_subdirectory(apps)

if(BUILD_TESTING)
    add_subdirectory(tests)
endif()
