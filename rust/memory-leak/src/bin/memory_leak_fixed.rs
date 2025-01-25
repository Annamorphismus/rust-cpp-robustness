use std::cell::RefCell;
use std::rc::{Rc, Weak};

//----------------------------------Szenario------------------------------------------
/*
 * In diesem Programm wird ein Memory Leak durch die Verwendung von `Rc` und `Weak`
 * verhindert.
 * Der `Task` hält eine schwache Referenz (`Weak<Scheduler>`) auf den `Scheduler`,
 * wodurch der Referenzzyklus zwischen `Scheduler` und `Task` aufgelöst wird.
 * Der `Scheduler` verwaltet die Tasks mit starken Referenzen (`Rc<Task>`), während die
 * Rückverweise der Tasks auf den Scheduler den Referenzzähler nicht erhöhen.
 */

struct Task {
    name: String,
    scheduler: RefCell<Option<Weak<Scheduler>>>, // Schwache Referenz
}

impl Task {
    fn new(name: &str) -> Rc<Task> {
        Rc::new(Task {
            name: name.to_string(),
            scheduler: RefCell::new(None),
        })
    }

    fn set_scheduler(&self, scheduler: Rc<Scheduler>) {
        *self.scheduler.borrow_mut() = Some(Rc::downgrade(&scheduler));
    }

    // Methode, um den Namen auszugeben
    fn print_name(&self) {
        println!("Task Name: {}", self.name);
    }
}

struct Scheduler {
    tasks: RefCell<Vec<Rc<Task>>>,
}

impl Scheduler {
    fn new() -> Rc<Scheduler> {
        Rc::new(Scheduler {
            tasks: RefCell::new(Vec::new()),
        })
    }

    fn add_task(&self, task: Rc<Task>) {
        self.tasks.borrow_mut().push(task);
    }

    fn print_tasks(&self) {
        println!("Scheduler verwaltet folgende Tasks:");
        for task in self.tasks.borrow().iter() {
            task.print_name();
        }
    }
}

fn main() {
    let scheduler = Scheduler::new();
    let task1 = Task::new("Task 1");
    let task2 = Task::new("Task 2");

    scheduler.add_task(task1.clone());
    scheduler.add_task(task2.clone());

    task1.set_scheduler(scheduler.clone());
    task2.set_scheduler(scheduler.clone());

    scheduler.print_tasks();

    println!("Scheduler und Tasks erstellt.");
    println!(
        "Referenzzähler des Schedulers: {}",
        Rc::strong_count(&scheduler)
    );
    println!("Referenzzähler von Task 1: {}", Rc::strong_count(&task1));
}
