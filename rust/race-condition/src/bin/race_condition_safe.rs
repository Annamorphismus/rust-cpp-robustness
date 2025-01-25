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

fn increment_counter_sync(counter: &Arc<Mutex<i32>>) {
    for _ in 0..1000 {
        let mut data = counter.lock().unwrap();
        *data += 1;
    }
}

fn prevent_race_condition() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            increment_counter_sync(&counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Erwarteter Zähler: 10000");
    println!("Tatsächlicher Zähler: {}", *counter.lock().unwrap());
}

fn main() {
    println!("Race Condition verhindern:");
    prevent_race_condition();
}
