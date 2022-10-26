#ifndef _FCITX5_QUWEI_DUMMY_H_
#define _FCITX5_QUWEI_DUMMY_H_

#include <iostream>
#include <memory>

namespace fcp {
class Dummy {
public:
    void sayHello() const;
};

std::unique_ptr<Dummy> newDummy();
}

#endif // _FCITX5_QUWEI_DUMMY_H_