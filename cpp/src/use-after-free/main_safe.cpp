#include "FileManager_safe.h"
#include <chrono>
#include <iostream>
#include <thread>

int main()
{
    FileManager manager;
    std::string filename = "example.txt";

    // Datei erzeugen
    {
        std::ofstream outfile(filename);
        outfile << "Zeile 1\nZeile 2\nZeile 3\n";
    }

    // Datei öffnen
    std::shared_ptr<std::fstream> filePtr =
        manager.openFile(filename, std::ios::in | std::ios::out);
    if (!filePtr) {
        std::cerr << "Datei konnte nicht geöffnet werden." << std::endl;
        return 1;
    }

    // Weak-Pointer für sicheren Zugriff auf die Datei anlegen
    std::weak_ptr<std::fstream> weakFilePtr = manager.getFilePointer(filename);

    // Leser-Thread: Wartet 100 ms, dann versucht er eine Zeile aus der Datei zu lesen
    std::thread reader([&]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        manager.readFile(filename); // Kann fehlschlagen, wenn die Datei geschlossen wurde
    });

    // Schreiber-Thread: Wartet 50 ms, dann schließt er die Datei
    std::thread writer([&]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
        manager.closeFile(filename); // Datei wird aus dem Cache entfernt und geschlossen
    });

    // Warten auf die Beendigung der Threads
    reader.join();
    writer.join();

    // Sicherer Zugriff auf die Datei mittels weak_ptr
    if (auto validFile = weakFilePtr.lock()) { // Prüfen, ob Datei noch existiert
        std::string content;
        std::getline(*validFile, content); // Erste Zeile aus der Datei lesen
        std::cout << "Gelesener Inhalt: " << content << std::endl;
    } else {
        std::cerr << "Die Datei wurde bereits freigegeben." << std::endl;
    }

    // Datei aus dem Dateisystem entfernen (Cleanup)
    std::remove(filename.c_str());
    return 0;
}

// Datei-Cache mit std::unordered_map aktualisieren
std::unordered_map<std::string, std::weak_ptr<std::fstream>> fileCache;
