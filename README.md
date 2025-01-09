# rust-cpp-rubustness
Implementation of my master project


## Bufferoverflow

Deaktivate the ASLR:

        echo 0 | tee /proc/sys/kernel/randomize_va_space

Check ASLR:

        cat /proc/sys/kernel/randomize_va_space

Compile the C++ Programms as follows:

        g++ -fno-stack-protector -z execstack -O0 -g -o server server.cpp
        g++ -fno-stack-protector -z execstack -O0 -g -o client client.cpp

