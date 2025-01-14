# rust-cpp-rubustness
Implementation of my master project



Deaktivate ASLR:

        echo 0 | tee /proc/sys/kernel/randomize_va_space

Check ASLR:

        cat /proc/sys/kernel/randomize_va_space

## Bufferoverflow CPP


Compile the C++ Code as follows:

        g++ -fno-stack-protector -z execstack -O0 -g -o server server.cpp
        g++ -fno-stack-protector -z execstack -O0 -g -o client client.cpp

Start Exploit:
        ./server
        ./client 


## Bufferoverflow RUST


Compile Rust Server

        cargo build
        cargo run

Compile C++ Client

        g++ -fno-stack-protector -z execstack -O0 -g -o client client.cpp
        ./client 88


# Use After Free
## Szenario: Dateiverwaltungs-System mit Datei-Cache
Das System soll häufig verwendete Dateien im Speicher halten, um den Zugriff zu beschleunigen. 
Der Cache verwaltet Dateiobjekte, die geöffnet, gelesen und geschlossen werden können. 
Ein Fehler in der Verwaltung der Lebensdauer dieser Dateiobjekte kann dazu führen, dass Zeiger auf bereits geschlossene und freigegebene Dateiobjekte existieren, was zu Dangling Pointern und Use-After-Free (UAF) Schwachstellen führt.