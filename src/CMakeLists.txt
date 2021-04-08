file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/include/peony/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/src/*.cc")


add_library(peony ${SOURCE_LIST} ${HEADER_LIST})

target_include_directories(peony PUBLIC ${CMAKE_CURRENT_BINARY_DIR} ../include)

target_link_libraries(peony PRIVATE
  stdc++fs
  ${CMAKE_THREAD_LIBS_INIT}
  ${Boost_LIBRARIES}
  unofficial::sqlite3::sqlite3
  # eclipse-paho-mqtt-c::paho-mqtt3c-static  
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
