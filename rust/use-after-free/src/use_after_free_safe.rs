use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex, Weak};
use std::thread;

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert ein Dateiverwaltungssystem, in dem mehrere Threads
 * gleichzeitig auf eine gemeinsame Datei zugreifen können. Ein Thread liest
 * die Datei, während ein anderer versucht, die Datei zu schließen und aus
 * dem Cache zu entfernen.
 *
 * Durch die Verwendung von `Weak<T>` wird verhindert, dass der Lese-Thread auf eine
 * bereits geschlossene Datei zugreift.
 */

//----------------------------------Ablauf------------------------------------------
/*
 * - Der `FileManager` verwaltet geöffnete Dateien in einem Cache (`HashMap`), geschützt
 *   durch einen `Mutex`. Dadurch können mehrere Threads sicher auf die Dateien zugreifen.
 * - Ein Thread liest eine Zeile aus der Datei, während ein anderer Thread versucht,
 *   die Datei zu schließen und aus dem Cache zu entfernen.
 * - Der Lese-Thread verwendet `Weak<T>`, um zu überprüfen, ob die Datei noch existiert.
 */

//----------------------------------Ergebnis------------------------------------------
/*
 * - Der Lese-Thread wird die Datei nur dann lesen, wenn sie noch im Cache existiert.
 * - Falls der Schreib-Thread die Datei vorher entfernt, wird der Leser darauf hingewiesen,
 *   dass die Datei nicht mehr verfügbar ist.
 */

/// Der `FileManager` verwaltet geöffnete Dateien und sichert den Zugriff mit einem `Mutex`.
struct FileManager {
    file_cache: Arc<Mutex<HashMap<String, Weak<Mutex<Option<File>>>>>>, // Weak<T> für Leser-Sicherheit
}

impl FileManager {
    /// Erstellt eine neue Instanz des `FileManager`.
    fn new() -> Self {
        FileManager {
            file_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Öffnet eine Datei und speichert sie als `Arc<Mutex<Option<File>>>` im Cache.
    fn open_file(&self, filename: &str, write: bool) -> io::Result<Arc<Mutex<Option<File>>>> {
        let mut cache = self.file_cache.lock().unwrap();

        // Falls die Datei bereits existiert, gib die starke Referenz zurück
        if let Some(file_weak) = cache.get(filename) {
            if let Some(file_arc) = file_weak.upgrade() {
                return Ok(file_arc);
            }
        }

        // Datei erstellen oder öffnen
        let file = if write {
            File::create(filename)?
        } else {
            File::open(filename)?
        };

        let file_arc = Arc::new(Mutex::new(Some(file))); // `Option<File>` für sicheres Entfernen
        cache.insert(filename.to_string(), Arc::downgrade(&file_arc)); // Nur Weak im Cache speichern
        Ok(file_arc)
    }

    /// Schließt eine Datei und entfernt sie sicher aus dem Cache.
    fn close_file(&self, filename: &str) {
        let mut cache = self.file_cache.lock().unwrap();

        if let Some(file_weak) = cache.remove(filename) {
            if let Some(file_arc) = file_weak.upgrade() {
                let mut file_lock = file_arc.lock().unwrap();
                if let Some(file) = file_lock.take() {
                    drop(file); // Explizites Freigeben der Datei
                    println!("File '{}' closed and removed from cache.", filename);
                }
            }
        } else {
            println!("File '{}' was not found in cache.", filename);
        }
    }
}

fn main() {
    let manager = FileManager::new();
    let filename = "example.txt";

    // Erstellt eine Beispieldatei
    {
        let mut file = File::create(filename).unwrap();
        writeln!(file, "Line 1\nLine 2\nLine 3").unwrap();
    }

    // Datei öffnen und `Weak<T>` nutzen
    let file_arc = manager.open_file(filename, false).unwrap();
    let file_weak = Arc::downgrade(&file_arc);

    let manager_clone = Arc::new(manager);
    let manager_clone_for_writer = manager_clone.clone();
    let filename_clone = filename.to_string();

    // Thread, der die Datei liest
    let reader = thread::spawn({
        move || {
            thread::sleep(std::time::Duration::from_millis(100)); // Verzögerung für realistischere Konkurrenzsituation

            // Versucht, die Datei zu lesen, falls sie noch existiert
            if let Some(file_arc) = file_weak.upgrade() {
                let file = file_arc.lock().unwrap();
                if let Some(ref file) = *file {
                    let mut buf_reader = io::BufReader::new(file);
                    let mut line = String::new();
                    if buf_reader.read_line(&mut line).is_ok() {
                        println!("Reader read: {}", line.trim());
                    } else {
                        println!("Reader: Could not read the file.");
                    }
                } else {
                    println!("Reader: File has already been closed.");
                }
            } else {
                println!("Reader: File no longer exists.");
            }
        }
    });

    // Thread, der die Datei schließt
    let writer = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50)); // Verzögerung zur Simulation von Konkurrenz

        // Entfernt die Datei aus dem Cache
        manager_clone_for_writer.close_file(&filename_clone);
        println!("Writer closed the file.");
    });

    // Warten auf beide Threads
    reader.join().unwrap();
    writer.join().unwrap();

    // Löscht die Beispieldatei vom Dateisystem
    if std::fs::remove_file(filename).is_ok() {
        println!("Example file deleted.");
    }
}
