use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert ein Logging- und Konfigurationssystem, bei dem zwei Threads
 * auf zwei gemeinsam genutzte Ressourcen zugreifen: die Konfigurationsdatei
 * (`config.txt`) und die Log-Datei (`log.txt`). Thread 1 aktualisiert zuerst die
 * Konfiguration und schreibt dann ins Log, während Thread 2 zuerst einen Fehler
 * protokolliert und danach die Konfiguration überprüft.
 * Ohne Synchronisationsmechanismen würde die unterschiedliche Sperrreihenfolge der
 * Ressourcen einen Deadlock verursachen.
 */

//--------------------Mechanismen zur Verhinderung von Deadlocks-----------------------
/*
 * 1. Konsistente Sperrreihenfolge:
 *    Beide Threads sperren die Ressourcen in derselben Reihenfolge:
 *    - Zuerst die Konfigurationsdatei (`config_mutex`), dann die Log-Datei (`log_mutex`).
 * 2. Verwendung von `std::sync::Mutex`:
 *    Der Zugriff auf die Ressourcen wird durch `Mutex` geschützt, um sicherzustellen,
 *    dass nur ein Thread gleichzeitig eine Ressource sperrt.
 *
 * Durch diese Mechanismen wird verhindert, dass die Threads sich gegenseitig blockieren.*/

fn update_config(thread_name: &str, config_mutex: Arc<Mutex<()>>, log_mutex: Arc<Mutex<()>>) {
    // Sperre beide Mutexe in konsistenter Reihenfolge
    let (first_lock, second_lock) = (&config_mutex, &log_mutex);
    let _lock1 = first_lock.lock().unwrap();
    let _lock2 = second_lock.lock().unwrap();

    // Schreiben in die Konfigurationsdatei
    println!("{thread_name} hat die Konfigurationsdatei gesperrt.");
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.txt")
        .unwrap();
    writeln!(config_file, "{thread_name} aktualisiert die Konfiguration.").unwrap();
    println!("{thread_name} hat die Konfiguration aktualisiert.");

    // Simulierte Verzögerung
    thread::sleep(Duration::from_millis(100));

    // Schreiben in die Log-Datei
    println!("{thread_name} hat die Log-Datei gesperrt.");
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();
    writeln!(
        log_file,
        "{thread_name} hat eine Konfigurationsänderung geloggt."
    )
    .unwrap();
    println!("{thread_name} hat die Änderung im Log festgehalten.");
}

// Funktion: Fehlerprotokollierung
fn log_error(thread_name: &str, config_mutex: Arc<Mutex<()>>, log_mutex: Arc<Mutex<()>>) {
    // Sperre beide Mutexe in konsistenter Reihenfolge
    let (first_lock, second_lock) = (&config_mutex, &log_mutex);
    let _lock1 = first_lock.lock().unwrap();
    let _lock2 = second_lock.lock().unwrap();

    // Schreiben in die Log-Datei
    println!("{thread_name} hat die Log-Datei gesperrt.");
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();
    writeln!(log_file, "{thread_name} protokolliert einen Fehler.").unwrap();
    println!("{thread_name} hat den Fehler protokolliert.");

    // Simulierte Verzögerung
    thread::sleep(Duration::from_millis(100));

    // Schreiben in die Konfigurationsdatei
    println!("{thread_name} hat die Konfigurationsdatei gesperrt.");
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.txt")
        .unwrap();
    writeln!(
        config_file,
        "{thread_name} hat die Konfiguration überprüft."
    )
    .unwrap();
    println!("{thread_name} hat die Konfigurationsdatei überprüft.");
}

fn main() {
    println!("Programm gestartet.");

    // Gemeinsame Ressourcen mit Mutex schützen
    let config_mutex = Arc::new(Mutex::new(()));
    let log_mutex = Arc::new(Mutex::new(()));

    // Klone der Arc-Mutexe für die Threads erzeugen
    let thread1_config_mutex = Arc::clone(&config_mutex);
    let thread1_log_mutex = Arc::clone(&log_mutex);

    let thread2_config_mutex = Arc::clone(&config_mutex);
    let thread2_log_mutex = Arc::clone(&log_mutex);

    // Startet zwei Threads, die auf die Dateien zugreifen
    let thread1 = thread::spawn(move || {
        update_config("Thread 1", thread1_config_mutex, thread1_log_mutex);
    });

    let thread2 = thread::spawn(move || {
        log_error("Thread 2", thread2_config_mutex, thread2_log_mutex);
    });

    // Auf die Beendigung der Threads warten
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Programm beendet.");
}
