# Deactivate ASLR
echo 0 | tee /proc/sys/kernel/randomize_va_space

# check ASLR
cat /proc/sys/kernel/randomize_va_space

# actvate core dump
ulimit -c unlimited

------------------------------

Buffer Adrr.: 0x7fffffffce00
Rücksprungadresse: 0x004013b0

Um den Offset von der Buffer-Adresse ($rsi) zur Rücksprungadresse ($rbp + 8) zu bestimmen:

Rücksprungadresse: $rbp + 8 = 0x7fffffffd040
Buffer-Adresse: $rsi = 0x7fffffffce00


Schritte zur Berechnung des Offsets in GDB
1. Server starten und Breakpoint setzen
Starten Sie den Server in GDB und setzen Sie einen Breakpoint bei recvfrom:


gdb ./server
(gdb) break recvfrom
(gdb) run

2. Exploit ausführen
Führen Sie den Exploit aus, sodass der Server beim Breakpoint stoppt.

3. Adressen anzeigen
Überprüfen Sie die relevanten Adressen:

bash
Code kopieren
(gdb) info registers
Notieren Sie:

$rsi (Adresse des Buffers)
$rbp + 8 (Rücksprungadresse)
Sie können die Rücksprungadresse direkt mit:
print $rbp + 8

Offset berechnen:
print ($rbp + 8) - $rsi

Offset=saved rip−buf

232 =  0xf6 0x11 0x40 0x0 0x0 0x0 0x0 0x0