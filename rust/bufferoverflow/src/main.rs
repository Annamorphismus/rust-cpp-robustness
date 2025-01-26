use std::io;
use std::net::UdpSocket;
use std::ptr::copy_nonoverlapping;

const PORT: u16 = 1234;
const RECV_BUFFER_SIZE: usize = 1024;

#[inline(never)]
fn print_abracadabra() {
    println!("abracadabra");
}

#[inline(never)]
unsafe fn vulnerable_function(buffer: *const u8, len: usize) {
    let mut local_buffer = [0u8; 32];

    copy_nonoverlapping(buffer, local_buffer.as_mut_ptr(), len);
}

fn main() -> io::Result<()> {
    println!(
        "abracadabra function address: x{:0x}",
        print_abracadabra as usize
    );
    let socket = UdpSocket::bind(("0.0.0.0", PORT))?;
    println!(
        "[INFO] Server gestartet und wartet auf Daten am Port {}",
        PORT
    );

    loop {
        let mut buffer = [0u8; RECV_BUFFER_SIZE];
        // Receive data from any client
        let (received_bytes, src_addr) = socket.recv_from(&mut buffer)?;

        println!("received {} bytes from {:?}", received_bytes, src_addr);
        println!("[INFO] Daten empfangen, rufe vulnerable_function() auf");

        unsafe {
            vulnerable_function(buffer.as_ptr(), received_bytes);
        }
    }
}
