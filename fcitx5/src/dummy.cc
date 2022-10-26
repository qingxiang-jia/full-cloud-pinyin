#include "dummy.h"
#include <memory>

void fcp::Dummy::sayHello() const {
    std::cout << "hello~~~~";
}

std::unique_ptr<fcp::Dummy> fcp::newDummy() {
    return std::unique_ptr<fcp::Dummy>(new fcp::Dummy());
}
