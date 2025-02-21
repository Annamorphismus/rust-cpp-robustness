# rust-cpp-rubustness
Implementation of my master project


# Start Docker 

## Build

        docker build -t rust-cpp-robustness .
        
## Start

        docker run -it  --memory=512m \                                                                              1 ✘  22:33:16 
                --memory-swap=512m \
                --oom-kill-disable=false \
                --pids-limit=100 \
                --cpus=1 \
                --log-opt max-size=5m \
                --log-opt max-file=2 \
                --cap-add=SYS_PTRACE \
                --cap-add=SYS_ADMIN \
                --security-opt seccomp=unconfined \
                --security-opt apparmor=unconfined \
                --tmpfs /tmp:size=64m \
                --name buffer_overflow \
                rust-cpp-robustness




# Deaktivate ASLR:
You can use the setup.sh file or do:

        echo 0 | tee /proc/sys/kernel/randomize_va_space

Check ASLR:

        cat /proc/sys/kernel/randomize_va_space

# Compile C++

        cd  /cpp
        rm -rf build
        mkdir build
        cd ./build
        cmake -DCMAKE_BUILD_TYPE=Debug ..    
        make

## run

        ./src/[project name]/[binary]

# Compile Rust

        cd ./rust/[projekt name]
        cargo build
        cargo run --bin [binary]  


# Bufferoverflow
1. Compile C++ file in Rust folder:
          g++ -fno-stack-protector -z execstack -O0 -g -o client client.cpp

2. The offset size must be entered here as a transfer parameter. In Docker, the value is:
           - for the C++ version: “56”
           - for the Rust version: “88”
  
Please note that the address of the abracadabra function may need to be adjusted.


