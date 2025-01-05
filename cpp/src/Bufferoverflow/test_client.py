import socket

SERVER_IP = "127.0.0.1"
SERVER_PORT = 1234

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
message = b"Testnachricht"
sock.sendto(message, (SERVER_IP, SERVER_PORT))
print("Nachricht gesendet!")
