#include "../fcitx5/src/rs2cc.h"
#include "cxx.h"
#include "ffi.h"
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

extern "C" {
::std::size_t fcp$cxxbridge1$RustPinyinEngine$operator$sizeof() noexcept;
::std::size_t fcp$cxxbridge1$RustPinyinEngine$operator$alignof() noexcept;

::fcp::RustPinyinEngine *fcp$cxxbridge1$init() noexcept;

void fcp$cxxbridge1$RustPinyinEngine$query_candidates(const ::fcp::RustPinyinEngine &self, ::rust::Str preedit, ::rust::Vec<::fcp::CandidateWord> *return$) noexcept;

::fcp::Rs2Cc *fcp$cxxbridge1$newRs2Cc() noexcept {
  ::std::unique_ptr<::fcp::Rs2Cc> (*newRs2Cc$)() = ::fcp::newRs2Cc;
  return newRs2Cc$().release();
}

void fcp$cxxbridge1$Rs2Cc$sayHello(const ::fcp::Rs2Cc &self) noexcept {
  void (::fcp::Rs2Cc::*sayHello$)() const = &::fcp::Rs2Cc::sayHello;
  (self.*sayHello$)();
}

void fcp$cxxbridge1$Rs2Cc$setState(const ::fcp::Rs2Cc &self, const ::rust::String *preedit, const ::rust::Vec<::rust::String> *candidates) noexcept {
  void (::fcp::Rs2Cc::*setState$)(::rust::String, ::rust::Vec<::rust::String>) const = &::fcp::Rs2Cc::setState;
  (self.*setState$)(::rust::String(::rust::unsafe_bitcopy, *preedit), ::rust::Vec<::rust::String>(::rust::unsafe_bitcopy, *candidates));
}

void fcp$cxxbridge1$Rs2Cc$commit(const ::fcp::Rs2Cc &self, ::std::int32_t idx) noexcept {
  void (::fcp::Rs2Cc::*commit$)(::std::int32_t) const = &::fcp::Rs2Cc::commit;
  (self.*commit$)(idx);
}

void fcp$cxxbridge1$Rs2Cc$pageUp(const ::fcp::Rs2Cc &self) noexcept {
  void (::fcp::Rs2Cc::*pageUp$)() const = &::fcp::Rs2Cc::pageUp;
  (self.*pageUp$)();
}

void fcp$cxxbridge1$Rs2Cc$pageDown(const ::fcp::Rs2Cc &self) noexcept {
  void (::fcp::Rs2Cc::*pageDown$)() const = &::fcp::Rs2Cc::pageDown;
  (self.*pageDown$)();
}
} // extern "C"

::std::size_t RustPinyinEngine::layout::size() noexcept {
  return fcp$cxxbridge1$RustPinyinEngine$operator$sizeof();
}

::std::size_t RustPinyinEngine::layout::align() noexcept {
  return fcp$cxxbridge1$RustPinyinEngine$operator$alignof();
}

::rust::Box<::fcp::RustPinyinEngine> init() noexcept {
  return ::rust::Box<::fcp::RustPinyinEngine>::from_raw(fcp$cxxbridge1$init());
}

::rust::Vec<::fcp::CandidateWord> RustPinyinEngine::query_candidates(::rust::Str preedit) const noexcept {
  ::rust::MaybeUninit<::rust::Vec<::fcp::CandidateWord>> return$;
  fcp$cxxbridge1$RustPinyinEngine$query_candidates(*this, preedit, &return$.value);
  return ::std::move(return$.value);
}
} // namespace fcp

extern "C" {
::fcp::RustPinyinEngine *cxxbridge1$box$fcp$RustPinyinEngine$alloc() noexcept;
void cxxbridge1$box$fcp$RustPinyinEngine$dealloc(::fcp::RustPinyinEngine *) noexcept;
void cxxbridge1$box$fcp$RustPinyinEngine$drop(::rust::Box<::fcp::RustPinyinEngine> *ptr) noexcept;

void cxxbridge1$rust_vec$fcp$CandidateWord$new(const ::rust::Vec<::fcp::CandidateWord> *ptr) noexcept;
void cxxbridge1$rust_vec$fcp$CandidateWord$drop(::rust::Vec<::fcp::CandidateWord> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$fcp$CandidateWord$len(const ::rust::Vec<::fcp::CandidateWord> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$fcp$CandidateWord$capacity(const ::rust::Vec<::fcp::CandidateWord> *ptr) noexcept;
const ::fcp::CandidateWord *cxxbridge1$rust_vec$fcp$CandidateWord$data(const ::rust::Vec<::fcp::CandidateWord> *ptr) noexcept;
void cxxbridge1$rust_vec$fcp$CandidateWord$reserve_total(::rust::Vec<::fcp::CandidateWord> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$fcp$CandidateWord$set_len(::rust::Vec<::fcp::CandidateWord> *ptr, ::std::size_t len) noexcept;
void cxxbridge1$rust_vec$fcp$CandidateWord$truncate(::rust::Vec<::fcp::CandidateWord> *ptr, ::std::size_t len) noexcept;

static_assert(::rust::detail::is_complete<::fcp::Rs2Cc>::value, "definition of Rs2Cc is required");
static_assert(sizeof(::std::unique_ptr<::fcp::Rs2Cc>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::fcp::Rs2Cc>) == alignof(void *), "");
void cxxbridge1$unique_ptr$fcp$Rs2Cc$null(::std::unique_ptr<::fcp::Rs2Cc> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::fcp::Rs2Cc>();
}
void cxxbridge1$unique_ptr$fcp$Rs2Cc$raw(::std::unique_ptr<::fcp::Rs2Cc> *ptr, ::fcp::Rs2Cc *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::fcp::Rs2Cc>(raw);
}
const ::fcp::Rs2Cc *cxxbridge1$unique_ptr$fcp$Rs2Cc$get(const ::std::unique_ptr<::fcp::Rs2Cc>& ptr) noexcept {
  return ptr.get();
}
::fcp::Rs2Cc *cxxbridge1$unique_ptr$fcp$Rs2Cc$release(::std::unique_ptr<::fcp::Rs2Cc>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$fcp$Rs2Cc$drop(::std::unique_ptr<::fcp::Rs2Cc> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::fcp::Rs2Cc>::value>{}(ptr);
}
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::fcp::RustPinyinEngine *Box<::fcp::RustPinyinEngine>::allocation::alloc() noexcept {
  return cxxbridge1$box$fcp$RustPinyinEngine$alloc();
}
template <>
void Box<::fcp::RustPinyinEngine>::allocation::dealloc(::fcp::RustPinyinEngine *ptr) noexcept {
  cxxbridge1$box$fcp$RustPinyinEngine$dealloc(ptr);
}
template <>
void Box<::fcp::RustPinyinEngine>::drop() noexcept {
  cxxbridge1$box$fcp$RustPinyinEngine$drop(this);
}
template <>
Vec<::fcp::CandidateWord>::Vec() noexcept {
  cxxbridge1$rust_vec$fcp$CandidateWord$new(this);
}
template <>
void Vec<::fcp::CandidateWord>::drop() noexcept {
  return cxxbridge1$rust_vec$fcp$CandidateWord$drop(this);
}
template <>
::std::size_t Vec<::fcp::CandidateWord>::size() const noexcept {
  return cxxbridge1$rust_vec$fcp$CandidateWord$len(this);
}
template <>
::std::size_t Vec<::fcp::CandidateWord>::capacity() const noexcept {
  return cxxbridge1$rust_vec$fcp$CandidateWord$capacity(this);
}
template <>
const ::fcp::CandidateWord *Vec<::fcp::CandidateWord>::data() const noexcept {
  return cxxbridge1$rust_vec$fcp$CandidateWord$data(this);
}
template <>
void Vec<::fcp::CandidateWord>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$fcp$CandidateWord$reserve_total(this, new_cap);
}
template <>
void Vec<::fcp::CandidateWord>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$fcp$CandidateWord$set_len(this, len);
}
template <>
void Vec<::fcp::CandidateWord>::truncate(::std::size_t len) {
  return cxxbridge1$rust_vec$fcp$CandidateWord$truncate(this, len);
}
} // namespace cxxbridge1
} // namespace rust
