cmake_minimum_required(VERSION 3.25)
project(memory-leak)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Füge den include-Ordner hinzu
include_directories(include)

# Konfiguration für Debug Build mit ASAN
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address -g -O1 -fno-omit-frame-pointer -fno-stack-protector")
    set(CMAKE_LINKER_FLAGS "${CMAKE_LINKER_FLAGS} -fsanitize=address")
endif()

# Konfiguration für Release Build ohne ASAN (Exploit)
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -no-pie -fno-stack-protector -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=0 -g -O1")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -no-pie")
endif()



# Add executable for memory-leak
add_executable(memory_leak memory_leak.cpp)

# Add executable for memory-leak-safe
add_executable(memory_leak_safe memory_leak_safe.cpp)

target_include_directories(memory_leak PUBLIC ${CMAKE_SOURCE_DIR}/include)
