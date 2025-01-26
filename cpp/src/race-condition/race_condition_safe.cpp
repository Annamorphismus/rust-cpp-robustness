#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm verhindert eine Race Condition, indem der Zugriff auf den gemeinsamen
 * Zähler (`counter`) durch einen `std::mutex` synchronisiert wird. Dadurch wird
 * sichergestellt, dass immer nur ein Thread gleichzeitig den Zähler inkrementieren kann.
 * Ergebnis:
 * Der Zähler erreicht den erwarteten Wert von `10000`, da alle Inkremente korrekt
 * synchronisiert und keine Daten verloren gehen.
 */

void increment_counter_sync(int& counter, std::mutex& counter_mutex)
{
    for (int i = 0; i < 1000; ++i) {
        std::lock_guard<std::mutex> lock(counter_mutex); // Schutz des kritischen Abschnitts
        ++counter;
    }
}

void prevent_race_condition()
{
    int counter = 0; // Lokaler Zähler
    std::mutex counter_mutex;
    std::vector<std::thread> threads;

    for (int i = 0; i < 10; ++i) {
        threads.emplace_back(increment_counter_sync, std::ref(counter),
                             std::ref(counter_mutex)); // Counter und Mutex als Referenzen
    }

    for (auto& thread : threads) {
        thread.join();
    }

    std::cout << "Erwarteter Zähler: 10000\n";
    std::cout << "Tatsächlicher Zähler: " << counter << std::endl;
}

int main()
{
    prevent_race_condition();
    return 0;
}
