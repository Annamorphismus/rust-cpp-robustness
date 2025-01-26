use std::cell::RefCell;
use std::rc::Rc;
//----------------------------------Szenario------------------------------------------
/*
 * Das Programm simuliert ein Task-Management-System, in dem ein Scheduler
 * (Scheduler) Tasks (Task) verwaltet. Ein Memory Leak tritt auf, wenn die Tasks eine
 * starke Referenz (Rc) zurück zum Scheduler halten, während der Scheduler ebenfalls
 * starke Referenzen auf die Tasks hält. Dadurch entsteht ein Referenzzyklus,
 * der verhindert, dass der Speicher jemals freigegeben wird.
 */

//----------------------------------Ausgabe------------------------------------------
/*
 * Die Referenzzähler des Schedulers und der Tasks bleiben überhöht,
 * da sie sich gegenseitig referenzieren

 * Zwei Mutex-Objekte: Eins für die Konfigurationsdatei und eins für das Log
*/

struct Task {
    name: String, // Behalte das Feld "name"
    scheduler: RefCell<Option<Rc<Scheduler>>>,
}

impl Task {
    fn new(name: &str) -> Rc<Task> {
        Rc::new(Task {
            name: name.to_string(),
            scheduler: RefCell::new(None),
        })
    }

    fn set_scheduler(&self, scheduler: Rc<Scheduler>) {
        *self.scheduler.borrow_mut() = Some(scheduler);
    }

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
}

fn main() {
    let scheduler = Scheduler::new();
    let task1 = Task::new("Task 1");
    let task2 = Task::new("Task 2");

    scheduler.add_task(task1.clone());
    scheduler.add_task(task2.clone());

    task1.set_scheduler(scheduler.clone());
    task2.set_scheduler(scheduler.clone());

    // Nutze die Namen der Tasks
    task1.print_name();
    task2.print_name();

    println!("Scheduler und Tasks erstellt. Speicher wird nie freigegeben!");
    println!(
        "Referenzzähler des Schedulers: {}",
        Rc::strong_count(&scheduler)
    );
}
