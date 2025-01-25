# Verwende das heruntergeladene Image als Basis
FROM opensuse/leap:latest

# Aktualisiere Pakete
RUN zypper refresh && \
    zypper update -y

# Installiere zusätzliche Pakete
RUN zypper install -y gcc gcc-c++ make gdb python3 git vim curl cmake

# Installiere Rust (verwende rustup für die Installation)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Clone gdb-dashboard und konfigurieren
RUN git clone https://github.com/cyrus-and/gdb-dashboard.git /opt/gdb-dashboard
RUN echo "source /opt/gdb-dashboard/.gdbinit" >> /root/.gdbinit

# Setze Arbeitsverzeichnis
WORKDIR /home/

# Kopiere C++- und Rust-Dateien
COPY . .



# Setup file
WORKDIR /home/
RUN chmod +x ./setup.sh

# Standardbefehl
CMD ["/bin/bash"]
