#include <arpa/inet.h>
#include <errno.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

#define LOCAL_SERVER_PORT 1234
#define BUF 512 // Buffer size, intentionally large for overflow testing

// Funktion zum Verarbeiten empfangener Pakete
void handle_packet(char *buffer, int length, struct sockaddr_in *client_addr) {

  /* Adresse des Buffers anzeigen */
  printf("[DEBUG] Adresse des Buffers: %p\n", (void *)buffer);

  time_t current_time;
  char timestamp[BUF];

  // Debugging: Zeitangaben
  time(&current_time);
  strncpy(timestamp, ctime(&current_time), BUF);
  char *newline = strchr(timestamp, '\n');
  if (newline)
    *newline = '\0';

  // Debugging: Empfangene Daten anzeigen
  printf("[DEBUG] Empfangene Bytes: %d\n", length);
  printf("[DEBUG] Empfangene Daten (Hex): ");
  for (int i = 0; i < length; i++) {
    printf("%02x ", (unsigned char)buffer[i]);
  }
  printf("\n");

  // Nachricht anzeigen
  printf("[%s] Daten erhalten von %s:UDP%u: %.*s\n", timestamp,
         inet_ntoa(client_addr->sin_addr), ntohs(client_addr->sin_port), length,
         buffer);

  // Debugging: Speicheradresse des Puffers
  printf("[DEBUG] Speicheradresse des Buffers: %p\n", (void *)buffer);
}

int main(int argc, char **argv) {
  int server_socket, rc, received_bytes;
  socklen_t client_len;
  struct sockaddr_in client_addr, server_addr;
  char buffer[BUF];
  const int enable_reuse = 1;

  /* Erstelle UDP-Socket */
  server_socket = socket(AF_INET, SOCK_DGRAM, 0);
  if (server_socket < 0) {
    perror("Fehler beim Erstellen des Sockets");
    exit(EXIT_FAILURE);
  }

  /* Konfiguriere den Server */
  server_addr.sin_family = AF_INET;
  server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
  server_addr.sin_port = htons(LOCAL_SERVER_PORT);

  /* Port-Reuse aktivieren */
  setsockopt(server_socket, SOL_SOCKET, SO_REUSEADDR, &enable_reuse,
             sizeof(int));

  /* Binde den Socket */
  rc =
      bind(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr));
  if (rc < 0) {
    perror("Fehler beim Binden des Ports");
    exit(EXIT_FAILURE);
  } else {
    printf("[DEBUG] Server erfolgreich auf Port %d gebunden.\n",
           LOCAL_SERVER_PORT);
  }

  printf("[INFO] Wartet auf Daten am Port (UDP) %u\n", LOCAL_SERVER_PORT);

  /* Hauptschleife */
  while (1) {
    /* Initialisiere den Empfangspuffer */
    memset(buffer, 0, BUF);

    /* Empfang von Nachrichten */
    client_len = sizeof(client_addr);
    received_bytes = recvfrom(server_socket, buffer, BUF, 0,
                              (struct sockaddr *)&client_addr, &client_len);
    if (received_bytes < 0) {
      perror("[ERROR] Fehler beim Empfang");
      continue;
    }

    // Übergibt die empfangenen Daten an handle_packet
    handle_packet(buffer, received_bytes, &client_addr);
  }

  close(server_socket);
  return EXIT_SUCCESS;
}
