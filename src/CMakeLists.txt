file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/include/peony/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${peony_SOURCE_DIR}/src/*.cc")

add_library(peony_library ${SOURCE_LIST} ${HEADER_LIST})

target_include_directories(peony_library PUBLIC ../include)

# target_link_libraries(modern_library PRIVATE Boost::boost)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST})
