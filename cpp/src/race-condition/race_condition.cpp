#include <chrono>
#include <cstdlib>
#include <iostream>
#include <thread>
#include <vector>

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert eine Race Condition, indem mehrere Threads gleichzeitig
 * auf eine globale Variable (`counter`) zugreifen und diese inkrementieren.
 * Der Zugriff auf den `counter` erfolgt ohne Synchronisation, wodurch Dateninkonsistenzen
 * entstehen können, wenn mehrere Threads gleichzeitig auf die Variable zugreifen.
 */
//----------------------------------Ablauf------------------------------------------
/*
 * - 10 Threads werden gestartet, die jeweils 1000-mal den `counter` inkrementieren.
 * - Eine zufällige Verzögerung (`rand() % 10`) simuliert reale Bedingungen, in denen
 *   Threads asynchron arbeiten und sich gegenseitig überschreiben können.
 */
//----------------------------------Ergebnis------------------------------------------
/*
 * - Der erwartete Zählerwert ist 10000 (10 Threads × 1000 Inkremente).
 * - Aufgrund der Race Condition erreicht der `counter` jedoch oft einen inkorrekten Wert,
 *   da Threads gleichzeitig auf die Variable zugreifen und Inkremente verloren gehen.
 */

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