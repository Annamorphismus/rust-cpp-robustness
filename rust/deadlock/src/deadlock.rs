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

// Aktualisiert die Konfigurationsdatei und loggt die Änderung.
// Deadlock-Gefahr: Sperrt zuerst `config_mutex`, dann `log_mutex`
fn update_config(thread_name: &str, config_mutex: Arc<Mutex<()>>, log_mutex: Arc<Mutex<()>>) {
    // Sperre die Konfigurationsdatei zuerst
    let _config_lock = config_mutex.lock().unwrap();
    println!("{thread_name} hat die Konfigurationsdatei gesperrt.");

    // Öffnet die Datei für Anhängen, erstellt sie falls nicht vorhanden
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.txt")
        .unwrap();

    // Schreibt eine Meldung in die Konfigurationsdatei
    writeln!(config_file, "{thread_name} aktualisiert die Konfiguration.").unwrap();
    println!("{thread_name} hat die Konfiguration aktualisiert.");

    // Simulierte Verzögerung, um Deadlocks wahrscheinlicher zu machen
    thread::sleep(Duration::from_millis(100));

    // Sperre die Log-Datei
    let _log_lock = log_mutex.lock().unwrap();
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    // Schreibt eine Log-Meldung
    writeln!(
        log_file,
        "{thread_name} hat eine Konfigurationsänderung geloggt."
    )
    .unwrap();
    println!("{thread_name} hat die Änderung im Log festgehalten.");
}

// Protokolliert einen Fehler und überprüft danach die Konfigurationsdatei.
// Deadlock-Gefahr: Sperrt zuerst `log_mutex`, dann `config_mutex`.
fn log_error(thread_name: &str, log_mutex: Arc<Mutex<()>>, config_mutex: Arc<Mutex<()>>) {
    // Sperre die Log-Datei zuerst
    let _log_lock = log_mutex.lock().unwrap();
    println!("{thread_name} hat die Log-Datei gesperrt.");

    // Öffnet die Datei für Anhängen, erstellt sie falls nicht vorhanden
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    // Schreibt eine Fehlermeldung in die Log-Datei
    writeln!(log_file, "{thread_name} protokolliert einen Fehler.").unwrap();
    println!("{thread_name} hat den Fehler protokolliert.");

    // Simulierte Verzögerung, um Deadlocks wahrscheinlicher zu machen
    thread::sleep(Duration::from_millis(100));

    // Sperrt danach die Konfigurationsdatei
    let _config_lock = config_mutex.lock().unwrap();
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.txt")
        .unwrap();

    // Schreibt eine Überprüfungsmeldung in die Konfigurationsdatei
    writeln!(
        config_file,
        "{thread_name} hat die Konfiguration überprüft."
    )
    .unwrap();
    println!("{thread_name} hat die Konfigurationsdatei überprüft.");
}

fn main() {
    println!("Programm gestartet.");

    // Erzeugt zwei Mutex-Sperren für die Konfigurations- und Log-Datei
    let config_mutex = Arc::new(Mutex::new(()));
    let log_mutex = Arc::new(Mutex::new(()));

    // Klone der Arc-Mutexe für Thread 1
    let thread1_config_mutex = Arc::clone(&config_mutex);
    let thread1_log_mutex = Arc::clone(&log_mutex);

    // Klone der Arc-Mutexe für Thread 2
    let thread2_config_mutex = Arc::clone(&config_mutex);
    let thread2_log_mutex = Arc::clone(&log_mutex);

    // Erstellt und startet Thread 1 (Sperr-Reihenfolge: config → log)
    let thread1 = thread::spawn(move || {
        update_config("Thread 1", thread1_config_mutex, thread1_log_mutex);
    });

    // Erstellt und startet Thread 2 (Sperr-Reihenfolge: log → config)
    let thread2 = thread::spawn(move || {
        log_error("Thread 2", thread2_log_mutex, thread2_config_mutex);
    });

    // Warten auf die Beendigung beider Threads
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Programm beendet.");
}
