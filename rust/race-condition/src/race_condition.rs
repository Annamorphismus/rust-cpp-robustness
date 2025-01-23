use std::thread;

// Globale Variable
static mut COUNTER: i32 = 0;

pub struct RaceCondition;

impl RaceCondition {
    pub fn new() -> Self {
        RaceCondition
    }

    pub fn simulate(&self) {
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = thread::spawn(|| {
                for _ in 0..1000 {
                    unsafe {
                        // Kritischer Abschnitt ohne Schutz
                        COUNTER += 1;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        unsafe {
            println!("Race Condition - Erwarteter Zähler: 10000");
            println!("Race Condition - Tatsächlicher Zähler: {}", COUNTER);
        }
    }
}
