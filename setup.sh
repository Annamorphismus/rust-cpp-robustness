#!/bin/bash

# ASLR deaktivieren
echo 0 > /proc/sys/kernel/randomize_va_space

echo "ASLR wurde deaktiviert."

