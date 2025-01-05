use std::env;
use std::ffi::OsString;

fn abracadabra() {
    println!("Abracadabra! Function called!");
}

#[repr(C)]
struct Hackvist {
    buffer: Vec<u8>,     // Ein dynamischer Puffer, der die Größe sicher handhabt
    point: Option<fn()>, // Ein sicherer optionaler Funktionszeiger
}

impl Hackvist {
    fn new() -> Self {
        Hackvist {
            buffer: Vec::with_capacity(16), // Maximal 16 Bytes
            point: None,
        }
    }

    fn add_input(&mut self, input: &[u8]) {
        // Prüfen, ob die Eingabe zu groß ist
        if input.len() > self.buffer.capacity() {
            eprintln!("Error: Input exceeds buffer capacity.");
            return;
        }

        // Eingabe sicher in den Puffer kopieren
        self.buffer.clear();
        self.buffer.extend_from_slice(input);

        // Optional: Funktionalität sicher einrichten
        if self.buffer.len() == 16 && self.buffer == b"CALL_ABRACADABRA" {
            self.point = Some(abracadabra);
        } else {
            self.point = None;
        }
    }

    fn call_function(&self) {
        if let Some(func) = self.point {
            func();
        } else {
            println!("No valid function pointer set.");
        }
    }
}

fn main() {
    // Command-line Argumente einlesen
    let mut args: Vec<OsString> = env::args_os().into_iter().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <payload>", args[0].to_string_lossy());
        return;
    }

    let first_arg: OsString = args.remove(1);
    let input_bytes: &[u8] = first_arg.as_bytes();

    // Struktur initialisieren
    let mut hackvist = Hackvist::new();
    hackvist.add_input(input_bytes);
    hackvist.call_function();
}
