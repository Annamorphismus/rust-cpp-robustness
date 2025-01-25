cmake_minimum_required(VERSION 3.25)
project(deadlocks)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Include den zentralen include-Ordner
include_directories(${CMAKE_SOURCE_DIR}/include)

# Debug Build Konfiguration
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address -g -O1 -fno-omit-frame-pointer -fno-stack-protector")
    set(CMAKE_LINKER_FLAGS "${CMAKE_LINKER_FLAGS} -fsanitize=address")
endif()

# Release Build Konfiguration
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -no-pie -fno-stack-protector -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=0 -g -O1")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -no-pie")
endif()

# Executable hinzufügen
add_executable(deadlocks deadlock.cpp)
add_executable(deadlocks_safe deadlock_safe.cpp)


# Ziel-spezifische Include-Pfade hinzufügen
target_include_directories(deadlocks PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_include_directories(deadlocks_safe PUBLIC ${CMAKE_SOURCE_DIR}/include)

