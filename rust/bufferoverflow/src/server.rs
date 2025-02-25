use std::io;
use std::net::UdpSocket;
use std::ptr::copy_nonoverlapping;

const PORT: u16 = 1234; // Der Server lauscht auf diesem UDP-Port
const RECV_BUFFER_SIZE: usize = 1024; // Größe des Empfangspuffers für eingehende UDP-Pakete

// Funktion, die bei erfolgreichem Exploit aufgerufen werden könnte
#[inline(never)]
fn print_abracadabra() {
    println!("abracadabra");
}

// Unsichere Funktion mit einer potenziellen Buffer Overflow-Schwachstelle
#[inline(never)]
unsafe fn vulnerable_function(buffer: *const u8, len: usize) {
    let mut local_buffer = [0u8; 32];

    // Kopiert Daten vom übergebenen Zeiger in den lokalen Puffer, ohne eine Längenprüfung
    // Dies kann zu einem Buffer Overflow führen, wenn `len > 32` ist.
    copy_nonoverlapping(buffer, local_buffer.as_mut_ptr(), len);
}

fn main() -> io::Result<()> {
    // Gibt die Speicheradresse der Funktion `print_abracadabra` aus
    // Diese Adresse könnte von einem Angreifer genutzt werden, um eine Exploit-Payload zu erstellen
    println!(
        "abracadabra function address: x{:0x}",
        print_abracadabra as usize
    );

    // Erstellt einen UDP-Socket und bindet ihn an Port 1234
    let socket = UdpSocket::bind(("0.0.0.0", PORT))?;
    println!(
        "[INFO] Server gestartet und wartet auf Daten am Port {}",
        PORT
    );
    // Endlosschleife: Der Server wartet auf eingehende UDP-Pakete
    loop {
        // Puffer zum Speichern empfangener Daten
        let mut buffer = [0u8; RECV_BUFFER_SIZE];

        // Empfang von Daten von einem beliebigen Client
        let (received_bytes, src_addr) = socket.recv_from(&mut buffer)?;

        // Gibt die Anzahl der empfangenen Bytes und die Quelladresse aus
        println!("received {} bytes from {:?}", received_bytes, src_addr);
        println!("[INFO] Daten empfangen, rufe vulnerable_function() auf");

        // Übergibt den Puffer an die unsichere Funktion
        // Wenn `received_bytes > 32`, kommt es zu einem Buffer Overflow
        unsafe {
            vulnerable_function(buffer.as_ptr(), received_bytes);
        }
    }
}
