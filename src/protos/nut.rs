// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `nut.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Ok {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Ok {
    fn default() -> &'a Ok {
        <Ok as ::protobuf::Message>::default_instance()
    }
}

impl Ok {
    pub fn new() -> Ok {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Ok {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Ok {
        Ok::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Ok>(
                "Ok",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Ok {
        static instance: ::protobuf::rt::LazyV2<Ok> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Ok::new)
    }
}

impl ::protobuf::Clear for Ok {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ok {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ok {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignInRequest {
    // message fields
    pub email: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignInRequest {
    fn default() -> &'a SignInRequest {
        <SignInRequest as ::protobuf::Message>::default_instance()
    }
}

impl SignInRequest {
    pub fn new() -> SignInRequest {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string password = 2;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for SignInRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignInRequest {
        SignInRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &SignInRequest| { &m.email },
                |m: &mut SignInRequest| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &SignInRequest| { &m.password },
                |m: &mut SignInRequest| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignInRequest>(
                "SignInRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignInRequest {
        static instance: ::protobuf::rt::LazyV2<SignInRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignInRequest::new)
    }
}

impl ::protobuf::Clear for SignInRequest {
    fn clear(&mut self) {
        self.email.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignInRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignInRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignInResponse {
    // message fields
    pub token: ::std::string::String,
    pub roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignInResponse {
    fn default() -> &'a SignInResponse {
        <SignInResponse as ::protobuf::Message>::default_instance()
    }
}

impl SignInResponse {
    pub fn new() -> SignInResponse {
        ::std::default::Default::default()
    }

    // string token = 1;


    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.token, ::std::string::String::new())
    }

    // repeated string roles = 2;


    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }
    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for SignInResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.token.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.token);
        }
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.token.is_empty() {
            os.write_string(1, &self.token)?;
        }
        for v in &self.roles {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignInResponse {
        SignInResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "token",
                |m: &SignInResponse| { &m.token },
                |m: &mut SignInResponse| { &mut m.token },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "roles",
                |m: &SignInResponse| { &m.roles },
                |m: &mut SignInResponse| { &mut m.roles },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignInResponse>(
                "SignInResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignInResponse {
        static instance: ::protobuf::rt::LazyV2<SignInResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignInResponse::new)
    }
}

impl ::protobuf::Clear for SignInResponse {
    fn clear(&mut self) {
        self.token.clear();
        self.roles.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignInResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignInResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignUpRequest {
    // message fields
    pub real_name: ::std::string::String,
    pub nickname: ::std::string::String,
    pub email: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignUpRequest {
    fn default() -> &'a SignUpRequest {
        <SignUpRequest as ::protobuf::Message>::default_instance()
    }
}

impl SignUpRequest {
    pub fn new() -> SignUpRequest {
        ::std::default::Default::default()
    }

    // string real_name = 1;


    pub fn get_real_name(&self) -> &str {
        &self.real_name
    }
    pub fn clear_real_name(&mut self) {
        self.real_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_real_name(&mut self, v: ::std::string::String) {
        self.real_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_real_name(&mut self) -> &mut ::std::string::String {
        &mut self.real_name
    }

    // Take field
    pub fn take_real_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.real_name, ::std::string::String::new())
    }

    // string nickname = 2;


    pub fn get_nickname(&self) -> &str {
        &self.nickname
    }
    pub fn clear_nickname(&mut self) {
        self.nickname.clear();
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ::std::string::String) {
        self.nickname = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&mut self) -> &mut ::std::string::String {
        &mut self.nickname
    }

    // Take field
    pub fn take_nickname(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nickname, ::std::string::String::new())
    }

    // string email = 3;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string password = 4;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for SignUpRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.real_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nickname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.real_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.real_name);
        }
        if !self.nickname.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.nickname);
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.email);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.real_name.is_empty() {
            os.write_string(1, &self.real_name)?;
        }
        if !self.nickname.is_empty() {
            os.write_string(2, &self.nickname)?;
        }
        if !self.email.is_empty() {
            os.write_string(3, &self.email)?;
        }
        if !self.password.is_empty() {
            os.write_string(4, &self.password)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignUpRequest {
        SignUpRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "real_name",
                |m: &SignUpRequest| { &m.real_name },
                |m: &mut SignUpRequest| { &mut m.real_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "nickname",
                |m: &SignUpRequest| { &m.nickname },
                |m: &mut SignUpRequest| { &mut m.nickname },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &SignUpRequest| { &m.email },
                |m: &mut SignUpRequest| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &SignUpRequest| { &m.password },
                |m: &mut SignUpRequest| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignUpRequest>(
                "SignUpRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignUpRequest {
        static instance: ::protobuf::rt::LazyV2<SignUpRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignUpRequest::new)
    }
}

impl ::protobuf::Clear for SignUpRequest {
    fn clear(&mut self) {
        self.real_name.clear();
        self.nickname.clear();
        self.email.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignUpRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignUpRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tnut.proto\"\x04\n\x02Ok\"A\n\rSignInRequest\x12\x14\n\x05email\x18\
    \x01\x20\x01(\tR\x05email\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08p\
    assword\"<\n\x0eSignInResponse\x12\x14\n\x05token\x18\x01\x20\x01(\tR\
    \x05token\x12\x14\n\x05roles\x18\x02\x20\x03(\tR\x05roles\"z\n\rSignUpRe\
    quest\x12\x1b\n\treal_name\x18\x01\x20\x01(\tR\x08realName\x12\x1a\n\x08\
    nickname\x18\x02\x20\x01(\tR\x08nickname\x12\x14\n\x05email\x18\x03\x20\
    \x01(\tR\x05email\x12\x1a\n\x08password\x18\x04\x20\x01(\tR\x08password2\
    Z\n\nNutService\x12+\n\x06SignIn\x12\x0e.SignInRequest\x1a\x0f.SignInRes\
    ponse\"\0\x12\x1f\n\x06SignUp\x12\x0e.SignUpRequest\x1a\x03.Ok\"\0J\xbc\
    \x06\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x88\x01\
    \n\x02\x06\0\x12\x04\x06\0\t\x012|\x20import\x20\"google/protobuf/empty.\
    proto\";\n\x20import\x20\"google/protobuf/timestamp.proto\";\n\x20import\
    \x20\"google/protobuf/duration.proto\";\n\n\n\n\x03\x06\0\x01\x12\x03\
    \x06\x08\x12\n\x0b\n\x04\x06\0\x02\0\x12\x03\x07\x026\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03\x07\x06\x0c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x07\r\
    \x1a\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x07$2\n\x0b\n\x04\x06\0\x02\x01\
    \x12\x03\x08\x02*\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x08\x06\x0c\n\
    \x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x08\r\x1a\n\x0c\n\x05\x06\0\x02\x01\
    \x03\x12\x03\x08$&\n\t\n\x02\x04\0\x12\x03\x0b\0\r\n\n\n\x03\x04\0\x01\
    \x12\x03\x0b\x08\n\n\n\n\x02\x04\x01\x12\x04\r\0\x10\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\r\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0e\x02\x13\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x0e\t\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0e\
    \x11\x12\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0f\x02\x16\n\x0c\n\x05\x04\
    \x01\x02\x01\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03\x0f\t\x11\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0f\x14\x15\n\n\n\
    \x02\x04\x02\x12\x04\x12\0\x15\x01\n\n\n\x03\x04\x02\x01\x12\x03\x12\x08\
    \x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x13\x02\x13\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x13\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x13\
    \t\x0e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x13\x11\x12\n\x0b\n\x04\x04\
    \x02\x02\x01\x12\x03\x14\x02\x1c\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\
    \x14\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x14\x0b\x11\n\x0c\n\
    \x05\x04\x02\x02\x01\x01\x12\x03\x14\x12\x17\n\x0c\n\x05\x04\x02\x02\x01\
    \x03\x12\x03\x14\x1a\x1b\n\n\n\x02\x04\x03\x12\x04\x17\0\x1c\x01\n\n\n\
    \x03\x04\x03\x01\x12\x03\x17\x08\x15\n\x0b\n\x04\x04\x03\x02\0\x12\x03\
    \x18\x02\x17\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x18\x02\x08\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x03\x18\t\x12\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03\x18\x15\x16\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x19\x02\x16\n\
    \x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x19\x02\x08\n\x0c\n\x05\x04\x03\
    \x02\x01\x01\x12\x03\x19\t\x11\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\
    \x19\x14\x15\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x1a\x02\x13\n\x0c\n\x05\
    \x04\x03\x02\x02\x05\x12\x03\x1a\x02\x08\n\x0c\n\x05\x04\x03\x02\x02\x01\
    \x12\x03\x1a\t\x0e\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\x1a\x11\x12\n\
    \x0b\n\x04\x04\x03\x02\x03\x12\x03\x1b\x02\x16\n\x0c\n\x05\x04\x03\x02\
    \x03\x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03\x1b\
    \t\x11\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03\x1b\x14\x15b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
