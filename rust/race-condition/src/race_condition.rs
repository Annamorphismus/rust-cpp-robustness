use std::sync::Arc;
use std::thread;

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert eine Race Condition, indem mehrere Threads
 * gleichzeitig auf eine gemeinsame Variable (`counter`) zugreifen und diese
 * inkrementieren. Der Zugriff erfolgt ohne Synchronisationsmechanismen, wodurch
 * Dateninkonsistenzen entstehen können.
 */
//----------------------------------Ursache-------------------------------------------
/*
 * Jeder Thread greift unsicher (über `unsafe`) auf den `counter` zu und
 * inkrementiert ihn. Es gibt keine Synchronisation, die sicherstellt,
 * dass der Zugriff atomar erfolgt.
 */
//----------------------------------Ergebnis------------------------------------------
/*
 * Aufgrund der Race Condition erreicht der `counter` möglicherweise
 * nicht den erwarteten Wert (`10000`), sondern einen inkonsistenten Wert,
 * da Threads gleichzeitig auf dieselbe Speicheradresse zugreifen.
 */

use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    // Erstellt einen gemeinsamen Zähler mit `RefCell`
    // Problem: `RefCell` ist nicht threadsicher.
    let counter = Arc::new(RefCell::new(0));

    // Vektor zur Speicherung der Thread-Handles
    let mut handles = vec![];

    for _ in 0..10 {
        // Erstellt eine neue Referenz , die in den Thread übernommen wird
        let counter = Arc::clone(&counter);

        // Erstellt und startet einen neuen Thread
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // Race Condition!
                // Mehrere Threads greifen gleichzeitig auf `counter` zu, ohne Synchronisation.
                *counter.borrow_mut() += 1;
            }
        });
        // Speichert das Handle des Threads, um später auf seinen Abschluss zu warten
        handles.push(handle);
    }
    // Wartet darauf, dass alle Threads beendet sind
    for handle in handles {
        handle.join().unwrap();
    }

    // Erwarteter Wert: 10000 (10 Threads * 1000 Inkremente pro Thread)
    println!("Erwarteter Zähler: 10000");

    // Tatsächlicher Wert kann abweichen, da eine Race Condition vorliegt
    println!("Tatsächlicher Zähler: {}", counter.borrow());
}
