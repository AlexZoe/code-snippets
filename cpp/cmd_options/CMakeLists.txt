project(cmd_options)
cmake_minimum_required(VERSION 3.16)

find_package(Boost COMPONENTS
    program_options
)

file(GLOB SOURCES ${CMAKE_SOURCE_DIR}/src/*.cpp)
include_directories(${CMAKE_SOURCE_DIR}/include)

add_executable(prog
    ${SOURCES}
)
target_link_libraries(prog
    ${Boost_LIBRARIES}
)
