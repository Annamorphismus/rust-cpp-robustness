./target/debug/buffer_overflow $(python3 -c "import sys; sys.stdout.buffer.write(b'A' * 16 + b'\xd0\xe9\x55\x55\x55\x55\x00\x00')")


1. Ziel
Manipulation einer Speicherstruktur, um den Funktionszeiger (hackvist.point) auf die Funktion abracadabra umzuleiten und sie auszuführen.
2. Grundlagen des Exploits
Buffer Overflow:
Ein Puffer (hackvist.buffer) mit einer festen Größe wird mit mehr Daten überschrieben, als er aufnehmen kann.
Dadurch werden angrenzende Speicherbereiche überschrieben (hier: der Funktionszeiger hackvist.point).
3. Speicherlayout
Struktur Hackvist:
buffer: 16 Bytes groß, beginnt bei Offset 0.
point: Ein Zeiger, beginnt bei Offset 16.
Der Abstand zwischen buffer und point beträgt 16 Bytes. Überschüssige Daten im buffer überschreiben den Zeiger point.
4. Funktionsweise des Exploits
Payload:
16 Bytes (As), um den buffer zu füllen.
Die Adresse der Funktion abracadabra (0x55555555e9d0) im Little-Endian-Format, um point zu überschreiben.
Unsicherer Speicherzugriff:
Mit std::ptr::copy wird der Payload ohne Längenprüfung in den buffer kopiert.
Da der Payload länger als 16 Bytes ist, wird point überschrieben.
Aufruf von point:
Der manipulierte point zeigt auf abracadabra.
Das Programm ruft die Adresse auf, wodurch die Funktion ausgeführt wird.
