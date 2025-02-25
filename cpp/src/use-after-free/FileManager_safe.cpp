#include "FileManager_safe.h"
#include <fstream>
#include <iostream>
#include <unordered_map>

//----------------------------------Szenario------------------------------------------
/*
 * Sichere Version zur Vermeidung von Use-After-Free:
 * - Statt eines rohen Zeigers wird std::weak_ptr zurückgegeben.
 * - Dadurch kann überprüft werden, ob die Datei noch existiert, bevor auf sie zugegriffen wird.
 */

// Konstruktor
FileManager::FileManager() {}

// Destruktor
FileManager::~FileManager()
{
    std::lock_guard<std::mutex> lock(cacheMutex);
    fileCache.clear(); // Entfernt alle gespeicherten Dateien sicher aus dem Cache
}

// Öffnet eine Datei und speichert sie sicher im Cache
std::shared_ptr<std::fstream> FileManager::openFile(const std::string& filename,
                                                    std::ios::openmode mode)
{
    std::lock_guard<std::mutex> lock(cacheMutex);
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        if (auto validFile = it->second.lock()) { // Überprüfen, ob das File noch gültig
            return validFile;
        }
        fileCache.erase(it); // Entferne nicht mehr gültige Einträge aus dem Cache
    }
    // Neues shared_ptr-Objekt für die Datei erstellen
    auto filePtr = std::make_shared<std::fstream>(filename, mode);
    if (filePtr->is_open()) {
        fileCache[filename] = filePtr; // Datei in den Cache aufnehmen
        return filePtr;
    }
    return nullptr; // Datei konnte nicht geöffnet werden
}

// Liest eine Zeile sicher aus der Datei
std::string FileManager::readFile(const std::string& filename)
{
    std::lock_guard<std::mutex> lock(cacheMutex); // Schutz vor parallelen Zugriffen
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        if (auto validFile = it->second.lock()) { // Prüft, ob die Datei noch existiert
            std::string content;
            std::getline(*validFile, content); // Liest eine Zeile aus der Datei
            return content;
        }
    }
    std::cerr << "Die Datei wurde bereits freigegeben oder ist nicht vorhanden." << std::endl;
    return "";
}

// Schließt und entfernt eine Datei sicher aus dem Cache
void FileManager::closeFile(const std::string& filename)
{
    std::lock_guard<std::mutex> lock(cacheMutex);
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        if (auto validFile = it->second.lock()) {
            if (validFile->is_open()) {
                validFile->close();
            }
        }
        fileCache.erase(it);
    }
}

// Gibt einen sicheren Zeiger auf eine Datei zurück
std::weak_ptr<std::fstream> FileManager::getFilePointer(const std::string& filename)
{
    std::lock_guard<std::mutex> lock(cacheMutex);
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        return it->second;
    }
    return std::weak_ptr<std::fstream>();
}
