project(mock_db)
cmake_minimum_required(VERSION 3.16)

file(GLOB SOURCES ${CMAKE_SOURCE_DIR}/*.c*)
include_directories(${CMAKE_SOURCE_DIR}/)

add_executable(prog
    ${SOURCES}
)
