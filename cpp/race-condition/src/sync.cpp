#include "sync.h"
#include "race_condition.h"
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

// Globale Variable
std::mutex counter_mutex;

// Funktion, die den Zähler inkrementiert (mit Synchronisation)
void increment_counter_sync()
{
    for (int i = 0; i < 1000; ++i) {
        std::lock_guard<std::mutex> lock(counter_mutex); // Schutz des kritischen Abschnitts
        ++counter;
    }
}

void prevent_race_condition()
{
    std::vector<std::thread> threads; // Container für die Threads

    // 10 Threads erzeugen und starten
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back(increment_counter_sync); // Threads hinzufügen
    }

    // Warten, bis alle Threads beendet sind
    for (auto& thread : threads) {
        thread.join(); // Join auf jeden Thread im Vector
    }

    // Ergebnis ausgeben
    std::cout << "Erwarteter Zähler: 10000\n"; // 10 Threads x 1000 Inkremente
    std::cout << "Tatsächlicher Zähler: " << counter << std::endl;
}
