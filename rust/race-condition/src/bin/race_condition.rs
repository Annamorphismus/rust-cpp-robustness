use std::sync::Arc;
use std::thread;

fn increment_counter(counter: &Arc<i32>) {
    for _ in 0..1000 {
        unsafe {
            // Unsicherer Zugriff auf den gemeinsamen Zähler
            *(Arc::as_ptr(counter) as *mut i32) += 1;
        }
    }
}

fn simulate_race_condition() {
    // Gemeinsame Variable, ohne Synchronisierung
    let counter = Arc::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            increment_counter(&counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Erwarteter Zähler: 10000");
    unsafe {
        println!(
            "Tatsächlicher Zähler (Race Condition): {}",
            *(Arc::as_ptr(&counter))
        );
    }
}

fn main() {
    println!("Simulation einer Race Condition:");
    simulate_race_condition();
}
