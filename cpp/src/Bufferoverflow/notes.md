# Deactivate ASLR
echo 0 | tee /proc/sys/kernel/randomize_va_space

# check ASLR
cat /proc/sys/kernel/randomize_va_space

# actvate core dump
ulimit -c unlimited


echo 0 | tee /proc/sys/kernel/randomize_va_space




Buffer Adrr.: 0x7fffffffce00
Rücksprungadresse: 0x004013b0

Um den Offset von der Buffer-Adresse ($rsi) zur Rücksprungadresse ($rbp + 8) zu bestimmen:

Rücksprungadresse: $rbp + 8 = 0x7fffffffd040
Buffer-Adresse: $rsi = 0x7fffffffce00
Offset:
bash
Code kopieren
Offset = 0x7fffffffd040 - 0x7fffffffce00 = 0x240 (576 Bytes)