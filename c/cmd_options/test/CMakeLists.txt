cmake_minimum_required(VERSION 2.8)

project(program_options_test)

set(MAJOR 1)
set(MINOR 0)
set(REV 0)

include_directories(../include .)

add_executable(prog_option_testrunner testrunner.cpp
    ../src/program_options.c
)

target_compile_definitions(prog_option_testrunner PRIVATE VERSION="${MAJOR}.${MINOR}.${REV}")
target_link_libraries(prog_option_testrunner
    gtest
    pthread
)
