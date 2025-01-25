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


