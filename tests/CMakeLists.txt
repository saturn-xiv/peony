add_executable(jwt-test jwt.cc)
target_link_libraries(jwt-test PRIVATE peony)
set_target_properties(jwt-test PROPERTIES LINK_FLAGS_RELEASE -s)

add_executable(tty-test tty.cc)
target_link_libraries(tty-test PRIVATE peony)
set_target_properties(tty-test PROPERTIES LINK_FLAGS_RELEASE -s)
