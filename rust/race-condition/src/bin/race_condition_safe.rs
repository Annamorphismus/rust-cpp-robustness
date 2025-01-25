use std::sync::{Arc, Mutex};
use std::thread;

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
