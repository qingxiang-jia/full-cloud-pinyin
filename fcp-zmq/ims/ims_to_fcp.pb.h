// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: ims_to_fcp.proto
// Protobuf C++ Version: 4.25.1

#ifndef GOOGLE_PROTOBUF_INCLUDED_ims_5fto_5ffcp_2eproto_2epb_2eh
#define GOOGLE_PROTOBUF_INCLUDED_ims_5fto_5ffcp_2eproto_2epb_2eh

#include <limits>
#include <string>
#include <type_traits>
#include <utility>

#include "google/protobuf/port_def.inc"
#if PROTOBUF_VERSION < 4025000
#error "This file was generated by a newer version of protoc which is"
#error "incompatible with your Protocol Buffer headers. Please update"
#error "your headers."
#endif  // PROTOBUF_VERSION

#if 4025001 < PROTOBUF_MIN_PROTOC_VERSION
#error "This file was generated by an older version of protoc which is"
#error "incompatible with your Protocol Buffer headers. Please"
#error "regenerate this file with a newer version of protoc."
#endif  // PROTOBUF_MIN_PROTOC_VERSION
#include "google/protobuf/port_undef.inc"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/arenastring.h"
#include "google/protobuf/generated_message_tctable_decl.h"
#include "google/protobuf/generated_message_util.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/generated_message_reflection.h"
#include "google/protobuf/message.h"
#include "google/protobuf/repeated_field.h"  // IWYU pragma: export
#include "google/protobuf/extension_set.h"  // IWYU pragma: export
#include "google/protobuf/generated_enum_reflection.h"
#include "google/protobuf/unknown_field_set.h"
// @@protoc_insertion_point(includes)

// Must be included last.
#include "google/protobuf/port_def.inc"

#define PROTOBUF_INTERNAL_EXPORT_ims_5fto_5ffcp_2eproto

namespace google {
namespace protobuf {
namespace internal {
class AnyMetadata;
}  // namespace internal
}  // namespace protobuf
}  // namespace google

// Internal implementation detail -- do not use these members.
struct TableStruct_ims_5fto_5ffcp_2eproto {
  static const ::uint32_t offsets[];
};
extern const ::google::protobuf::internal::DescriptorTable
    descriptor_table_ims_5fto_5ffcp_2eproto;
class FcitxEvent;
struct FcitxEventDefaultTypeInternal;
extern FcitxEventDefaultTypeInternal _FcitxEvent_default_instance_;
namespace google {
namespace protobuf {
}  // namespace protobuf
}  // namespace google

enum KeyEvent : int {
  NUM_0 = 0,
  NUM_1 = 1,
  NUM_2 = 2,
  NUM_3 = 3,
  NUM_4 = 4,
  NUM_5 = 5,
  NUM_6 = 6,
  NUM_7 = 7,
  NUM_8 = 8,
  NUM_9 = 9,
  A_LWR = 100,
  B_LWR = 101,
  C_LWR = 102,
  D_LWR = 103,
  E_LWR = 104,
  F_LWR = 105,
  G_LWR = 106,
  H_LWR = 107,
  I_LWR = 108,
  J_LWR = 109,
  K_LWR = 110,
  L_LWR = 111,
  M_LWR = 112,
  N_LWR = 113,
  O_LWR = 114,
  P_LWR = 115,
  Q_LWR = 116,
  R_LWR = 117,
  S_LWR = 118,
  T_LWR = 119,
  U_LWR = 120,
  V_LWR = 121,
  W_LWR = 122,
  X_LWR = 123,
  Y_LWR = 124,
  Z_LWR = 125,
  A_UPR = 200,
  B_UPR = 201,
  C_UPR = 202,
  D_UPR = 203,
  E_UPR = 204,
  F_UPR = 205,
  G_UPR = 206,
  H_UPR = 207,
  I_UPR = 208,
  J_UPR = 209,
  K_UPR = 210,
  L_UPR = 211,
  M_UPR = 212,
  N_UPR = 213,
  O_UPR = 214,
  P_UPR = 215,
  Q_UPR = 216,
  R_UPR = 217,
  S_UPR = 218,
  T_UPR = 219,
  U_UPR = 220,
  V_UPR = 221,
  W_UPR = 222,
  X_UPR = 223,
  Y_UPR = 224,
  Z_UPR = 225,
  COMMA = 10,
  PERIOD = 11,
  QEST_MARK = 12,
  EXCL_MARK = 13,
  SEMI_COLON = 14,
  DBL_QUOTE = 15,
  SGL_QUOTE = 16,
  BRKT_OPEN = 17,
  BRKT_CLOSE = 18,
  SLASH = 19,
  BACKSLASH = 20,
  ELLIPSIS = 21,
  ENTER = 30,
  SPACE = 31,
  MINUS = 32,
  EQUAL = 33,
  UP = 40,
  DOWN = 41,
  LEFT = 42,
  RIGHT = 43,
  SHIFT = 50,
  CTRL = 51,
  ALT = 52,
  KeyEvent_INT_MIN_SENTINEL_DO_NOT_USE_ =
      std::numeric_limits<::int32_t>::min(),
  KeyEvent_INT_MAX_SENTINEL_DO_NOT_USE_ =
      std::numeric_limits<::int32_t>::max(),
};

bool KeyEvent_IsValid(int value);
extern const uint32_t KeyEvent_internal_data_[];
constexpr KeyEvent KeyEvent_MIN = static_cast<KeyEvent>(0);
constexpr KeyEvent KeyEvent_MAX = static_cast<KeyEvent>(225);
constexpr int KeyEvent_ARRAYSIZE = 225 + 1;
const ::google::protobuf::EnumDescriptor*
KeyEvent_descriptor();
template <typename T>
const std::string& KeyEvent_Name(T value) {
  static_assert(std::is_same<T, KeyEvent>::value ||
                    std::is_integral<T>::value,
                "Incorrect type passed to KeyEvent_Name().");
  return ::google::protobuf::internal::NameOfEnum(KeyEvent_descriptor(), value);
}
inline bool KeyEvent_Parse(absl::string_view name, KeyEvent* value) {
  return ::google::protobuf::internal::ParseNamedEnum<KeyEvent>(
      KeyEvent_descriptor(), name, value);
}

// ===================================================================


// -------------------------------------------------------------------

class FcitxEvent final :
    public ::google::protobuf::Message /* @@protoc_insertion_point(class_definition:FcitxEvent) */ {
 public:
  inline FcitxEvent() : FcitxEvent(nullptr) {}
  ~FcitxEvent() override;
  template<typename = void>
  explicit PROTOBUF_CONSTEXPR FcitxEvent(::google::protobuf::internal::ConstantInitialized);

  inline FcitxEvent(const FcitxEvent& from)
      : FcitxEvent(nullptr, from) {}
  FcitxEvent(FcitxEvent&& from) noexcept
    : FcitxEvent() {
    *this = ::std::move(from);
  }

  inline FcitxEvent& operator=(const FcitxEvent& from) {
    CopyFrom(from);
    return *this;
  }
  inline FcitxEvent& operator=(FcitxEvent&& from) noexcept {
    if (this == &from) return *this;
    if (GetArena() == from.GetArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  inline const ::google::protobuf::UnknownFieldSet& unknown_fields() const
      ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return _internal_metadata_.unknown_fields<::google::protobuf::UnknownFieldSet>(::google::protobuf::UnknownFieldSet::default_instance);
  }
  inline ::google::protobuf::UnknownFieldSet* mutable_unknown_fields()
      ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return _internal_metadata_.mutable_unknown_fields<::google::protobuf::UnknownFieldSet>();
  }

  static const ::google::protobuf::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::google::protobuf::Descriptor* GetDescriptor() {
    return default_instance().GetMetadata().descriptor;
  }
  static const ::google::protobuf::Reflection* GetReflection() {
    return default_instance().GetMetadata().reflection;
  }
  static const FcitxEvent& default_instance() {
    return *internal_default_instance();
  }
  static inline const FcitxEvent* internal_default_instance() {
    return reinterpret_cast<const FcitxEvent*>(
               &_FcitxEvent_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(FcitxEvent& a, FcitxEvent& b) {
    a.Swap(&b);
  }
  inline void Swap(FcitxEvent* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetArena() != nullptr &&
        GetArena() == other->GetArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetArena() == other->GetArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::google::protobuf::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(FcitxEvent* other) {
    if (other == this) return;
    ABSL_DCHECK(GetArena() == other->GetArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  FcitxEvent* New(::google::protobuf::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<FcitxEvent>(arena);
  }
  using ::google::protobuf::Message::CopyFrom;
  void CopyFrom(const FcitxEvent& from);
  using ::google::protobuf::Message::MergeFrom;
  void MergeFrom( const FcitxEvent& from) {
    FcitxEvent::MergeImpl(*this, from);
  }
  private:
  static void MergeImpl(::google::protobuf::Message& to_msg, const ::google::protobuf::Message& from_msg);
  public:
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  ::size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::google::protobuf::internal::ParseContext* ctx) final;
  ::uint8_t* _InternalSerialize(
      ::uint8_t* target, ::google::protobuf::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const { return _impl_._cached_size_.Get(); }

  private:
  ::google::protobuf::internal::CachedSize* AccessCachedSize() const final;
  void SharedCtor(::google::protobuf::Arena* arena);
  void SharedDtor();
  void InternalSwap(FcitxEvent* other);

  private:
  friend class ::google::protobuf::internal::AnyMetadata;
  static ::absl::string_view FullMessageName() {
    return "FcitxEvent";
  }
  protected:
  explicit FcitxEvent(::google::protobuf::Arena* arena);
  FcitxEvent(::google::protobuf::Arena* arena, const FcitxEvent& from);
  public:

  static const ClassData _class_data_;
  const ::google::protobuf::Message::ClassData*GetClassData() const final;

  ::google::protobuf::Metadata GetMetadata() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kEventFieldNumber = 1,
  };
  // .KeyEvent event = 1;
  void clear_event() ;
  ::KeyEvent event() const;
  void set_event(::KeyEvent value);

  private:
  ::KeyEvent _internal_event() const;
  void _internal_set_event(::KeyEvent value);

  public:
  // @@protoc_insertion_point(class_scope:FcitxEvent)
 private:
  class _Internal;

  friend class ::google::protobuf::internal::TcParser;
  static const ::google::protobuf::internal::TcParseTable<
      0, 1, 0,
      0, 2>
      _table_;
  friend class ::google::protobuf::MessageLite;
  friend class ::google::protobuf::Arena;
  template <typename T>
  friend class ::google::protobuf::Arena::InternalHelper;
  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;
  struct Impl_ {

        inline explicit constexpr Impl_(
            ::google::protobuf::internal::ConstantInitialized) noexcept;
        inline explicit Impl_(::google::protobuf::internal::InternalVisibility visibility,
                              ::google::protobuf::Arena* arena);
        inline explicit Impl_(::google::protobuf::internal::InternalVisibility visibility,
                              ::google::protobuf::Arena* arena, const Impl_& from);
    int event_;
    mutable ::google::protobuf::internal::CachedSize _cached_size_;
    PROTOBUF_TSAN_DECLARE_MEMBER
  };
  union { Impl_ _impl_; };
  friend struct ::TableStruct_ims_5fto_5ffcp_2eproto;
};

// ===================================================================




// ===================================================================


#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// -------------------------------------------------------------------

// FcitxEvent

// .KeyEvent event = 1;
inline void FcitxEvent::clear_event() {
  PROTOBUF_TSAN_WRITE(&_impl_._tsan_detect_race);
  _impl_.event_ = 0;
}
inline ::KeyEvent FcitxEvent::event() const {
  // @@protoc_insertion_point(field_get:FcitxEvent.event)
  return _internal_event();
}
inline void FcitxEvent::set_event(::KeyEvent value) {
  _internal_set_event(value);
  // @@protoc_insertion_point(field_set:FcitxEvent.event)
}
inline ::KeyEvent FcitxEvent::_internal_event() const {
  PROTOBUF_TSAN_READ(&_impl_._tsan_detect_race);
  return static_cast<::KeyEvent>(_impl_.event_);
}
inline void FcitxEvent::_internal_set_event(::KeyEvent value) {
  PROTOBUF_TSAN_WRITE(&_impl_._tsan_detect_race);
  ;
  _impl_.event_ = value;
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)


namespace google {
namespace protobuf {

template <>
struct is_proto_enum<::KeyEvent> : std::true_type {};
template <>
inline const EnumDescriptor* GetEnumDescriptor<::KeyEvent>() {
  return ::KeyEvent_descriptor();
}

}  // namespace protobuf
}  // namespace google

// @@protoc_insertion_point(global_scope)

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_INCLUDED_ims_5fto_5ffcp_2eproto_2epb_2eh
