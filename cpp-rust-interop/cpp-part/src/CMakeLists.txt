add_executable(program program.cc)

add_library(RustLib STATIC IMPORTED)
set_target_properties(RustLib PROPERTIES IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/../rust-part/target/release/librust_part.a)

target_link_libraries(program PRIVATE RustLib)

# target_link_libraries(program ${CMAKE_SOURCE_DIR}../../rust-part/target/release/librust_part.a)
# target_include_directories(program PUBLIC "../../rust-part/include")