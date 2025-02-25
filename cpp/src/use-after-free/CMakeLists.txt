cmake_minimum_required(VERSION 3.25)
project(use-after-free)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS OFF)


# Include den zentralen include-Ordner
include_directories(${CMAKE_SOURCE_DIR}/include)

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

add_executable(use-after-free
    main.cpp
    FileManager.cpp

)

add_executable(use-after-free-safe
    main_safe.cpp
    FileManager_safe.cpp
)

target_include_directories(use-after-free PUBLIC ${CMAKE_SOURCE_DIR}/include)

target_include_directories(use-after-free-safe PUBLIC ${CMAKE_SOURCE_DIR}/include)