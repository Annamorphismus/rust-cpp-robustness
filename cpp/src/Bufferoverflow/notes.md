# Deactivate ASLR
echo 0 | tee /proc/sys/kernel/randomize_va_space

# check ASLR
cat /proc/sys/kernel/randomize_va_space

# actvate core dump
ulimit -c unlimited

--------------------------------
echo 0 | tee /proc/sys/kernel/randomize_va_space


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

bash
Code kopieren
(gdb) print $rbp + 8
4. Offset berechnen
Nutzen Sie GDB, um die Differenz zu berechnen:

bash
Code kopieren
(gdb) print ($rbp + 8) - $rsi
Das Ergebnis ist der Offset zur Rücksprungadresse in Bytes.

Beispielausgabe
Angenommen:

$rsi = 0x7fffffffce00
$rbp + 8 = 0x7fffffffd038
Dann gibt der Befehl:

bash
Code kopieren
(gdb) print ($rbp + 8) - $rsi
die Differenz zurück:

plaintext
Code kopieren
$1 = 312
Der berechnete Offset ist 312 Bytes.

