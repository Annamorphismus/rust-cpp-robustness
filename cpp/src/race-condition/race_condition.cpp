#include <chrono>
#include <cstdlib>
#include <iostream>
#include <thread>
#include <vector>

int counter = 0;

// Funktion, die den Zähler inkrementiert
void increment_counter()
{
    for (int i = 0; i < 1000; ++i) {
        ++counter; // Kritischer Abschnitt
        std::this_thread::sleep_for(
            std::chrono::microseconds(rand() % 10)); // Zufällige Verzögerung
    }
}

void simulate_race_condition()
{
    std::vector<std::thread> threads; // Container für die Threads

    // 10 Threads erzeugen und starten
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back(increment_counter); // Threads hinzufügen
    }

    // Warten, bis alle Threads beendet sind
    for (auto& thread : threads) {
        thread.join(); // Join auf jeden Thread im Vector
    }

    // Ergebnis ausgeben
    std::cout << "Erwarteter Zähler: 10000\n"; // 10 Threads x 1000 Inkremente
    std::cout << "Tatsächlicher Zähler: " << counter << std::endl;
}

int main() { simulate_race_condition(); }