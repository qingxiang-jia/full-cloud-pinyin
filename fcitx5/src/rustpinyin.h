#ifndef _FCITX5_QUWEI_RUSTPINYIN_H_
#define _FCITX5_QUWEI_RUSTPINYIN_H_

#include "../../fcpinyin/ffi.h"
#include "rs2cc.h"

class RustPinyin {
public:
    fcp::RustPinyinEngine* fcp;
    RustPinyin();
};

#endif // _FCITX5_QUWEI_RUSTPINYIN_H_
