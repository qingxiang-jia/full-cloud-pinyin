#ifndef _FCITX5_QUWEI_RUSTPINYIN_H_
#define _FCITX5_QUWEI_RUSTPINYIN_H_

#include "../../fcpinyin/ffi.h"

class RustPinyin {
public:
    fcp::RustPinyinEngine* fcp;
    RustPinyin();
    ::rust::Vec<::fcp::CandidateWord> queryCandidates(std::string preedit);
};

#endif // _FCITX5_QUWEI_RUSTPINYIN_H_
