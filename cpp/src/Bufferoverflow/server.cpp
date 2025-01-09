#include <cstddef>
#include <cstring>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

constexpr static inline auto PORT = 1234;
constexpr static inline auto RECV_BUFFER_SIZE = 1024;

// Ziel-Funktion, die durch den Exploit aufgerufen werden soll
void print_abracadabra() { std::cout << "abracadabra" << std::endl; }

void hexdump(const void *addr, size_t length) {
  const unsigned char *ptr = (const unsigned char *)addr;
  const size_t bytesPerLine = 16; // typical grouping

  for (size_t i = 0; i < length; i += bytesPerLine) {
    // Print the offset (like the addresses GDB shows)
    printf("%08zx  ", i);

    // Print each byte in hex
    for (size_t j = 0; j < bytesPerLine; j++) {
      if (i + j < length) {
        printf("%02x ", ptr[i + j]);
      } else {
        printf("   "); // padding if line is short
      }
    }

    // Add spacing between hex and ASCII
    printf(" ");

    // Print the same bytes in ASCII (printable chars, else '.')
    for (size_t j = 0; j < bytesPerLine; j++) {
      if (i + j < length) {
        unsigned char c = ptr[i + j];
        if (isprint(c)) {
          printf("%c", c);
        } else {
          printf(".");
        }
      } else {
        // If we’re past the end, just pad
        break;
      }
    }

    printf("\n");
  }
}

void vulnerable_function(char *buffer, std::size_t len) {
  char local_buffer[32];

  // hexdump(buffer, len);

  std::memcpy(local_buffer, buffer, len);
  hexdump(local_buffer, 512);
}

int main() {
  // UDP-Socket erstellen
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  if (sockfd < 0) {
    perror("Socket erstellen fehlgeschlagen");
    return 1;
  }

  sockaddr_in server_addr = {};
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(PORT);
  server_addr.sin_addr.s_addr = INADDR_ANY;

  if (bind(sockfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
    perror("Bind fehlgeschlagen");
    close(sockfd);
    return 1;
  }

  std::cout << "[INFO] Server gestartet und wartet auf Daten am Port " << PORT
            << std::endl;

  while (true) {
    char buffer[RECV_BUFFER_SIZE];
    sockaddr_in client_addr = {};
    socklen_t client_len = sizeof(client_addr);

    auto received_bytes =
        recvfrom(sockfd, buffer, RECV_BUFFER_SIZE, 0,
                 (struct sockaddr *)&client_addr, &client_len);

    if (received_bytes < 0) {
      perror("Fehler beim Empfang von Daten");
      continue;
    }

    std::cout << "received " << received_bytes << " bytes" << std::endl;

    std::cout << "[INFO] Daten empfangen, rufe vulnerable_function() auf"
              << std::endl;

    vulnerable_function(buffer, received_bytes);
  }

  close(sockfd);
  return 0;
}
