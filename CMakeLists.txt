cmake_minimum_required(VERSION 3.18)

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

find_package(Boost REQUIRED
    Boost::date_time Boost::log
)
find_package(eclipse-paho-mqtt-c CONFIG REQUIRED)

include(CTest)
add_subdirectory(src)

if(BUILD_TESTING)
add_subdirectory(tests)
endif()



# include(docker/conan/peony.cmake)


# include(ExternalProject)
# include(FetchContent)

# SET(CMAKE_CXX_STANDARD 17)
# SET(CMAKE_CXX_STANDARD_REQUIRED ON)

# # -----------------------------
# include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
# conan_basic_setup()
# # -----------------------------
# find_package(Threads REQUIRED)

# # FetchContent_Declare(ZXing
# #     GIT_REPOSITORY  "https://github.com/nu-book/zxing-cpp.git"
# #     GIT_TAG         "v1.1.1"
# # )

# SET(FETCHCONTENT_QUIET OFF)

# # -----------------------------

# SET(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
# SET(CMAKE_EXE_LINKER_FLAGS "-static")

# SET(PEONY_THIRD_LIBRARIES stdc++fs
#     ${CMAKE_THREAD_LIBS_INIT}    
#     ${CONAN_LIBS}
# )

# file(GLOB PEONY_HEADERS src/*.h)
# file(GLOB PEONY_SOURCES src/*.cc)

# execute_process(COMMAND git describe --tags --always --dirty
#     OUTPUT_VARIABLE GIT_REV
#     ERROR_QUIET
# )
# string(STRIP "${GIT_REV}" GIT_REV)
# configure_file(src/config.h.in config.h)
# include_directories(
#     ${CMAKE_CURRENT_BINARY_DIR}
#     ${PEONTY_HEADERS}
# )

# include(CTest)
# enable_testing()

# add_executable(peony-test src/peony-test.cpp src/peony.cc)
# target_link_libraries(peony-test ${PEONY_THIRD_LIBRARIES})

# # FetchContent_Declare(gRPC
# #     GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
# #     GIT_TAG         "v1.34.0"
# # )
# # set(protobuf_BUILD_PROTOC_BINARIES ON)
# # set(gRPC_SSL_PROVIDER "package")


# # FetchContent_Declare(ZXing
# #     GIT_REPOSITORY  "https://github.com/nu-book/zxing-cpp.git"
# #     GIT_TAG         "v1.1.1"
# # )

# # set(BUILD_SHARED_LIBS OFF)
# # FetchContent_Declare(mailio
# #     GIT_REPOSITORY  "https://github.com/karastojko/mailio.git"
# #     GIT_TAG         "2e70e8a"
# # )
# # set(MAILIO_BUILD_EXAMPLES OFF)
# # set(MAILIO_BUILD_DOCUMENTATION OFF)
# # set(MAILIO_BUILD_LATEX_DOCUMENTATION OFF)
# # set(MAILIO_BUILD_SHARED_LIBRARY OFF)

# # FetchContent_MakeAvailable(mailio)

# add_executable(peony src/main.cpp ${PEONY_SOURCES})
# target_link_libraries(peony ${PEONY_THIRD_LIBRARIES})
# set_target_properties(peony PROPERTIES LINK_FLAGS_RELEASE -s)
# set_target_properties(peony PROPERTIES LINK_SEARCH_END_STATIC 1)
