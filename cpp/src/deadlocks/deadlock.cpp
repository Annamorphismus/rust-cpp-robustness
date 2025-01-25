#include <chrono>
#include <fstream>
#include <iostream>
#include <mutex>
#include <string>
#include <thread>

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert ein System, in dem zwei Threads auf gemeinsame Ressourcen
 * (Konfigurationsdatei und Log-Datei) zugreifen. Durch die unterschiedliche Reihenfolge
 * beim Sperren der Dateien kann ein Deadlock entstehen, bei dem beide Threads gegenseitig
 * aufeinander warten und das Programm blockieren.
 */

std::mutex configFileMutex;
std::mutex logFileMutex;

// Funktion: Konfigurationsänderung
void updateConfig(const std::string& threadName)
{
    // Sperre die Konfigurationsdatei zuerst
    std::lock_guard<std::mutex> configLock(configFileMutex);
    std::cout << threadName << " hat die Konfigurationsdatei gesperrt.\n";

    // Schreiben in die Konfigurationsdatei
    std::ofstream configFile("config.txt", std::ios::app);
    if (configFile.is_open()) {
        configFile << threadName << " aktualisiert die Konfiguration.\n";
        configFile.close();
        std::cout << threadName << " hat die Konfiguration aktualisiert.\n";
    } else {
        std::cerr << threadName << " konnte die Konfigurationsdatei nicht öffnen!\n";
    }

    // Simulierte Verzögerung
    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    // Sperre die Log-Datei
    std::lock_guard<std::mutex> logLock(logFileMutex);
    std::ofstream logFile("log.txt", std::ios::app);
    if (logFile.is_open()) {
        logFile << threadName << " hat eine Konfigurationsänderung geloggt.\n";
        logFile.close();
        std::cout << threadName << " hat die Änderung im Log festgehalten.\n";
    } else {
        std::cerr << threadName << " konnte die Log-Datei nicht öffnen!\n";
    }
}

// Funktion: Fehlerprotokollierung
void logError(const std::string& threadName)
{
    // Sperre die Log-Datei zuerst
    std::lock_guard<std::mutex> logLock(logFileMutex);
    std::cout << threadName << " hat die Log-Datei gesperrt.\n";

    // Schreiben in die Log-Datei
    std::ofstream logFile("log.txt", std::ios::app);
    if (logFile.is_open()) {
        logFile << threadName << " protokolliert einen Fehler.\n";
        logFile.close();
        std::cout << threadName << " hat den Fehler protokolliert.\n";
    } else {
        std::cerr << threadName << " konnte die Log-Datei nicht öffnen!\n";
    }

    // Simulierte Verzögerung
    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    // Sperre die Konfigurationsdatei
    std::lock_guard<std::mutex> configLock(configFileMutex);
    std::ofstream configFile("config.txt", std::ios::app);
    if (configFile.is_open()) {
        configFile << threadName << " hat die Konfiguration überprüft.\n";
        configFile.close();
        std::cout << threadName << " hat die Konfigurationsdatei überprüft.\n";
    } else {
        std::cerr << threadName << " konnte die Konfigurationsdatei nicht öffnen!\n";
    }
}

int main()
{
    std::cout << "Programm gestartet.\n";

    // Zwei Threads starten
    std::thread thread1(updateConfig, "Thread 1");
    std::thread thread2(logError, "Thread 2");

    // Auf die Beendigung der Threads warten
    thread1.join();
    thread2.join();

    std::cout << "Programm beendet.\n";
    return 0;
}
