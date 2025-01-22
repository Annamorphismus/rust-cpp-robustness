#include "race_condition.h"
#include "sync.h"
#include <iostream>

int main()
{
    std::cout << "Optionen:\n";
    std::cout << "1: Race Condition simulieren\n";
    std::cout << "2: Race Condition lösen\n";
    std::cout << "Wähle: ";
    int choice;
    std::cin >> choice;

    if (choice == 1) {
        simulate_race_condition();
    } else if (choice == 2) {
        prevent_race_condition();
    } else {
        std::cerr << "Ungültige Auswahl!" << std::endl;
    }

    return 0;
}
