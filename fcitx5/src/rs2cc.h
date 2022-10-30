#ifndef _FCITX5_QUWEI_DUMMY_H_
#define _FCITX5_QUWEI_DUMMY_H_

#include <iostream>
#include <memory>

class QuweiEngine;

namespace fcp {
class Rs2Cc {
public:
    void sayHello() const;
    void setEngine(QuweiEngine* engine);
    QuweiEngine* engine;
};

std::unique_ptr<Rs2Cc> newRs2Cc();
}

#endif // _FCITX5_QUWEI_RS2CC_H_