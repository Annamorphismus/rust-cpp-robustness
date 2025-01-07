use std::net::UdpSocket;

fn print_magic() {
    println!("Abracadabra");
}

fn main() {
    // Größe des Buffers
    const BUFFER_SIZE: usize = 16;

    // UDP-Socket erstellen und an Adresse binden
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("Couldn't bind to address");
    println!("UDP Server listening on 127.0.0.1:3400");

    // Debug: Adresse von `print_magic` ausgeben
    println!("Address of print_magic: {:p}", print_magic as *const ());

    loop {
        // Buffer erstellen
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut recv_buffer = [0u8; 512]; // Größerer Empfangspuffer

        // Daten empfangen
        let (number_of_bytes, src_addr) = socket
            .recv_from(&mut recv_buffer)
            .expect("Failed to receive data");

        println!("Received {} bytes from {}", number_of_bytes, src_addr);

        // Unsicherer Zugriff: Buffer Overflow auslösen
        unsafe {
            let buffer_ptr = buffer.as_mut_ptr();

            for i in 0..number_of_bytes {
                // Schreibe Daten in den Buffer oder darüber hinaus
                let target_ptr = buffer_ptr.add(i);
                *target_ptr = recv_buffer[i];
                println!(
                    "Writing byte {:02x} to address {:p}",
                    recv_buffer[i], target_ptr
                );
            }
        }

        println!("If you see 'Abracadabra', the buffer overflow succeeded!");
    }
}
