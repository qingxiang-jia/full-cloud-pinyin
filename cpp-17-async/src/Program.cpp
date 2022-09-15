#include <chrono>
#include <future>
#include <iostream>
#include <memory>
#include <new>
#include <thread>
#include <functional>



template <class F, typename... Args>
void call_async(F&& fun) {
    // https://stackoverflow.com/a/56834117/1509779
    auto futptr = std::make_shared<std::future<void>>();
    *futptr = std::async(std::launch::async, [futptr, fun]() {
        fun();
    });
}

void longOperation(std::string input) {
    std::this_thread::sleep_for(std::chrono::milliseconds(4000));
    std::cout << "You typed: " << input << "\n";
}

void fireAndForgetLongOp(std::string input, std::function<void (std::string)> fn) {
    auto futPtr = std::make_shared<std::future<void>>();
    *futPtr = std::async(std::launch::async, [futPtr, fn, input]() {
        fn(input);
    });
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
        fireAndForgetLongOp(in, longOperation);
        std::cout << "next line after async\n";
    }
    return 0;
}
