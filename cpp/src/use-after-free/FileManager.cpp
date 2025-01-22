// FileManager.cpp
#include "FileManager.h"
#include <fstream>
#include <iostream>

FileManager::FileManager()
{
    // Konstruktor
}

FileManager::~FileManager()
{
    // Alle Dateien werden automatisch geschlossen, wenn shared_ptr
    // zerstört werden
    std::lock_guard<std::mutex> lock(cacheMutex);
    fileCache.clear();
}

std::shared_ptr<std::fstream> FileManager::openFile(const std::string& filename,
                                                    std::ios::openmode mode)
{
    std::lock_guard<std::mutex> lock(cacheMutex);

    // Überprüfen, ob die Datei bereits im Cache ist
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        if (it->second->is_open()) {
            std::cout << "Datei " << filename << " bereits geöffnet." << std::endl;
            return it->second;
        } else {
            // Wenn die Datei im Cache ist, aber nicht geöffnet, wird diese entfernt
            fileCache.erase(it);
        }
    }

    // Datei öffnen
    auto filePtr = std::make_shared<std::fstream>(filename, mode);
    if (filePtr->is_open()) {
        fileCache[filename] = filePtr;
        std::cout << "Datei " << filename << " geöffnet und zum Cache hinzugefügt." << std::endl;
        return filePtr;
    } else {
        std::cerr << "Fehler beim Öffnen der Datei " << filename << "." << std::endl;
        return nullptr;
    }
}

std::string FileManager::readFile(const std::string& filename)
{
    std::lock_guard<std::mutex> lock(cacheMutex);
    auto it = fileCache.find(filename);
    if (it != fileCache.end() && it->second->is_open()) {
        std::string content;
        std::getline(*(it->second), content);
        return content;
    } else {
        std::cerr << "Datei " << filename << " ist nicht geöffnet oder existiert nicht im Cache."
                  << std::endl;
        return "";
    }
}

void FileManager::closeFile(const std::string& filename)
{
    std::lock_guard<std::mutex> lock(cacheMutex);

    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        if (it->second->is_open()) {
            it->second->close();
            std::cout << "Datei " << filename << " geschlossen." << std::endl;
        }
        fileCache.erase(it);
        std::cout << "Datei " << filename << " aus dem Cache entfernt." << std::endl;
    } else {
        std::cerr << "Datei " << filename << " ist nicht im Cache vorhanden." << std::endl;
    }
}
std::fstream* FileManager::getFilePointer(const std::string& filename)
{
    std::lock_guard<std::mutex> lock(cacheMutex);
    auto it = fileCache.find(filename);
    if (it != fileCache.end() && it->second->is_open()) {
        return it->second.get();
    } else {
        std::cerr << "Datei" << filename << "ist nicht geöffnet oder nicht im Cache vorhanden."
                  << std::endl;
        return nullptr;
    }
}
