# Verwende openSUSE Tumbleweed als Basis
FROM opensuse/tumbleweed

# Aktualisiere alle Pakete (optional, da Tumbleweed immer aktuell ist)
RUN zypper refresh && zypper update -y

# Installiere die benötigten Tools
RUN zypper install -y gcc gcc-c++ make gdb python3 git vim curl cmake

# Optional: Überprüfe die GCC-Version
RUN gcc --version && g++ --version

# Installiere Rust
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
RUN chmod +x ./setup.sh

# Standardbefehl
CMD ["/bin/bash"]
