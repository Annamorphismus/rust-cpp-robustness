#include <iostream>
#include <memory>
#include <string>
#include <vector>

//----------------------------------Szenario------------------------------------------
/*
 * Dieses Programm simuliert ein Scheduler-System, in dem ein Scheduler (`Scheduler`)
 * eine Liste von Tasks (`Task`) verwaltet. Jede Task referenziert zurück auf den Scheduler.
 * Da sowohl der Scheduler als auch die Tasks mit starken Referenzen (`std::shared_ptr`)
 * arbeiten, entsteht ein zyklischer Abhängigkeitsgraph (Referenzzyklus).
 * Dieser Zyklus verhindert, dass der Speicher für den Scheduler und die Tasks
 * freigegeben wird, was zu einem Memory Leak führt.
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
    void setScheduler(const std::shared_ptr<Scheduler>& sched) { scheduler = sched; }

    // Gibt den Namen der Task aus
    void printName() const { std::cout << "Task Name: " << name << '\n'; }

private:
    std::string name;
    std::shared_ptr<Scheduler> scheduler; // Starke Referenz auf Scheduler
};

class Scheduler
{
public:
    Scheduler() { std::cout << "Scheduler erstellt.\n"; }

    ~Scheduler() { std::cout << "Scheduler gelöscht.\n"; }

    // Fügt eine Task zum Scheduler hinzu
    void addTask(const std::shared_ptr<Task>& task) { tasks.push_back(task); }

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

    // Setze den Scheduler für jede Task
    task1->setScheduler(scheduler);
    task2->setScheduler(scheduler);

    // Gebe die Namen der Tasks aus
    task1->printName();
    task2->printName();

    // Debug-Ausgabe für Referenzzähler
    std::cout << "Referenzzähler des Schedulers: " << scheduler.use_count() << '\n';

    return 0; // Memory Leak durch zyklische Referenzen
}
