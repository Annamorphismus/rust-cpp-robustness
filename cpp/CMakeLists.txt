cmake_minimum_required(VERSION 3.25)
project(CPPProject)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Globale Include-Verzeichnisse
include_directories(include)

add_subdirectory(src/bufferoverflow)

add_subdirectory(src/deadlocks)

add_subdirectory(src/race-condition)

add_subdirectory(src/use-after-free)
add_subdirectory(src/memory-leak)



add_custom_target(vclean
    COMMAND ${CMAKE_COMMAND} --build . --target clean
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    COMMENT "Bereinige das Projekt..."
)
