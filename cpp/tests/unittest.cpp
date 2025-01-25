#include "../include/race_condition.h"
#include "../include/sync.h"
#include <gtest/gtest.h>
#include <mutex>

TEST(CounterTest, IncrementWithoutSync)
{
    int counter = 0;

    // Simuliere ohne Synchronisation
    increment_counter(counter);

    EXPECT_LT(counter, 10000); // Erwarte, dass der Zähler kleiner als 10000 ist
}

TEST(CounterTest, IncrementWithSync)
{
    int counter = 0;
    std::mutex counter_mutex;

    // Simuliere mit Synchronisation
    increment_counter_sync(counter, counter_mutex);

    EXPECT_EQ(counter, 10000); // Erwarte, dass der Zähler 10000 ist
}
