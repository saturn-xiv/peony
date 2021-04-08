file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/include/peony/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/src/*.cc")

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
configure_file(include/peony/config.h.in config.h)

add_library(peony ${SOURCE_LIST} ${HEADER_LIST})

target_include_directories(peony PUBLIC ${CMAKE_CURRENT_BINARY_DIR} ../include)

target_link_libraries(peony PRIVATE 
  ${CMAKE_THREAD_LIBS_INIT}
  ${Boost_LIBRARIES}
  unofficial::sqlite3::sqlite3
  eclipse-paho-mqtt-c::paho-mqtt3c-static
  # Poco::Net Poco::XML Poco::Zip Poco::JSON Poco::Data
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
