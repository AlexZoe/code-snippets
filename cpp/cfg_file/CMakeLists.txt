cmake_minimum_required(VERSION 2.8)

project(ConfigFile)

#add_compile_options(-Wall -std=c++17 -fsanitize=address)
SET(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall -std=c++17 -fsanitize=address")

add_executable(run main.cpp)
target_link_libraries(run
    config++
)
