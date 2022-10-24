cmake_minimum_required(VERSION 2.8)

project(program-options)

set(MAJOR 1)
set(MINOR 0)
set(REV 0)

include_directories(program-options include)

add_library(program-options STATIC src/program_options.c)
target_compile_definitions(program-options PRIVATE VERSION="${MAJOR}.${MINOR}.${REV}")

add_executable(main main.c)
target_link_libraries(main
    program-options
)
include_directories(include)
