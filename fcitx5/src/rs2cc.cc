#include "rs2cc.h"
#include <memory>

void fcp::Rs2Cc::sayHello() const {
    std::cout << "hello~~~~";
}

void fcp::Rs2Cc::setEngine(QuweiEngine* engine) {
    this->engine = engine;
}

std::unique_ptr<fcp::Rs2Cc> fcp::newRs2Cc() {
    return std::unique_ptr<fcp::Rs2Cc>(new fcp::Rs2Cc());
}
