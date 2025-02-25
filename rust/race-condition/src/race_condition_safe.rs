use std::sync::{Arc, Mutex};
use std::thread;
//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm verhindert eine Race Condition, indem der Zugriff auf den gemeinsamen * Zähler (`counter`)
 * durch einen `Mutex` synchronisiert wird. Dadurch kann immer nur ein Thread
 * gleichzeitig den Zähler inkrementieren.
 */
//----------------------------------Mechanismen---------------------------------------
/*
 * `Mutex`: Synchronisiert den Zugriff und stellt atomare Updates sicher.
 * `Arc`: Erlaubt das sichere Teilen des Zählers zwischen Threads.
 */
//----------------------------------Ergebnis------------------------------------------
/*
 * Der Zähler erreicht den erwarteten Wert von `10000`, da keine Dateninkonsistenz entsteht.
 */

// Diese Funktion inkrementiert den gemeinsamen Zähler innerhalb eines Mutex
fn increment_counter_sync(counter: &Arc<Mutex<i32>>) {
    for _ in 0..1000 {
        // Sperrt den Mutex und erhält eine exklusive Zugriffsmöglichkeit auf den Zähler
        let mut data = counter.lock().unwrap();
        *data += 1;
        // Der Mutex wird automatisch freigegeben, sobald `data` aus dem Scope geht
    }
}

// Erstellt mehrere Threads, die gemeinsam einen Zähler hochzählen.
// - Nutzt `Arc<Mutex<i32>>`, um einen synchronisierten, geteilten Zähler zu verwalten.
// - Startet 10 Threads, die den Zähler jeweils 1000-mal inkrementieren.
// - Stellt sicher, dass nach Abschluss aller Threads der erwartete Wert `10000` erreicht wird.
fn prevent_race_condition() {
    // Erstellt einen mit `Mutex` geschützten gemeinsamen Zähler und teilt ihn mit `Arc`
    let counter = Arc::new(Mutex::new(0));

    // Speichert die Handles der gestarteten Threads
    let mut handles = vec![];

    for _ in 0..10 {
        // Erstellt eine neue Referenz auf den Zähler für den neuen Thread
        let counter = Arc::clone(&counter);

        // Startet einen neuen Thread, der den Zähler inkrementiert
        let handle = thread::spawn(move || {
            increment_counter_sync(&counter);
        });
        // Speichert das Handle des Threads, um später auf seinen Abschluss zu warten
        handles.push(handle);
    }

    // Wartet darauf, dass alle Threads beendet sind
    for handle in handles {
        handle.join().unwrap();
    }
    // Gibt das erwartete und tatsächliche Ergebnis aus
    println!("Erwarteter Zähler: 10000");
    println!("Tatsächlicher Zähler: {}", *counter.lock().unwrap());
}

fn main() {
    println!("Race Condition verhindern:");
    prevent_race_condition();
}
