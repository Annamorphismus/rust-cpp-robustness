#pragma once

#include <fstream>
#include <memory>
#include <mutex>
#include <string>
#include <unordered_map>

class FileManager
{
public:
    FileManager();
    ~FileManager();

    // Öffnet eine Datei und fügt sie dem Cache hinzu
    std::shared_ptr<std::fstream> openFile(const std::string& filename, std::ios::openmode mode);

    // Liest den Inhalt einer Datei
    std::string readFile(const std::string& filename);

    // Schließt eine Datei und entfernt sie aus dem Cache
    void closeFile(const std::string& filename);

    // Sicherer Rückgabewert: Gibt einen `std::weak_ptr` zurück
    std::weak_ptr<std::fstream> getFilePointer(const std::string& filename);

private:
    // Jetzt speichert der Cache `std::weak_ptr`, nicht nur `std::shared_ptr`
    std::unordered_map<std::string, std::weak_ptr<std::fstream>> fileCache;

    // Mutex zur Sicherstellung der Thread-Sicherheit
    std::mutex cacheMutex;
};
