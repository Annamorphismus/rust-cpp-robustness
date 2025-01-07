use std::env;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;

#[allow(dead_code)]
fn abracadabra() {
    println!("Abracadabra! Function called!");
}

#[repr(C)]
struct Hackvist {
    buffer: [u8; 16], // Ein Puffer, der überschrieben werden kann
    point: *const (), // Ein Funktionszeiger, der manipuliert wird
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
    let mut hackvist = Hackvist {
        buffer: [0; 16],
        point: 0 as *const (),
    };

    // Speicherlayout der Struktur prüfen
    // println!(
    //     "Size of Hackvist struct: {} bytes",
    //     mem::size_of::<Hackvist>()
    //);
    // println!(
    //    "Alignment of Hackvist struct: {} bytes",
    //     mem::align_of::<Hackvist>()
    //);
    //println!(
    //   "Offset of buffer: {} bytes",
    //  mem::offset_of!(Hackvist, buffer)
    //);
    //println!(
    //   "Offset of point: {} bytes",
    //   mem::offset_of!(Hackvist, point)
    // );

    // Adressen der Felder in der Struktur ausgeben
    //println!(
    //  "Hackvist structure: buffer starts at {:?}, point starts at {:?}",
    //  &hackvist.buffer as *const _, &hackvist.point as *const _
    // );

    // Speicher vor dem Kopieren anzeigen
    // println!("Before copy: Hackvist buffer: {:?}", hackvist.buffer);
    //println!("Before copy: Hackvist point: {:p}", hackvist.point);

    // Unsicherer Speicherzugriff (Kopieren der Eingabe in den Speicher)
    unsafe {
        std::ptr::copy(
            input_bytes.as_ptr(),
            hackvist.buffer.as_mut_ptr(),
            std::cmp::min(input_bytes.len(), 32), // Bis zu 32 Bytes kopieren
        );
    }

    // Speicher nach dem Kopieren anzeigen
    // println!("After copy: Hackvist buffer: {:?}", hackvist.buffer);
    //println!("After copy: Hackvist point: {:p}", hackvist.point);

    // Gesamten Speicherbereich der Struktur anzeigen
    //println!("Hackvist raw memory: {:?}", unsafe {
    //  std::slice::from_raw_parts(
    //    &hackvist as *const _ as *const u8,
    //  mem::size_of::<Hackvist>(),
    //)
    //});

    // Adresse der abracadabra-Funktion anzeigen
    //println!("Address of abracadabra: {:p}", abracadabra as *const ());

    // Exploit prüfen
    if hackvist.point.is_null() {
        println!("Point is null. Try again.");
    } else {
        // Funktionszeiger aufrufen (unsicher)
        // println!("Calling function at overwritten pointer...");
        let func: fn() = unsafe { std::mem::transmute(hackvist.point) };
        func();
    }
}
