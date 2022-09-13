// Your First C++ Program

#include <chrono>
#include <future>
#include <iostream>
#include <memory>
#include <new>
#include <thread>

void longOperation(std::string input) {
    std::this_thread::sleep_for(std::chrono::milliseconds(4000));
    std::cout << "You typed: " << input << "\n";
}

int main() {
    std::string in;

    std::cout << "q to quit, empty input to continue\n";

    while (true) {
        std::cout << "say:\n";
        std::cin >> in;
        if (in == "q") {
            return 0;
        }
        std::thread t(longOperation, in);
        t.detach();
        std::cout << "next line after async\n";
    }
    return 0;
}
