add_executable(jwt-test jwt.cc)
target_include_directories(jwt-test PUBLIC ${CMAKE_CURRENT_BINARY_DIR} ${Boost_INCLUDE_DIRS} ../include)
target_link_libraries(jwt-test PRIVATE peony)
set_target_properties(jwt-test PROPERTIES LINK_FLAGS_RELEASE -s)
