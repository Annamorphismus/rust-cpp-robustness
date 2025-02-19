cmake_minimum_required(VERSION 3.25)
project(bufferoverflow)

# Setze C++-Standard auf 23
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Standard-Build-Typ setzen, falls keiner angegeben wurde
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Choose Debug or Release" FORCE)
endif()

# Include-Ordner hinzufügen
include_directories(${CMAKE_SOURCE_DIR}/include)

# Debug Build Konfiguration mit AddressSanitizer (ASAN)
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -O0 -fno-omit-frame-pointer -fno-stack-protector -no-pie")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -no-pie")
endif()

# Release Build Konfiguration für Exploit
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -no-pie -fno-stack-protector -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=0 -g -O0 -z execstack -z norelro")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -no-pie -z execstack -z norelro")
endif()

# Aktivierung von Compiler-Warnungen für bessere Code-Qualität
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic")

# Erstelle ein Ziel für den Client
add_executable(bufferoverflow_client
    client.cpp
)

# Erstelle ein Ziel für den Server
add_executable(bufferoverflow_server
    server.cpp
)

# Füge den zentralen Include-Ordner hinzu
target_include_directories(bufferoverflow_client PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_include_directories(bufferoverflow_server PUBLIC ${CMAKE_SOURCE_DIR}/include)

# Falls Threads verwendet werden, muss pthread gelinkt werden
# target_link_libraries(bufferoverflow_server pthread)
# target_link_libraries(bufferoverflow_client pthread)
