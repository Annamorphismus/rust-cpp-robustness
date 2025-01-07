#include <cstdint>
#include <cstring>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

#define SERVER_PORT 1234 // Port, auf dem der Server lauscht
#define BUFFER_SIZE 256  // Größe des Buffers für empfangene Daten

// Funktion, die bei erfolgreichem Angriff aufgerufen werden soll
void print_abracadabra() { std::cout << "abracadabra" << std::endl; }

int main() {
  // UDP-Socket erstellen
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  if (sockfd < 0) {
    perror("Socket erstellen fehlgeschlagen");
    return 1;
  }

  // Server-Adresse einrichten
  sockaddr_in server_addr = {};
  server_addr.sin_family = AF_INET;
  server_addr.sin_port =
      htons(SERVER_PORT); // Portnummer in Netzwerk-Byte-Reihenfolge
  server_addr.sin_addr.s_addr =
      INADDR_ANY; // Hört auf allen verfügbaren Schnittstellen

  // Socket an die Adresse binden
  if (bind(sockfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
    perror("Bind fehlgeschlagen");
    close(sockfd);
    return 1;
  }

  std::cout << "[INFO] Server gestartet und wartet auf Daten am Port "
            << SERVER_PORT << std::endl;

  while (true) {
    char buffer[BUFFER_SIZE]; // Buffer für eingehende Daten
    sockaddr_in client_addr = {};
    socklen_t client_len = sizeof(client_addr);

    // Daten vom Client empfangen
    ssize_t received_bytes =
        recvfrom(sockfd, buffer, BUFFER_SIZE, 0,
                 (struct sockaddr *)&client_addr, &client_len);
    if (received_bytes < 0) {
      perror("Fehler beim Empfang von Daten");
      continue;
    }

    std::cout << "[DEBUG] Empfangene Bytes: " << received_bytes << std::endl;

    // **Schritt 1: Debugging des Empfangenen Buffers**
    std::cout << "[DEBUG] Empfangener Buffer-Inhalt:" << std::endl;
    for (int i = 0; i < BUFFER_SIZE; i++) {
      std::cout << "0x" << std::hex << (unsigned int)(unsigned char)buffer[i]
                << " ";
      if ((i + 1) % 16 == 0)
        std::cout << std::endl;
    }

    // Interpretiere die Rücksprungadresse aus dem Buffer
    uintptr_t return_addr = *(reinterpret_cast<uintptr_t *>(
        buffer + BUFFER_SIZE - sizeof(uintptr_t)));
    std::cout << "[DEBUG] Rücksprungadresse (direkt gelesen): 0x" << std::hex
              << return_addr << std::dec << std::endl;

    // **Schritt 2: Überprüfen des Offsets**
    uintptr_t *calculated_return_address =
        reinterpret_cast<uintptr_t *>(buffer + BUFFER_SIZE - sizeof(uintptr_t));
    std::cout << "[DEBUG] Rücksprungadresse aus berechnetem Offset: 0x"
              << std::hex << *calculated_return_address << std::dec
              << std::endl;

    // Adresse der Ziel-Funktion ermitteln
    void (*func_ptr)() = print_abracadabra;
    uintptr_t func_addr = reinterpret_cast<uintptr_t>(func_ptr);
    std::cout << "[DEBUG] Adresse von print_abracadabra: 0x" << std::hex
              << func_addr << std::dec << std::endl;

    // Prüfe, ob die Rücksprungadresse auf print_abracadabra zeigt
    if (return_addr == func_addr) {
      std::cout << "[INFO] Rücksprungadresse zeigt auf print_abracadabra. "
                   "Springen zur Funktion!"
                << std::endl;
      print_abracadabra();
    } else {
      std::cout << "[INFO] Rücksprungadresse zeigt nicht auf "
                   "print_abracadabra. Keine Aktion."
                << std::endl;
    }
  }

  close(sockfd); // Socket schließen
  return 0;
}
