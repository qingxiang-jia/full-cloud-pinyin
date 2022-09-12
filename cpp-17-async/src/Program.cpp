// Your First C++ Program

#include <iostream>
#include <chrono>
#include <thread>

void longOperation() {
    std::this_thread::sleep_for(std::chrono::milliseconds(3000));
    std::cout << "Long operation is completed.\n";
}

int main() {
    std::cout << "Hello World!\n";
    longOperation();
    return 0;
}
