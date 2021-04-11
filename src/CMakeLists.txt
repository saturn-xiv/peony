FetchContent_Declare(
  googletest
  GIT_REPOSITORY  "https://github.com/google/googletest.git"
  GIT_TAG         "release-1.10.0"
)
FetchContent_MakeAvailable(googletest)
FetchContent_Declare(
  cpp_jwt
  GIT_REPOSITORY    "https://github.com/arun11299/cpp-jwt.git"
  GIT_TAG           "v1.4"
)
FetchContent_MakeAvailable(cpp_jwt)

file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/include/peony/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/src/*.cc")


add_library(peony ${SOURCE_LIST} ${HEADER_LIST})

target_include_directories(peony PUBLIC 
  ${CMAKE_CURRENT_BINARY_DIR} 
  ${Boost_INCLUDE_DIRS} 
  ${cpp_jwt_SOURCE_DIR}/include
  
  ../include
)

target_link_libraries(peony PRIVATE
  stdc++fs
  ${CMAKE_THREAD_LIBS_INIT}
  ${Boost_LIBRARIES}
  OpenSSL::SSL OpenSSL::Crypto
  unofficial-sodium::sodium
  cppzmq-static
  # unofficial::sqlite3::sqlite3
  # eclipse-paho-mqtt-c::paho-mqtt3c-static  
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
