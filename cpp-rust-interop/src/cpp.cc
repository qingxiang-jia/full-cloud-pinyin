#include "rust/cxx.h"
#include "cpp-rust-interop/src/cpp.h"

#include <string>
#include <iostream>

void print(rust::String s) {
  std::cout << std::string(s.c_str());
}