#include <iostream>
#include <memory>
#include <string>
#include <vector>

//----------------------------------Szenario------------------------------------------
/*
 * In diesem Programm wird ein Memory Leak durch die Verwendung von `std::weak_ptr`
 * verhindert.
 * Die Tasks speichern nur schwache Referenzen (`std::weak_ptr`) auf den Scheduler,
 * wodurch der Referenzzyklus zwischen `Scheduler` und `Task` aufgelöst wird.
 * Der Scheduler nutzt `std::enable_shared_from_this`, um eine starke Referenz
 * zu sich selbst zu erstellen, die sicher an die Tasks übergeben wird.
 */

class Scheduler; // Vorwärtsdeklaration

class Task
{
public:
    explicit Task(const std::string& name) : name(name)
    {
        std::cout << "Task \"" << name << "\" erstellt.\n";
    }

    ~Task() { std::cout << "Task \"" << name << "\" gelöscht.\n"; }

    // Setzt den Scheduler für diese Task
    void setScheduler(const std::shared_ptr<Scheduler>& sched)
    {
        scheduler = sched; // `weak_ptr` speichert keine starke Referenz
    }

    // Gibt den Namen der Task aus
    void printName() const { std::cout << "Task Name: " << name << '\n'; }

private:
    std::string name;
    std::weak_ptr<Scheduler> scheduler; // Schwache Referenz auf Scheduler
};

class Scheduler : public std::enable_shared_from_this<Scheduler>
{
public:
    Scheduler() { std::cout << "Scheduler erstellt.\n"; }

    ~Scheduler() { std::cout << "Scheduler gelöscht.\n"; }

    // Fügt eine Task zum Scheduler hinzu und setzt den Scheduler für die Task
    void addTask(const std::shared_ptr<Task>& task)
    {
        tasks.push_back(task);
        task->setScheduler(
            shared_from_this()); // Stellt sicher, dass eine schwache Referenz gesetzt wird
    }

private:
    std::vector<std::shared_ptr<Task>> tasks; // Liste von Tasks
};

int main()
{
    // Erstelle einen Scheduler und Tasks
    auto scheduler = std::make_shared<Scheduler>();
    auto task1 = std::make_shared<Task>("Task 1");
    auto task2 = std::make_shared<Task>("Task 2");

    // Füge die Tasks dem Scheduler hinzu
    scheduler->addTask(task1);
    scheduler->addTask(task2);

    // Gebe die Namen der Tasks aus
    task1->printName();
    task2->printName();

    // Debug-Ausgabe für Referenzzähler
    std::cout << "Referenzzähler des Schedulers: " << scheduler.use_count() << '\n';

    return 0; // Kein Memory Leak, da schwache Referenzen genutzt werden
}
