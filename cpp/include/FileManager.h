
#pragma once

#include <string>
#include <fstream>
#include <memory>
#include <mutex>
#include <unordered_map>

class FileManager {
public:
    FileManager();
    ~FileManager();

    // Öffnet eine Datei und fügt sie dem Cache hinzu
    std::shared_ptr<std::fstream> openFile(const std::string& filename, std::ios::openmode mode);

    // Liest den Inhalt einer Datei
    std::string readFile(const std::string& filename);

    // Schließt eine Datei und entfernt sie aus dem Cache
    void closeFile(const std::string& filename);

    // Gibt einen rohen Zeiger auf die fstream zurück (gefährlich!)
    std::fstream* getFilePointer(const std::string& filename);

private:
    // Cache zur Speicherung von Dateiobjekten
    std::unordered_map<std::string, std::shared_ptr<std::fstream>> fileCache;

    // Mutex zur Sicherstellung der Thread-Sicherheit
    std::mutex cacheMutex;
};
