use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileManager {
    file_cache: Arc<Mutex<HashMap<String, Arc<Mutex<File>>>>>,
}

impl FileManager {
    fn new() -> Self {
        FileManager {
            file_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn open_file(&self, filename: &str, write: bool) -> io::Result<Arc<Mutex<File>>> {
        let mut cache = self.file_cache.lock().unwrap();

        if let Some(file) = cache.get(filename) {
            return Ok(file.clone());
        }

        let file = if write {
            File::create(filename)?
        } else {
            File::open(filename)?
        };

        let file_arc = Arc::new(Mutex::new(file));
        cache.insert(filename.to_string(), file_arc.clone());
        Ok(file_arc)
    }

    fn close_file(&self, filename: &str) {
        let mut cache = self.file_cache.lock().unwrap();
        if cache.remove(filename).is_some() {
            println!("File '{}' closed and removed from cache.", filename);
        } else {
            println!("File '{}' was not found in cache.", filename);
        }
    }
}

fn main() {
    let manager = FileManager::new();
    let filename = "example.txt";

    // Erstellen einer Beispieldatei
    {
        let mut file = File::create(filename).unwrap();
        writeln!(file, "Line 1\nLine 2\nLine 3").unwrap();
    }

    // Datei öffnen und teilen
    let file_arc = manager.open_file(filename, false).unwrap();

    let manager_clone = Arc::new(manager);
    let manager_clone_for_writer = manager_clone.clone();
    let filename_clone = filename.to_string();

    // Thread, der die Datei liest
    let reader = thread::spawn({
        let file_arc = file_arc.clone();
        move || {
            thread::sleep(std::time::Duration::from_millis(100));
            let file = file_arc.lock().unwrap();
            let mut buf_reader = io::BufReader::new(&*file);
            let mut line = String::new();
            if buf_reader.read_line(&mut line).is_ok() {
                println!("Reader read: {}", line.trim());
            } else {
                println!("Reader: Could not read the file.");
            }
        }
    });

    // Thread, der die Datei schließt
    let writer = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        manager_clone_for_writer.close_file(&filename_clone);
        println!("Writer closed the file.");
    });

    reader.join().unwrap();
    writer.join().unwrap();

    // Löschen der Beispieldatei
    if std::fs::remove_file(filename).is_ok() {
        println!("Example file deleted.");
    }
}
