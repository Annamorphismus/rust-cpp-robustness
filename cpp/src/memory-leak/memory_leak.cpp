#include <iostream>
#include <string>
#include <vector>

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert ein Scheduler-System, in dem ein Scheduler (`Scheduler`)
 * eine Liste von Tasks (`Task`) verwaltet. Jede Task referenziert zurück auf den Scheduler.
 * Hier werden einfache Pointer (`new`) verwendet, ohne dass der Speicher explizit
 * freigegeben wird. Dies führt zu Memory Leaks.
 */

class Scheduler; // Vorwärtsdeklaration

class Task
{
public:
    explicit Task(const std::string& name) : name(name), scheduler(nullptr)
    {
        std::cout << "Task \"" << name << "\" erstellt.\n";
    }

    ~Task() { std::cout << "Task \"" << name << "\" gelöscht.\n"; }

    // Setzt den Scheduler für diese Task
    void setScheduler(Scheduler* sched) { scheduler = sched; }

    // Gibt den Namen der Task aus
    void printName() const { std::cout << "Task Name: " << name << '\n'; }

private:
    std::string name;
    Scheduler* scheduler; // Roher Zeiger auf Scheduler
};

class Scheduler
{
public:
    Scheduler() { std::cout << "Scheduler erstellt.\n"; }

    ~Scheduler() { std::cout << "Scheduler gelöscht.\n"; }

    // Fügt eine Task zum Scheduler hinzu
    void addTask(Task* task) { tasks.push_back(task); }

private:
    std::vector<Task*> tasks; // Liste von Tasks als rohe Zeiger
};

int main()
{
    // Erstelle einen Scheduler und Tasks
    Scheduler* scheduler = new Scheduler();
    Task* task1 = new Task("Task 1");
    Task* task2 = new Task("Task 2");

    // Füge die Tasks dem Scheduler hinzu
    scheduler->addTask(task1);
    scheduler->addTask(task2);

    // Setze den Scheduler für jede Task
    task1->setScheduler(scheduler);
    task2->setScheduler(scheduler);

    // Gebe die Namen der Tasks aus
    task1->printName();
    task2->printName();

    return 0; // Memory Leak durch fehlende Speicherfreigabe
}
