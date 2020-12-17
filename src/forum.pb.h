// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: forum.proto

#ifndef GOOGLE_PROTOBUF_INCLUDED_forum_2eproto
#define GOOGLE_PROTOBUF_INCLUDED_forum_2eproto

#include <limits>
#include <string>

#include <google/protobuf/port_def.inc>
#if PROTOBUF_VERSION < 3013000
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers. Please update
#error your headers.
#endif
#if 3013000 < PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers. Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/port_undef.inc>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_table_driven.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/inlined_string_field.h>
#include <google/protobuf/metadata_lite.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/message.h>
#include <google/protobuf/repeated_field.h>  // IWYU pragma: export
#include <google/protobuf/extension_set.h>  // IWYU pragma: export
#include <google/protobuf/unknown_field_set.h>
#include "nut.pb.h"
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>
#define PROTOBUF_INTERNAL_EXPORT_forum_2eproto
PROTOBUF_NAMESPACE_OPEN
namespace internal {
class AnyMetadata;
}  // namespace internal
PROTOBUF_NAMESPACE_CLOSE

// Internal implementation detail -- do not use these members.
struct TableStruct_forum_2eproto {
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTableField entries[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::AuxiliaryParseTableField aux[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTable schema[1]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::FieldMetadata field_metadata[];
  static const ::PROTOBUF_NAMESPACE_ID::internal::SerializationTable serialization_table[];
  static const ::PROTOBUF_NAMESPACE_ID::uint32 offsets[];
};
extern const ::PROTOBUF_NAMESPACE_ID::internal::DescriptorTable descriptor_table_forum_2eproto;
class CreatePostRequest;
class CreatePostRequestDefaultTypeInternal;
extern CreatePostRequestDefaultTypeInternal _CreatePostRequest_default_instance_;
PROTOBUF_NAMESPACE_OPEN
template<> ::CreatePostRequest* Arena::CreateMaybeMessage<::CreatePostRequest>(Arena*);
PROTOBUF_NAMESPACE_CLOSE

// ===================================================================

class CreatePostRequest PROTOBUF_FINAL :
    public ::PROTOBUF_NAMESPACE_ID::Message /* @@protoc_insertion_point(class_definition:CreatePostRequest) */ {
 public:
  inline CreatePostRequest() : CreatePostRequest(nullptr) {}
  virtual ~CreatePostRequest();

  CreatePostRequest(const CreatePostRequest& from);
  CreatePostRequest(CreatePostRequest&& from) noexcept
    : CreatePostRequest() {
    *this = ::std::move(from);
  }

  inline CreatePostRequest& operator=(const CreatePostRequest& from) {
    CopyFrom(from);
    return *this;
  }
  inline CreatePostRequest& operator=(CreatePostRequest&& from) noexcept {
    if (GetArena() == from.GetArena()) {
      if (this != &from) InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* GetDescriptor() {
    return GetMetadataStatic().descriptor;
  }
  static const ::PROTOBUF_NAMESPACE_ID::Reflection* GetReflection() {
    return GetMetadataStatic().reflection;
  }
  static const CreatePostRequest& default_instance();

  static void InitAsDefaultInstance();  // FOR INTERNAL USE ONLY
  static inline const CreatePostRequest* internal_default_instance() {
    return reinterpret_cast<const CreatePostRequest*>(
               &_CreatePostRequest_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(CreatePostRequest& a, CreatePostRequest& b) {
    a.Swap(&b);
  }
  inline void Swap(CreatePostRequest* other) {
    if (other == this) return;
    if (GetArena() == other->GetArena()) {
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(CreatePostRequest* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetArena() == other->GetArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  inline CreatePostRequest* New() const final {
    return CreateMaybeMessage<CreatePostRequest>(nullptr);
  }

  CreatePostRequest* New(::PROTOBUF_NAMESPACE_ID::Arena* arena) const final {
    return CreateMaybeMessage<CreatePostRequest>(arena);
  }
  void CopyFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) final;
  void MergeFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) final;
  void CopyFrom(const CreatePostRequest& from);
  void MergeFrom(const CreatePostRequest& from);
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  ::PROTOBUF_NAMESPACE_ID::uint8* _InternalSerialize(
      ::PROTOBUF_NAMESPACE_ID::uint8* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _cached_size_.Get(); }

  private:
  inline void SharedCtor();
  inline void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(CreatePostRequest* other);
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "CreatePostRequest";
  }
  protected:
  explicit CreatePostRequest(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  private:
  static void ArenaDtor(void* object);
  inline void RegisterArenaDtor(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  public:

  ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadata() const final;
  private:
  static ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadataStatic() {
    ::PROTOBUF_NAMESPACE_ID::internal::AssignDescriptors(&::descriptor_table_forum_2eproto);
    return ::descriptor_table_forum_2eproto.file_level_metadata[kIndexInFileMessages];
  }

  public:

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kSubjectFieldNumber = 1,
    kContentTypeFieldNumber = 2,
    kBodyFieldNumber = 3,
  };
  // string subject = 1;
  void clear_subject();
  const std::string& subject() const;
  void set_subject(const std::string& value);
  void set_subject(std::string&& value);
  void set_subject(const char* value);
  void set_subject(const char* value, size_t size);
  std::string* mutable_subject();
  std::string* release_subject();
  void set_allocated_subject(std::string* subject);
  private:
  const std::string& _internal_subject() const;
  void _internal_set_subject(const std::string& value);
  std::string* _internal_mutable_subject();
  public:

  // string content_type = 2;
  void clear_content_type();
  const std::string& content_type() const;
  void set_content_type(const std::string& value);
  void set_content_type(std::string&& value);
  void set_content_type(const char* value);
  void set_content_type(const char* value, size_t size);
  std::string* mutable_content_type();
  std::string* release_content_type();
  void set_allocated_content_type(std::string* content_type);
  private:
  const std::string& _internal_content_type() const;
  void _internal_set_content_type(const std::string& value);
  std::string* _internal_mutable_content_type();
  public:

  // string body = 3;
  void clear_body();
  const std::string& body() const;
  void set_body(const std::string& value);
  void set_body(std::string&& value);
  void set_body(const char* value);
  void set_body(const char* value, size_t size);
  std::string* mutable_body();
  std::string* release_body();
  void set_allocated_body(std::string* body);
  private:
  const std::string& _internal_body() const;
  void _internal_set_body(const std::string& value);
  std::string* _internal_mutable_body();
  public:

  // @@protoc_insertion_point(class_scope:CreatePostRequest)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr subject_;
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr content_type_;
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr body_;
  mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  friend struct ::TableStruct_forum_2eproto;
};
// ===================================================================


// ===================================================================

#ifdef __GNUC__
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// CreatePostRequest

// string subject = 1;
inline void CreatePostRequest::clear_subject() {
  subject_.ClearToEmpty(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline const std::string& CreatePostRequest::subject() const {
  // @@protoc_insertion_point(field_get:CreatePostRequest.subject)
  return _internal_subject();
}
inline void CreatePostRequest::set_subject(const std::string& value) {
  _internal_set_subject(value);
  // @@protoc_insertion_point(field_set:CreatePostRequest.subject)
}
inline std::string* CreatePostRequest::mutable_subject() {
  // @@protoc_insertion_point(field_mutable:CreatePostRequest.subject)
  return _internal_mutable_subject();
}
inline const std::string& CreatePostRequest::_internal_subject() const {
  return subject_.Get();
}
inline void CreatePostRequest::_internal_set_subject(const std::string& value) {
  
  subject_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), value, GetArena());
}
inline void CreatePostRequest::set_subject(std::string&& value) {
  
  subject_.Set(
    &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::move(value), GetArena());
  // @@protoc_insertion_point(field_set_rvalue:CreatePostRequest.subject)
}
inline void CreatePostRequest::set_subject(const char* value) {
  GOOGLE_DCHECK(value != nullptr);
  
  subject_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(value),
              GetArena());
  // @@protoc_insertion_point(field_set_char:CreatePostRequest.subject)
}
inline void CreatePostRequest::set_subject(const char* value,
    size_t size) {
  
  subject_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(
      reinterpret_cast<const char*>(value), size), GetArena());
  // @@protoc_insertion_point(field_set_pointer:CreatePostRequest.subject)
}
inline std::string* CreatePostRequest::_internal_mutable_subject() {
  
  return subject_.Mutable(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline std::string* CreatePostRequest::release_subject() {
  // @@protoc_insertion_point(field_release:CreatePostRequest.subject)
  return subject_.Release(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline void CreatePostRequest::set_allocated_subject(std::string* subject) {
  if (subject != nullptr) {
    
  } else {
    
  }
  subject_.SetAllocated(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), subject,
      GetArena());
  // @@protoc_insertion_point(field_set_allocated:CreatePostRequest.subject)
}

// string content_type = 2;
inline void CreatePostRequest::clear_content_type() {
  content_type_.ClearToEmpty(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline const std::string& CreatePostRequest::content_type() const {
  // @@protoc_insertion_point(field_get:CreatePostRequest.content_type)
  return _internal_content_type();
}
inline void CreatePostRequest::set_content_type(const std::string& value) {
  _internal_set_content_type(value);
  // @@protoc_insertion_point(field_set:CreatePostRequest.content_type)
}
inline std::string* CreatePostRequest::mutable_content_type() {
  // @@protoc_insertion_point(field_mutable:CreatePostRequest.content_type)
  return _internal_mutable_content_type();
}
inline const std::string& CreatePostRequest::_internal_content_type() const {
  return content_type_.Get();
}
inline void CreatePostRequest::_internal_set_content_type(const std::string& value) {
  
  content_type_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), value, GetArena());
}
inline void CreatePostRequest::set_content_type(std::string&& value) {
  
  content_type_.Set(
    &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::move(value), GetArena());
  // @@protoc_insertion_point(field_set_rvalue:CreatePostRequest.content_type)
}
inline void CreatePostRequest::set_content_type(const char* value) {
  GOOGLE_DCHECK(value != nullptr);
  
  content_type_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(value),
              GetArena());
  // @@protoc_insertion_point(field_set_char:CreatePostRequest.content_type)
}
inline void CreatePostRequest::set_content_type(const char* value,
    size_t size) {
  
  content_type_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(
      reinterpret_cast<const char*>(value), size), GetArena());
  // @@protoc_insertion_point(field_set_pointer:CreatePostRequest.content_type)
}
inline std::string* CreatePostRequest::_internal_mutable_content_type() {
  
  return content_type_.Mutable(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline std::string* CreatePostRequest::release_content_type() {
  // @@protoc_insertion_point(field_release:CreatePostRequest.content_type)
  return content_type_.Release(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline void CreatePostRequest::set_allocated_content_type(std::string* content_type) {
  if (content_type != nullptr) {
    
  } else {
    
  }
  content_type_.SetAllocated(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), content_type,
      GetArena());
  // @@protoc_insertion_point(field_set_allocated:CreatePostRequest.content_type)
}

// string body = 3;
inline void CreatePostRequest::clear_body() {
  body_.ClearToEmpty(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline const std::string& CreatePostRequest::body() const {
  // @@protoc_insertion_point(field_get:CreatePostRequest.body)
  return _internal_body();
}
inline void CreatePostRequest::set_body(const std::string& value) {
  _internal_set_body(value);
  // @@protoc_insertion_point(field_set:CreatePostRequest.body)
}
inline std::string* CreatePostRequest::mutable_body() {
  // @@protoc_insertion_point(field_mutable:CreatePostRequest.body)
  return _internal_mutable_body();
}
inline const std::string& CreatePostRequest::_internal_body() const {
  return body_.Get();
}
inline void CreatePostRequest::_internal_set_body(const std::string& value) {
  
  body_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), value, GetArena());
}
inline void CreatePostRequest::set_body(std::string&& value) {
  
  body_.Set(
    &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::move(value), GetArena());
  // @@protoc_insertion_point(field_set_rvalue:CreatePostRequest.body)
}
inline void CreatePostRequest::set_body(const char* value) {
  GOOGLE_DCHECK(value != nullptr);
  
  body_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(value),
              GetArena());
  // @@protoc_insertion_point(field_set_char:CreatePostRequest.body)
}
inline void CreatePostRequest::set_body(const char* value,
    size_t size) {
  
  body_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(
      reinterpret_cast<const char*>(value), size), GetArena());
  // @@protoc_insertion_point(field_set_pointer:CreatePostRequest.body)
}
inline std::string* CreatePostRequest::_internal_mutable_body() {
  
  return body_.Mutable(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline std::string* CreatePostRequest::release_body() {
  // @@protoc_insertion_point(field_release:CreatePostRequest.body)
  return body_.Release(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline void CreatePostRequest::set_allocated_body(std::string* body) {
  if (body != nullptr) {
    
  } else {
    
  }
  body_.SetAllocated(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), body,
      GetArena());
  // @@protoc_insertion_point(field_set_allocated:CreatePostRequest.body)
}

#ifdef __GNUC__
  #pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)


// @@protoc_insertion_point(global_scope)

#include <google/protobuf/port_undef.inc>
#endif  // GOOGLE_PROTOBUF_INCLUDED_GOOGLE_PROTOBUF_INCLUDED_forum_2eproto
