cmake_minimum_required(VERSION 3.16)

# include(ExternalProject)
include(FetchContent)

project(peony VERSION 0.1.0)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -----------------------------

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# -----------------------------

set(FETCHCONTENT_QUIET OFF)

FetchContent_Declare(gRPC
    GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
    GIT_TAG         "v1.34.0"
)
set(protobuf_BUILD_PROTOC_BINARIES ON)
set(gRPC_SSL_PROVIDER "package")


FetchContent_Declare(mailio
    GIT_REPOSITORY  "https://github.com/karastojko/mailio.git"
    GIT_TAG         "version_0-20-0"
)

FetchContent_MakeAvailable(gRPC mailio)

# -----------------------------

option(PEONY_BUILD_STATIC "build all static binary" OFF)

# -----------------------------

file(GLOB PEONY_HEADERS src/*.h)
file(GLOB PEONY_SOURCES src/*.cc)
execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
configure_file(src/config.h.in config.h)

include_directories(${CMAKE_CURRENT_BINARY_DIR})

add_executable(peony src/main.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
set_target_properties(peony PROPERTIES LINK_FLAGS_RELEASE -s)

if(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)
    SET(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
    SET(CMAKE_EXE_LINKER_FLAGS "-static")
    SET_TARGET_PROPERTIES(peony PROPERTIES LINK_SEARCH_END_STATIC 1)
endif(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)

set(PEONY_THIRD_LIBRARIES stdc++fs ${CONAN_LIBS} grpc++ mailio)

target_link_libraries(peony ${PEONY_THIRD_LIBRARIES})
# -----------------------------

include(CTest)
enable_testing()

add_executable(audio-test src/audio-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(audio-test ${PEONY_THIRD_LIBRARIES})

add_executable(aws-test src/aws-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(aws-test ${PEONY_THIRD_LIBRARIES})

add_executable(crawler-test src/crawler-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(crawler-test ${PEONY_THIRD_LIBRARIES})

add_executable(crontab-test src/crontab-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(crontab-test ${PEONY_THIRD_LIBRARIES})

add_executable(crypt-test src/crypt-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(crypt-test ${PEONY_THIRD_LIBRARIES})

add_executable(elastic-search-test src/elastic-search-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(elastic-search-test ${PEONY_THIRD_LIBRARIES})

add_executable(gpio-test src/gpio-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(gpio-test ${PEONY_THIRD_LIBRARIES})

add_executable(jwt-test src/jwt-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(jwt-test ${PEONY_THIRD_LIBRARIES})

add_executable(queue-test src/queue-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(queue-test ${PEONY_THIRD_LIBRARIES})

add_executable(tty-test src/tty-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(tty-test ${PEONY_THIRD_LIBRARIES})

add_executable(twilio-test src/twilio-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(twilio-test ${PEONY_THIRD_LIBRARIES})

add_executable(utils-test src/utils-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(utils-test ${PEONY_THIRD_LIBRARIES})
