cmake_minimum_required(VERSION 2.8)

project(nng_ipc)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_C_FLAGS  "${CMAKE_C_FLAGS} -Wall -g -Werror -fsanitize=address")

# Check own repository URL
execute_process (
  OUTPUT_VARIABLE var
  COMMAND bash -c "git -C ${CMAKE_SOURCE_DIR} config --get remote.origin.url"
)
# Set own URL to distinguish between external repos and this repo
set(DEP_REPO_URL ${var})

# ================================ NNG ==================================== #
# Download and unpack nng at configure
if (NOT HAS_NNG)
  message("  ------------------ ----------------- ------------------")
  message("  ------------------ Configuring NNG ------------------")
  message("  ------------------ ----------------- ------------------")

  # Check if the repository has been downloaded already
  execute_process (
    RESULT_VARIABLE result
    OUTPUT_VARIABLE var
    # Check URL of downloaded repository
    COMMAND bash -c "git -C ${CMAKE_SOURCE_DIR}/nng config --get \
            remote.origin.url"
  )
  # Above command returns the URL of "dependencies" if folder exists but does
  # not contain a valid repository itself
  if (result OR ("${var}" STREQUAL ${DEP_REPO_URL}))
    set(DOWNLOAD_NNG 1)
  endif()

  configure_file(
    ${CMAKE_SOURCE_DIR}/cmake/CMakeLists.txt.nng
    ${CMAKE_BINARY_DIR}/nng-download/CMakeLists.txt
  )

  execute_process(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" ${TOOLCHAIN_FILE_OPTION} .
    RESULT_VARIABLE result
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/nng-download
  )
  if (result)
    message(FATAL_ERROR "CMake step for nng failed: ${result}")
  endif()

  execute_process(COMMAND ${CMAKE_COMMAND} --build .
    RESULT_VARIABLE result
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/nng-download
  )
  if (result)
    message(FATAL_ERROR "Build step for nng failed: ${result}")
  endif()

  set(HAS_NNG ON CACHE BOOL "Have we downloaded nng?")
endif ()

# ======================================================================= #

include_directories(${CMAKE_SOURCE_DIR}/nng/include)

add_library(heartbeat heartbeat.c)
target_link_libraries(heartbeat
    ${CMAKE_BINARY_DIR}/nng-build/libnng.a
    pthread
    )

add_executable(publisher publisher.c)
target_link_libraries(publisher
    ${CMAKE_BINARY_DIR}/nng-build/libnng.a
    pthread
    heartbeat
)

add_executable(listener listener.c)
target_link_libraries(listener
    ${CMAKE_BINARY_DIR}/nng-build/libnng.a
    pthread
    heartbeat
)
