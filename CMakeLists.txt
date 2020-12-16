cmake_minimum_required(VERSION 3.16)

include(ExternalProject)
include(FetchContent)

project(peony VERSION 0.1.0)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -----------------------------
option(PEONY_BUILD_STATIC "build all static binary" OFF)

if(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)
    set(OPENSSL_USE_STATIC_LIBS TRUE)
    set(Boost_USE_STATIC_LIBS ON)
endif(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)

find_package(OpenSSL REQUIRED Crypto SSL)
find_package(Boost REQUIRED log_setup log thread system filesystem regex timer date_time chrono random  program_options unit_test_framework)

# set(BUILD_EXAMPLES OFF)
# set(BUILD_STATIC_LIBS ON)
# set(BUILD_TESTS OFF)
# set(BUILD_TOOLS OFF)
# FetchContent_Declare(RabbitMQ
#     GIT_REPOSITORY "https://github.com/alanxz/rabbitmq-c.git"
#     GIT_TAG "v0.10.0"
# )

# FetchContent_MakeAvailable(RabbitMQ)

# -----------------------------

file(GLOB PEONY_HEADERS src/*.h)
file(GLOB PEONY_SOURCES src/*.cc)
execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
configure_file(src/config.h.in config.h)
include_directories(
    ${CMAKE_CURRENT_BINARY_DIR}
)

add_executable(peony src/main.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
set_target_properties(peony PROPERTIES LINK_FLAGS_RELEASE -s)

if(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)
    SET(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
    SET(CMAKE_EXE_LINKER_FLAGS "-static")
    SET_TARGET_PROPERTIES(peony PROPERTIES LINK_SEARCH_END_STATIC 1)
endif(CMAKE_CROSSCOMPILING OR PEONY_BUILD_STATIC)

# -----------------------------
set(THIRD_LIBRARIES stdc++fs 
    ${OPENSSL_LIBRARIES}
    ${Boost_LIBRARIES}
    rabbitmq
)
# -----------------------------
 
target_link_libraries(peony ${THIRD_LIBRARIES})

# -----------------------------

include(CTest)
enable_testing()

add_executable(queue_test src/queue_test.cpp ${PEONY_HEADERS} ${PEONY_SOURCES})
target_link_libraries(queue_test ${THIRD_LIBRARIES})

