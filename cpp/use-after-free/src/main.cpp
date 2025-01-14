
#include "FileManager.h"
#include <chrono>
#include <cstring> // Für std::strcpy
#include <iostream>
#include <thread>

int main()
{
    FileManager manager;
    std::string filename = "example.txt";

    // Erstellen einer Beispieldatei
    {
        std::ofstream outfile(filename);
        outfile << "Zeile 1\nZeile 2\nZeile 3\n";
    }

    // Öffnen der Datei und Erhalten eines rohen Zeigers
    std::shared_ptr<std::fstream> filePtr =
        manager.openFile(filename, std::ios::in | std::ios::out);
    if (!filePtr) {
        std::cerr << "Datei konnte nicht geöffnet werden." << std::endl;
        return 1;
    }

    std::fstream* rawPtr = manager.getFilePointer(filename);
    if (!rawPtr) {
        std::cerr << "Konnte keinen rohen Zeiger auf die Datei erhalten." << std::endl;
        return 1;
    }

    // Starten eines Threads, der auf die Datei zugreift
    std::thread reader([&manager, &filename]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        manager.readFile(filename);
    });

    // Starten eines Threads, der die Datei schließt und aus dem Cache entfernt
    std::thread writer([&manager, &filename]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
        manager.closeFile(filename);
    });

    reader.join();
    writer.join();

    // Objekt löschen, wodurch der `shared_ptr` und somit die Datei freigegeben wird
    filePtr.reset();

    // Versuch, auf den freigegebenen Zeiger zuzugreifen (Use-After-Free)
    std::cout << "Versuche auf den freigegebenen Zeiger zuzugreifen:" << std::endl;

    // Exploit: Überschreiben des freigegebenen Speichers
    if (rawPtr != nullptr) {
        // Achtung: Dies ist absichtlich undefiniertes Verhalten und dient nur zu
        // Demonstrationszwecken!
        std::strcpy(reinterpret_cast<char*>(rawPtr), "Exploit!"); // Undefined Behavior
    }

    // Versuch, die manipulierten Daten zu lesen
    if (rawPtr != nullptr) {
        std::cout << "Manipulierte Daten aus dem freigegebenen Zeiger: " << rawPtr->is_open()
                  << std::endl;
        // Weitere undefinierte Operationen könnten folgen
        std::string content;
        std::getline(*rawPtr, content); // Undefined Behavior
        std::cout << "Gelesener Inhalt: " << content << std::endl;
    }

    // Entfernen der Beispieldatei
    std::remove(filename.c_str());

    return 0;
}
