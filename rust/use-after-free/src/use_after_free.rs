use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Datei-Manager mit Thread-Sicherheit.
struct FileManager {
    file_cache: Arc<Mutex<HashMap<String, Arc<Mutex<File>>>>>,
}

impl FileManager {
    /// Erstellt einen neuen FileManager.
    fn new() -> Self {
        FileManager {
            file_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Öffnet eine Datei und speichert sie im Cache.
    fn open_file(&self, filename: &str, write: bool) -> Arc<Mutex<File>> {
        let mut cache = self.file_cache.lock().unwrap();

        if let Some(file) = cache.get(filename) {
            return file.clone();
        }

        let file = if write {
            File::create(filename).unwrap()
        } else {
            File::open(filename).unwrap()
        };

        let file_arc = Arc::new(Mutex::new(file));
        cache.insert(filename.to_string(), file_arc.clone());
        file_arc
    }

    /// Entfernt eine Datei aus dem Cache.
    fn close_file(&self, filename: &str) {
        let mut cache = self.file_cache.lock().unwrap();
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

    // Datei erstellen
    {
        let mut file = File::create(filename).unwrap();
        writeln!(file, "Zeile 1\nZeile 2\nZeile 3").unwrap();
    }

    // Datei öffnen und Referenz speichern
    let file_arc = manager.open_file(filename, false);
    let manager_clone = Arc::new(manager);
    let filename_clone = filename.to_string();

    // Thread 1 liest die Datei
    let reader = thread::spawn({
        let file_arc = Arc::clone(&file_arc);
        move || {
            thread::sleep(Duration::from_millis(100));
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

    // Thread 2 schließt die Datei
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        manager_clone.close_file(&filename_clone);
        println!("Schreiber hat die Datei geschlossen.");
    });

    reader.join().unwrap();
    writer.join().unwrap();

    println!("Versuch, auf eine geschlossene Datei zuzugreifen:");

    // Rust verhindert UAF: Das Arc ist bereits gelöscht!
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

    // Datei löschen
    if std::fs::remove_file(filename).is_ok() {
        println!("Beispieldatei gelöscht.");
    }
}
