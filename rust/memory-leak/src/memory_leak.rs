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
*/

// Die `Task`-Struktur repräsentiert eine Aufgabe, die von einem `Scheduler` verwaltet wird.
// Jede `Task` hält eine starke Referenz (`Rc<Scheduler>`), wodurch ein zyklischer
// Referenzzähler entsteht, wenn der `Scheduler` ebenfalls `Rc<Task>` speichert.
struct Task {
    name: String, // Behalte das Feld "name"
    scheduler: RefCell<Option<Rc<Scheduler>>>,
}

impl Task {
    // Erstellt eine neue `Task`-Instanz mit dem gegebenen Namen und gibt eine `Rc<Task>` zurück.
    fn new(name: &str) -> Rc<Task> {
        Rc::new(Task {
            name: name.to_string(),
            scheduler: RefCell::new(None), // Anfangs kein Scheduler zugewiese
        })
    }

    // Setzt eine Referenz zum `Scheduler` und speichert sie als `Rc<Scheduler>`.
    // Problem: Da `Rc` eine starke Referenz ist, entsteht ein Referenzzyklus.
    fn set_scheduler(&self, scheduler: Rc<Scheduler>) {
        *self.scheduler.borrow_mut() = Some(scheduler);
    }

    /// Gibt den Namen der `Task` aus.
    fn print_name(&self) {
        println!("Task Name: {}", self.name);
    }
}

// Die `Scheduler`-Struktur verwaltet mehrere `Task`-Objekte.
// Der `Scheduler` speichert eine Liste von `Rc<Task>`, wodurch die Tasks
// nicht automatisch gelöscht werden können.
struct Scheduler {
    tasks: RefCell<Vec<Rc<Task>>>, // Starke Referenzen auf Tasks
}

impl Scheduler {
    // Erstellt eine neue `Scheduler`-Instanz und gibt eine `Rc<Scheduler>` zurück.
    fn new() -> Rc<Scheduler> {
        Rc::new(Scheduler {
            tasks: RefCell::new(Vec::new()), // Initial leere Liste von Tasks
        })
    }

    /// Fügt eine `Task` zum Scheduler hinzu, indem sie in `tasks` gespeichert wird.
    fn add_task(&self, task: Rc<Task>) {
        self.tasks.borrow_mut().push(task);
    }
}

fn main() {
    // Erstellt eine Instanz des `Scheduler`
    let scheduler = Scheduler::new();

    // Erstellt zwei Tasks
    let task1 = Task::new("Task 1");
    let task2 = Task::new("Task 2");

    // Fügt die Tasks zum Scheduler hinzu (starke Referenz `Rc<Task>`)
    scheduler.add_task(task1.clone());
    scheduler.add_task(task2.clone());

    // Weist den Tasks den Scheduler zu (starke Referenz `Rc<Scheduler>`)
    // Problem: Dies erzeugt einen Referenzzyklus, da sich Tasks und Scheduler gegenseitig halten.
    task1.set_scheduler(scheduler.clone());
    task2.set_scheduler(scheduler.clone());

    // Gibt die Namen der Tasks aus
    task1.print_name();
    task2.print_name();

    // Debugging: Gibt die Referenzzähler aus
    println!("Scheduler und Tasks erstellt. Speicher wird nie freigegeben!");
    println!(
        "Referenzzähler des Schedulers: {}",
        Rc::strong_count(&scheduler)
    );
}
