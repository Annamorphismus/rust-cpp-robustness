cmake_minimum_required(VERSION 3.25)
project(race-condition)

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

# Füge ein ausführbares Ziel für race_condition.cpp hinzu
add_executable(race_condition race_condition.cpp)

# Füge ein ausführbares Ziel für sync.cpp hinzu
add_executable(race_condition_safe race_condition_safe.cpp)

# Beide ausführbaren Dateien verwenden denselben Include-Ordner
target_include_directories(race_condition PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_include_directories(race_condition_safe PUBLIC ${CMAKE_SOURCE_DIR}/include)
