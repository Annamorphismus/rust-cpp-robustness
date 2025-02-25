use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Datei-Manager mit Thread-Sicherheit.
/// Verwaltet geöffnete Dateien mithilfe eines `HashMap`-Caches,
/// der mit einem `Mutex` geschützt ist.
struct FileManager {
    file_cache: Arc<Mutex<HashMap<String, Arc<Mutex<File>>>>>, // Sicherer Dateicache
}

impl FileManager {
    /// Erstellt eine neue Instanz des `FileManager` mit einem leeren Dateicache.
    fn new() -> Self {
        FileManager {
            file_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Öffnet eine Datei und speichert sie im Cache, falls sie noch nicht existiert.
    /// - `filename`: Der Name der Datei.
    /// - `write`: Falls `true`, wird die Datei im Schreibmodus (`File::create`) geöffnet.
    ///            Falls `false`, wird sie nur zum Lesen (`File::open`) geöffnet.
    fn open_file(&self, filename: &str, write: bool) -> Arc<Mutex<File>> {
        let mut cache = self.file_cache.lock().unwrap(); // Sperrt den Cache für sicheren Zugriff

        // Falls die Datei bereits existiert, gib die gespeicherte Referenz zurück.
        if let Some(file) = cache.get(filename) {
            return file.clone();
        }

        // Datei erstellen oder öffnen
        let file = if write {
            File::create(filename).unwrap()
        } else {
            File::open(filename).unwrap()
        };

        let file_arc = Arc::new(Mutex::new(file)); // Datei wird in einem `Arc<Mutex<File>>` gespeichert
        cache.insert(filename.to_string(), file_arc.clone()); // Im Cache speichern
        file_arc
    }

    /// Entfernt eine Datei aus dem Cache.
    /// Falls die Datei existiert, wird sie aus dem Cache entfernt und geschlossen.
    fn close_file(&self, filename: &str) {
        let mut cache = self.file_cache.lock().unwrap(); // Sperrt den Cache
        if cache.remove(filename).is_some() {
            println!(
                "Datei '{}' geschlossen und aus dem Cache entfernt.",
                filename
            );
        } else {
            println!("Datei '{}' wurde nicht im Cache gefunden.", filename);
        }
    }
}

fn main() {
    let manager = FileManager::new();
    let filename = "example.txt";

    // Erstellt eine Beispieldatei und schreibt drei Zeilen hinein
    {
        let mut file = File::create(filename).unwrap();
        writeln!(file, "Zeile 1\nZeile 2\nZeile 3").unwrap();
    }

    // Öffnet die Datei und speichert eine Referenz darauf
    let file_arc = manager.open_file(filename, false);
    let manager_clone = Arc::new(manager); // Erzeugt eine Referenz auf den Manager
    let filename_clone = filename.to_string(); // Erstellt eine Kopie des Dateinamens für den anderen Thread

    // Thread 1: Liest die Datei
    let reader = thread::spawn({
        let file_arc = Arc::clone(&file_arc); // Cloned das `Arc`, um es in den Thread zu übertragen
        move || {
            thread::sleep(Duration::from_millis(100)); // Wartet kurz, um die Konkurrenzsituation zu simulieren

            // Sperrt die Datei und liest eine Zeile daraus
            let file = file_arc.lock().unwrap();
            let mut buf_reader = BufReader::new(&*file);
            let mut line = String::new();
            if buf_reader.read_line(&mut line).is_ok() {
                println!("Leser hat gelesen: {}", line.trim());
            } else {
                println!("Leser: Konnte die Datei nicht lesen.");
            }
        }
    });

    // Thread 2: Schließt die Datei
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Wartet kurz, um die Konkurrenzsituation zu simulieren
        manager_clone.close_file(&filename_clone); // Entfernt die Datei aus dem Cache
        println!("Schreiber hat die Datei geschlossen.");
    });

    // Wartet auf beide Threads
    reader.join().unwrap();
    writer.join().unwrap();

    println!("Versuch, auf eine geschlossene Datei zuzugreifen:");

    // **Rust verhindert UAF:** Falls `file_arc` noch existiert, wird es geprüft
    if let Ok(file) = file_arc.lock() {
        let mut buf_reader = BufReader::new(&*file);
        let mut line = String::new();
        if buf_reader.read_line(&mut line).is_ok() {
            println!(
                "Rust verhindert UAF – Datei ist immer noch sicher nutzbar: {}",
                line.trim()
            );
        }
    } else {
        println!("Rust verhindert UAF – kein Zugriff auf gelöschte Datei möglich!");
    }

    // Löscht die Beispieldatei
    if std::fs::remove_file(filename).is_ok() {
        println!("Beispieldatei gelöscht.");
    }
}
