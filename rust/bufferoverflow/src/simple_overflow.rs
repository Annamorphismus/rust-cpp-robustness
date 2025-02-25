fn main() {
    let buffer = [0u8; 32];
    println!("{}", buffer[64]); // Fehler: Index außerhalb des gültigen Bereichs!
}
