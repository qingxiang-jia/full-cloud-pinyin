#include <chrono>
#include <future>
#include <iostream>
#include <memory>
#include <new>
#include <thread>
#include <functional>

template <class F, typename... Args>
void call_async(F&& fun, Args... param) {
    // Modified from https://stackoverflow.com/a/56834117/1509779
    auto futptr = std::make_shared<std::future<void>>();
    *futptr = std::async(std::launch::async, [futptr, fun, param...]() {
        fun(param...);
    });
}

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
        call_async(longOperation, in);
        std::cout << "next line after async\n";
    }
    return 0;
}
