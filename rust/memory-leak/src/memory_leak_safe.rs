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

// Repräsentiert eine einzelne Aufgabe (`Task`) innerhalb eines Schedulers.
// Jede Aufgabe kennt ihren Namen und besitzt eine schwache Referenz (`Weak<Scheduler>`)
// auf den Scheduler, um zyklische Referenzen zu vermeiden.
struct Task {
    name: String,
    scheduler: RefCell<Option<Weak<Scheduler>>>, // Schwache Referenz auf den Scheduler
}

impl Task {
    // Erstellt eine neue `Task` mit dem gegebenen Namen und gibt eine `Rc<Task>`-Referenz zurück.
    fn new(name: &str) -> Rc<Task> {
        Rc::new(Task {
            name: name.to_string(),
            scheduler: RefCell::new(None), // Anfangs keine Verbindung zum Scheduler
        })
    }
    // Setzt den Scheduler für die Aufgabe, indem `Rc::downgrade` verwendet wird,
    // um eine `Weak<Scheduler>`-Referenz zu erstellen.
    fn set_scheduler(&self, scheduler: Rc<Scheduler>) {
        *self.scheduler.borrow_mut() = Some(Rc::downgrade(&scheduler));
    }

    /// Gibt den Namen der Task aus.
    fn print_name(&self) {
        println!("Task Name: {}", self.name);
    }
}
// Der `Scheduler` verwaltet eine Liste von `Task`-Instanzen.
// Die `tasks`-Liste speichert `Rc<Task>`-Referenzen, wodurch sichergestellt wird,
// dass die Aufgaben solange existieren, wie sie vom Scheduler verwaltet werden.
struct Scheduler {
    tasks: RefCell<Vec<Rc<Task>>>, // Initial leerer Task-Container
}

impl Scheduler {
    fn new() -> Rc<Scheduler> {
        Rc::new(Scheduler {
            tasks: RefCell::new(Vec::new()),
        })
    }
    // Fügt eine `Task` dem Scheduler hinzu.
    fn add_task(&self, task: Rc<Task>) {
        self.tasks.borrow_mut().push(task);
    }

    // Gibt alle verwalteten Tasks aus.
    fn print_tasks(&self) {
        println!("Scheduler verwaltet folgende Tasks:");
        for task in self.tasks.borrow().iter() {
            task.print_name();
        }
    }
}

fn main() {
    // Erstellt eine Instanz des `Scheduler`
    let scheduler = Scheduler::new();

    // Erstellt zwei Tasks
    let task1 = Task::new("Task 1");
    let task2 = Task::new("Task 2");

    // Fügt die Tasks zum Scheduler hinzu
    scheduler.add_task(task1.clone());
    scheduler.add_task(task2.clone());

    // Setzt den Scheduler für die Tasks, wobei eine schwache Referenz verwendet wird
    task1.set_scheduler(scheduler.clone());
    task2.set_scheduler(scheduler.clone());

    // Gibt alle Aufgaben aus, die der Scheduler verwaltet
    scheduler.print_tasks();

    // Debugging: Gibt die Referenzzähler für den Scheduler und die Tasks aus
    println!("Scheduler und Tasks erstellt.");
    println!(
        "Referenzzähler des Schedulers: {}",
        Rc::strong_count(&scheduler)
    );
    println!("Referenzzähler von Task 1: {}", Rc::strong_count(&task1));
}
