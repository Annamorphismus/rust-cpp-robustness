use std::sync::{Arc, Mutex};
use std::thread;

pub struct Synchronization;

impl Synchronization {
    pub fn new() -> Self {
        Synchronization
    }

    pub fn simulate(&self) {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    let mut num = counter_clone.lock().unwrap(); // Kritischer Abschnitt geschützt
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Synchronisation - Erwarteter Zähler: 10000");
        println!(
            "Synchronisation - Tatsächlicher Zähler: {}",
            *counter.lock().unwrap()
        );
    }
}
