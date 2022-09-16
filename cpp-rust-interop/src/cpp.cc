#include "cxx.h"
#include "cpp.h"

#include <string>
#include <iostream>

void print(rust::String s) {
  std::cout << std::string(s.c_str());
}