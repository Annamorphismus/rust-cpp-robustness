# Verwende das heruntergeladene Image als Basis
FROM opensuse/leap:latest

# Aktualisiere Pakete
RUN zypper refresh && \
    zypper update -y

# Installiere zusätzliche Pakete
RUN zypper install -y gcc gcc-c++ make gdb python3 git vim
RUN git clone https://github.com/cyrus-and/gdb-dashboard.git /opt/gdb-dashboard
RUN echo "source /opt/gdb-dashboard/.gdbinit" >> /root/.gdbinit

# Setze Arbeitsverzeichnis
WORKDIR /home/

# Kopiere Exploit und Server
COPY ./cpp/src/Bufferoverflow/server.cpp ./server.cpp
COPY ./cpp/src/Bufferoverflow/exploit.cpp ./exploit.cpp
COPY ./cpp/src/Bufferoverflow/setup.sh ./setup.sh

RUN chmod +x ./setup.sh

# Kompiliere Server und Exploit
RUN g++ -fno-stack-protector -g -o server server.cpp
RUN g++ -fno-stack-protector -g -o exploit exploit.cpp



# Standardbefehl
CMD ["/bin/bash"]

