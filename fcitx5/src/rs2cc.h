#ifndef _FCITX5_QUWEI_DUMMY_H_
#define _FCITX5_QUWEI_DUMMY_H_

#include <iostream>
#include <memory>

class QuweiEngine;
namespace rust {
    inline namespace cxxbridge1 {
        class String;
        template <typename T> class Vec;
    } 
}
        

namespace fcp {
class Rs2Cc {
public:
    void sayHello() const;
    void setState(rust::String preedit, rust::Vec<::rust::String> candidates) const;
    void commit(int idx) const;
    void pageUp() const;
    void pageDown() const;
    void setEngine(QuweiEngine* engine);
    QuweiEngine* engine;
};

std::unique_ptr<Rs2Cc> newRs2Cc();
}

#endif // _FCITX5_QUWEI_RS2CC_H_