add_executable(lilium main.cc)
target_link_libraries(lilium PRIVATE peony ${CONAN_LIBS})
set_target_properties(lilium PROPERTIES LINK_FLAGS_RELEASE -s)
