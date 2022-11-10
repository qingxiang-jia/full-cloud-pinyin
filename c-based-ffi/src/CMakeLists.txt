set(CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 11)

add_executable(main main.c c_api.c)
target_link_libraries(main "${CMAKE_SOURCE_DIR}/target/release/librust_api.a")