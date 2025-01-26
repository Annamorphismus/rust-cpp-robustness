#include "FileManager.h"
#include <fstream>
#include <iostream>

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Szenario demonstriert ein Use-After-Free-Problem mithilfe eines FileManagers:
 * Der FileManager verwaltet geöffnete Dateien in einem Cache, wobei Dateien mit
 * `std::shared_ptr` freigegeben werden. Ein roher Zeiger (`rawPtr`) wird auf eine Datei
 * zurückgegeben, die später aus dem Cache entfernt und freigegeben wird.
 *
 * **Exploit:**
 * Nach der Freigabe der Ressource wird der Speicherbereich des rohen Zeigers überschrieben
 * (`std::strcpy`), was ein klassisches Undefined Behavior erzeugt. Angreifer könnten diesen
 * Zustand ausnutzen, um den Speicher gezielt zu manipulieren, etwa um Schadcode einzuschleusen
 * oder sensible Daten zu lesen.
 */

// Konstruktor für die Klasse FileManager
// Initialisiert die Klasse, aber keine spezifischen Ressourcen.
FileManager::FileManager()
{
    // Konstruktor
}

// Destruktor für die Klasse FileManager
// Schließt alle Dateien, indem der Cache geleert wird.
// Die Verwendung von std::shared_ptr sorgt dafür, dass alle Ressourcen automatisch freigegeben
// werden.
FileManager::~FileManager()
{
    // Sichert den Zugriff auf den Datei-Cache (Thread-Sicherheit).
    std::lock_guard<std::mutex> lock(cacheMutex);
    fileCache.clear(); // Löscht alle Einträge im Cache.
}

// Öffnet eine Datei mit dem gegebenen Dateinamen und Modus.
// Gibt einen std::shared_ptr<std::fstream> zurück, der die Datei repräsentiert.
std::shared_ptr<std::fstream> FileManager::openFile(const std::string& filename,
                                                    std::ios::openmode mode)
{
    // Sichert den Zugriff auf den Datei-Cache (Thread-Sicherheit).
    std::lock_guard<std::mutex> lock(cacheMutex);

    // Überprüfen, ob die Datei bereits im Cache vorhanden ist.
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        // Wenn die Datei geöffnet ist, wird sie direkt zurückgegeben.
        if (it->second->is_open()) {
            std::cout << "Datei " << filename << " bereits geöffnet." << std::endl;
            return it->second;
        } else {
            // Falls die Datei im Cache vorhanden ist, aber geschlossen wurde,
            // wird der Eintrag aus dem Cache entfernt.
            fileCache.erase(it);
        }
    }

    // Öffnet die Datei und erstellt ein std::shared_ptr<std::fstream>.
    auto filePtr = std::make_shared<std::fstream>(filename, mode);
    if (filePtr->is_open()) {
        // Fügt die Datei dem Cache hinzu, wenn sie erfolgreich geöffnet wurde.
        fileCache[filename] = filePtr;
        std::cout << "Datei " << filename << " geöffnet und zum Cache hinzugefügt." << std::endl;
        return filePtr;
    } else {
        // Gibt eine Fehlermeldung aus, wenn die Datei nicht geöffnet werden konnte.
        std::cerr << "Fehler beim Öffnen der Datei " << filename << "." << std::endl;
        return nullptr; // Gibt einen leeren shared_ptr zurück.
    }
}

// Liest die erste Zeile der angegebenen Datei und gibt sie als std::string zurück.
std::string FileManager::readFile(const std::string& filename)
{
    // Sichert den Zugriff auf den Datei-Cache (Thread-Sicherheit).
    std::lock_guard<std::mutex> lock(cacheMutex);

    // Überprüfen, ob die Datei im Cache existiert und geöffnet ist.
    auto it = fileCache.find(filename);
    if (it != fileCache.end() && it->second->is_open()) {
        std::string content;
        // Liest eine Zeile aus der Datei.
        std::getline(*(it->second), content);
        return content; // Gibt den gelesenen Inhalt zurück.
    } else {
        // Gibt eine Fehlermeldung aus, wenn die Datei nicht geöffnet oder nicht im Cache ist.
        std::cerr << "Datei " << filename << " ist nicht geöffnet oder existiert nicht im Cache."
                  << std::endl;
        return ""; // Gibt einen leeren String zurück.
    }
}

// Schließt die angegebene Datei und entfernt sie aus dem Cache.
void FileManager::closeFile(const std::string& filename)
{
    // Sichert den Zugriff auf den Datei-Cache (Thread-Sicherheit).
    std::lock_guard<std::mutex> lock(cacheMutex);

    // Überprüfen, ob die Datei im Cache vorhanden ist.
    auto it = fileCache.find(filename);
    if (it != fileCache.end()) {
        if (it->second->is_open()) {
            // Schließt die Datei, wenn sie geöffnet ist.
            it->second->close();
            std::cout << "Datei " << filename << " geschlossen." << std::endl;
        }
        // Entfernt die Datei aus dem Cache.
        fileCache.erase(it);
        std::cout << "Datei " << filename << " aus dem Cache entfernt." << std::endl;
    } else {
        // Gibt eine Fehlermeldung aus, wenn die Datei nicht im Cache ist.
        std::cerr << "Datei " << filename << " ist nicht im Cache vorhanden." << std::endl;
    }
}

// Gibt einen rohen Zeiger auf die Datei zurück, wenn sie geöffnet und im Cache vorhanden ist.
std::fstream* FileManager::getFilePointer(const std::string& filename)
{
    // Sichert den Zugriff auf den Datei-Cache (Thread-Sicherheit).
    std::lock_guard<std::mutex> lock(cacheMutex);

    // Überprüfen, ob die Datei im Cache existiert und geöffnet ist.
    auto it = fileCache.find(filename);
    if (it != fileCache.end() && it->second->is_open()) {
        return it->second.get(); // Gibt den rohen Zeiger auf das std::shared_ptr-Objekt zurück.
    } else {
        // Gibt eine Fehlermeldung aus, wenn die Datei nicht geöffnet oder nicht im Cache ist.
        std::cerr << "Datei" << filename << "ist nicht geöffnet oder nicht im Cache vorhanden."
                  << std::endl;
        return nullptr; // Gibt einen nullptr zurück.
    }
}
