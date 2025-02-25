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
    let counter = Arc::new(RefCell::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                *counter.borrow_mut() += 1; // ❌ Nicht threadsicher!
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Erwarteter Zähler: 10000");
    println!("Tatsächlicher Zähler: {}", counter.borrow());
}
