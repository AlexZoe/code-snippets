project(cmd_options)
cmake_minimum_required(VERSION 3.16)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Boost COMPONENTS
    program_options
)

file(GLOB SOURCES ${CMAKE_SOURCE_DIR}/src/*.cpp)
include_directories(${CMAKE_SOURCE_DIR}/include)

add_executable(id_parser
    ${SOURCES}
)
target_link_libraries(id_parser
    ${Boost_LIBRARIES}
)

add_subdirectory(unit_tests)
