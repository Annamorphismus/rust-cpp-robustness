#include <cstddef>
#include <cstring>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

// Port und Puffergröße definieren
constexpr static inline auto PORT = 1234;             // Portnummer für den Server
constexpr static inline auto RECV_BUFFER_SIZE = 1024; // Maximale Größe für empfangene Daten

// Ziel-Funktion, die möglicherweise durch einen Exploit aufgerufen wird
void print_abracadabra() { std::cout << "abracadabra" << std::endl; }

// Funktion mit Sicherheitslücke (Buffer Overflow möglich)
void vulnerable_function(char* buffer, std::size_t len)
{
    char local_buffer[32];                  // Lokaler Puffer von fester Größe
    std::memcpy(local_buffer, buffer, len); // Daten in den lokalen Puffer kopieren
}

int main()
{
    // Erstelle einen UDP-Socket
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("Fehler beim Erstellen des Sockets");
        return 1;
    }

    // Server-Adresse konfigurieren
    sockaddr_in server_addr = {};
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT); // Portnummer setzen
    server_addr.sin_addr.s_addr = INADDR_ANY;

    // Socket an die Adresse binden
    if (bind(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("Fehler beim Binden");
        close(sockfd);
        return 1;
    }

    std::cout << "[INFO] Server gestartet, wartet auf Daten auf Port " << PORT << std::endl;

    while (true) {
        char buffer[RECV_BUFFER_SIZE];              // Puffer für empfangene Daten
        sockaddr_in client_addr = {};               // Adresse des Clients
        socklen_t client_len = sizeof(client_addr); // Größe der Client-Adresse

        // Daten vom Client empfangen
        auto received_bytes = recvfrom(sockfd, buffer, RECV_BUFFER_SIZE, 0,
                                       (struct sockaddr*)&client_addr, &client_len);
        if (received_bytes < 0) {
            perror("Fehler beim Empfang von Daten");
            continue;
        }

        std::cout << "Empfangen: " << received_bytes << " Bytes" << std::endl;

        // Verwundbare Funktion aufrufen
        vulnerable_function(buffer, received_bytes);
    }

    close(sockfd); // Socket schließen
    return 0;
}
