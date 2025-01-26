# rust-cpp-rubustness
Implementation of my master project

# Start Docker 

        docker build -t rust-cpp-robustness .
        docker run -it --privileged --network=host rust-cpp-robustness



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


