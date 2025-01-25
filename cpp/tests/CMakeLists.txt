# Setze den C++-Standard
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Finde Google Test
find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

# Testziel erstellen
add_executable(run_tests
    main.cpp
   # unittest.cpp
    ../src/race-condition/race_condition.cpp
    ../src/race-condition/sync.cpp
)

# Google Test-Bibliotheken und pthread hinzufügen
target_link_libraries(run_tests ${GTEST_LIBRARIES} pthread)

# Tests in den Build-Prozess integrieren
enable_testing()
add_test(NAME UnitTests COMMAND run_tests)
