cmake_minimum_required(VERSION 3.18)

include(ExternalProject)
include(FetchContent)

project(peony VERSION 0.1.1)

include(peony.cmake)

# FetchContent_Declare(gRPC
#     GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
#     GIT_TAG         "v1.34.0"
# )
# set(protobuf_BUILD_PROTOC_BINARIES ON)
# set(gRPC_SSL_PROVIDER "package")


# FetchContent_Declare(ZXing
#     GIT_REPOSITORY  "https://github.com/nu-book/zxing-cpp.git"
#     GIT_TAG         "v1.1.1"
# )

# set(BUILD_SHARED_LIBS OFF)
# FetchContent_Declare(mailio
#     GIT_REPOSITORY  "https://github.com/karastojko/mailio.git"
#     GIT_TAG         "2e70e8a"
# )
# set(MAILIO_BUILD_EXAMPLES OFF)
# set(MAILIO_BUILD_DOCUMENTATION OFF)
# set(MAILIO_BUILD_LATEX_DOCUMENTATION OFF)
# set(MAILIO_BUILD_SHARED_LIBRARY OFF)

# FetchContent_MakeAvailable(mailio)

add_executable(peony src/main.cpp ${PEONY_SOURCES} ${PEONY_IMGUI_SOURCES})
target_link_libraries(peony ${PEONY_THIRD_LIBRARIES})



