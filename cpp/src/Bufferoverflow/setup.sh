#!/bin/bash

# ASLR deaktivieren
echo 0 > /proc/sys/kernel/randomize_va_space

# W^X Protection deaktivieren (falls unterstützt)
echo 0 > /proc/sys/kernel/exec-shield || true

echo "ASLR und W^X Protection wurden deaktiviert."

