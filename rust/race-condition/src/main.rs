mod race_condition;
mod sync;

use race_condition::RaceCondition;
use sync::Synchronization;

fn main() {
    println!("Optionen:");
    println!("1: Race Condition simulieren");
    println!("2: Synchronisation simulieren");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            let race_condition = RaceCondition::new();
            race_condition.simulate();
        }
        "2" => {
            let sync = Synchronization::new();
            sync.simulate();
        }
        _ => println!("Ungültige Eingabe!"),
    }
}
