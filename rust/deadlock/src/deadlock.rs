use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert ein Logging- und Konfigurationssystem, in dem zwei Threads
* auf zwei gemeinsame Ressourcen zugreifen: eine Konfigurationsdatei (`config.txt`) und
* eine Log-Datei (`log.txt`). Thread 1 sperrt zuerst die Konfigurationsdatei und dann die
* Log-Datei, während Thread 2 die Sperren in umgekehrter Reihenfolge vornimmt.
* Dadurch entsteht ein Deadlock, da beide Threads aufeinander warten, ohne fortzufahren.
*/

//----------------------------------Ausgabe------------------------------------------
/*
 * Die Konsolenausgabe zeigt, welche Ressource von welchem Thread gesperrt wurde:
 *   Thread 1 hat die Konfigurationsdatei gesperrt.
 *   Thread 2 hat die Log-Datei gesperrt.
 * Danach bleibt das Programm hängen, da ein Deadlock die Ausführung blockiert.
 */

fn update_config(thread_name: &str, config_mutex: Arc<Mutex<()>>, log_mutex: Arc<Mutex<()>>) {
    // Sperre die Konfigurationsdatei zuerst
    let _config_lock = config_mutex.lock().unwrap();
    println!("{thread_name} hat die Konfigurationsdatei gesperrt.");

    // Schreiben in die Konfigurationsdatei
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.txt")
        .unwrap();
    writeln!(config_file, "{thread_name} aktualisiert die Konfiguration.").unwrap();
    println!("{thread_name} hat die Konfiguration aktualisiert.");

    // Simulierte Verzögerung
    thread::sleep(Duration::from_millis(100));

    // Sperre die Log-Datei
    let _log_lock = log_mutex.lock().unwrap();
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

fn log_error(thread_name: &str, log_mutex: Arc<Mutex<()>>, config_mutex: Arc<Mutex<()>>) {
    // Sperre die Log-Datei zuerst
    let _log_lock = log_mutex.lock().unwrap();
    println!("{thread_name} hat die Log-Datei gesperrt.");

    // Schreiben in die Log-Datei
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();
    writeln!(log_file, "{thread_name} protokolliert einen Fehler.").unwrap();
    println!("{thread_name} hat den Fehler protokolliert.");

    // Simulierte Verzögerung
    thread::sleep(Duration::from_millis(100));

    // Sperre die Konfigurationsdatei
    let _config_lock = config_mutex.lock().unwrap();
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

    let config_mutex = Arc::new(Mutex::new(()));
    let log_mutex = Arc::new(Mutex::new(()));

    let thread1_config_mutex = Arc::clone(&config_mutex);
    let thread1_log_mutex = Arc::clone(&log_mutex);

    let thread2_config_mutex = Arc::clone(&config_mutex);
    let thread2_log_mutex = Arc::clone(&log_mutex);

    let thread1 = thread::spawn(move || {
        update_config("Thread 1", thread1_config_mutex, thread1_log_mutex);
    });

    let thread2 = thread::spawn(move || {
        log_error("Thread 2", thread2_log_mutex, thread2_config_mutex);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Programm beendet.");
}
