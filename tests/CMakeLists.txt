add_executable(jwt-test jwt.cc)
target_link_libraries(jwt-test PRIVATE peony)
set_target_properties(jwt-test PROPERTIES LINK_FLAGS_RELEASE -s)
