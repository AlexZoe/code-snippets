project(cmd_options)
cmake_minimum_required(VERSION 3.16)

find_package(Boost COMPONENTS
    program_options
)

find_package(GTest CONFIG REQUIRED)

file(GLOB SOURCES ${CMAKE_SOURCE_DIR}/src/*.cpp ${CMAKE_SOURCE_DIR}/unit_tests/*.cpp)
list(REMOVE_ITEM SOURCES ${CMAKE_SOURCE_DIR}/src/main.cpp)
include_directories(${CMAKE_SOURCE_DIR}/include)
include_directories(${GTEST_INCLUDE_DIRS})

add_executable(test_runner
    ${SOURCES}
)
target_link_libraries(test_runner
    ${Boost_LIBRARIES}
    gtest
    pthread
)
