add_executable(jwt-test jwt.cc)

target_link_libraries(jwt PRIVATE peony_library)
