#pragma once
#include "../fcitx5/src/rs2cc.h"
#include "cxx.h"
#include <algorithm>
#include <array>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <new>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>

namespace fcp {
  struct CandidateWord;
  struct RustPinyinEngine;
  using Rs2Cc = ::fcp::Rs2Cc;
}

namespace fcp {
#ifndef CXXBRIDGE1_STRUCT_fcp$CandidateWord
#define CXXBRIDGE1_STRUCT_fcp$CandidateWord
struct CandidateWord final {
  ::rust::String word;
  ::std::int32_t len;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_fcp$CandidateWord

#ifndef CXXBRIDGE1_STRUCT_fcp$RustPinyinEngine
#define CXXBRIDGE1_STRUCT_fcp$RustPinyinEngine
struct RustPinyinEngine final : public ::rust::Opaque {
  ::rust::Vec<::fcp::CandidateWord> query_candidates(::rust::Str preedit) const noexcept;
  ~RustPinyinEngine() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_fcp$RustPinyinEngine

::rust::Box<::fcp::RustPinyinEngine> init() noexcept;
} // namespace fcp
