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

# Optionale Tests hinzufügen (falls in cpp/tests/)
#enable_testing()
#add_subdirectory(tests)
