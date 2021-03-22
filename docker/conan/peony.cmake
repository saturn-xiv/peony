include(ExternalProject)
include(FetchContent)

SET(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_STANDARD_REQUIRED ON)

# -----------------------------
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()
# -----------------------------
find_package(Threads REQUIRED)
find_package(OpenGL REQUIRED)


# FetchContent_Declare(ZXing
#     GIT_REPOSITORY  "https://github.com/nu-book/zxing-cpp.git"
#     GIT_TAG         "v1.1.1"
# )

SET(FETCHCONTENT_QUIET OFF)

# -----------------------------

SET(CMAKE_FIND_LIBRARY_SUFFIXES ".a")

SET(PEONY_THIRD_LIBRARIES stdc++fs
    ${CMAKE_THREAD_LIBS_INIT} 
    ${OPENGL_gl_LIBRARY} 
    ${CONAN_LIBS}
)
SET(PEONY_IMGUI_SOURCES imgui_impl_glfw.cpp imgui_impl_opengl3.cpp)

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
    ${PEONTY_HEADERS}
)

include(CTest)
enable_testing()

add_executable(peony-test src/peony-test.cpp ${PEONY_SOURCES} ${PEONY_IMGUI_SOURCES})
target_link_libraries(peony-test ${PEONY_THIRD_LIBRARIES})