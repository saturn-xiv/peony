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

FetchContent_MakeAvailable(gRPC)

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

set(PEONY_THIRD_LIBRARIES stdc++fs ${CONAN_LIBS} grpc++)

target_link_libraries(peony ${PEONY_THIRD_LIBRARIES})
# -----------------------------

include(CTest)
enable_testing()

add_executable(audio-test src/audio-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(audio-test ${PEONY_THIRD_LIBRARIES})

add_executable(elastic-search-test src/elastic-search-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(elastic-search-test ${PEONY_THIRD_LIBRARIES})

add_executable(gpio-test src/gpio-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(gpio-test ${PEONY_THIRD_LIBRARIES})

add_executable(tty-test src/tty-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(tty-test ${PEONY_THIRD_LIBRARIES})

add_executable(twilio-test src/twilio-test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(twilio-test ${PEONY_THIRD_LIBRARIES})
