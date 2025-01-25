# rust-cpp-rubustness
Implementation of my master project


Deaktivate ASLR:

        echo 0 | tee /proc/sys/kernel/randomize_va_space

Check ASLR:

        cat /proc/sys/kernel/randomize_va_space

# Compile C++
cd  /cpp
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


