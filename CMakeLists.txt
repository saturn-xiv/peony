cmake_minimum_required(VERSION 3.18)

include(ExternalProject)
include(FetchContent)

project(peony VERSION 0.1.0)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -----------------------------

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# -----------------------------

find_package(OpenGL REQUIRED)

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
    GIT_TAG         "2e70e8a"
)
set(MAILIO_BUILD_EXAMPLES OFF)
set(MAILIO_BUILD_DOCUMENTATION OFF)
set(MAILIO_BUILD_LATEX_DOCUMENTATION OFF)
set(MAILIO_BUILD_SHARED_LIBRARY OFF)

FetchContent_MakeAvailable(gRPC mailio)

ExternalProject_Add(bundle
    GIT_REPOSITORY      "https://github.com/r-lyeh-archived/bundle.git"
    GIT_TAG             "ae07927"
    PREFIX              "${CMAKE_BINARY_DIR}/3rd/bundle"
    CONFIGURE_COMMAND   ""
    BUILD_COMMAND       ""
    INSTALL_COMMAND     ""
)

ExternalProject_Add(stb
    GIT_REPOSITORY      "https://github.com/nothings/stb.git"
    GIT_TAG             "b42009b"
    PREFIX              "${CMAKE_BINARY_DIR}/3rd/stb"
    CONFIGURE_COMMAND   ""
    BUILD_COMMAND       ""
    INSTALL_COMMAND     ""
)

# -----------------------------

option(PEONY_BUILD_STATIC "build all static binary" OFF)

# -----------------------------

file(GLOB PEONY_HEADERS src/*.h)
file(GLOB PEONY_SOURCES src/*.cc
    # ${CMAKE_CURRENT_BINARY_DIR}/3rd/bundle/src/bundle/bundle.cpp
)
execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
configure_file(src/config.h.in config.h)

include_directories(
    ${CMAKE_CURRENT_BINARY_DIR}
    # ${CMAKE_CURRENT_BINARY_DIR}/3rd/bundle/src/bundle
    ${CMAKE_CURRENT_BINARY_DIR}/3rd/stb/src/stb
)

add_executable(peony src/main.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
set_target_properties(peony PROPERTIES LINK_FLAGS_RELEASE -s)
add_dependencies(peony bundle stb)

if(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)
    SET(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
    SET(CMAKE_EXE_LINKER_FLAGS "-static")
    SET_TARGET_PROPERTIES(peony PROPERTIES LINK_SEARCH_END_STATIC 1)
endif(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)

set(PEONY_THIRD_LIBRARIES stdc++fs ${CONAN_LIBS} grpc++ mailio ${OPENGL_gl_LIBRARY})

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
