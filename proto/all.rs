// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct BaseResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BaseResp {}

impl BaseResp {
    pub fn new() -> BaseResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BaseResp {
        static mut instance: ::protobuf::lazy::Lazy<BaseResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BaseResp,
        };
        unsafe {
            instance.get(BaseResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }
}

impl ::protobuf::Message for BaseResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BaseResp {
    fn new() -> BaseResp {
        BaseResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<BaseResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    BaseResp::get_code_for_reflect,
                    BaseResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    BaseResp::get_msg_for_reflect,
                    BaseResp::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BaseResp>(
                    "BaseResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BaseResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BaseResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BaseResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginAdminRequest {
    // message fields
    pub username: ::std::string::String,
    pub password: ::std::string::String,
    pub code: ::std::string::String,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginAdminRequest {}

impl LoginAdminRequest {
    pub fn new() -> LoginAdminRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginAdminRequest {
        static mut instance: ::protobuf::lazy::Lazy<LoginAdminRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginAdminRequest,
        };
        unsafe {
            instance.get(LoginAdminRequest::new)
        }
    }

    // string username = 1;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // string password = 2;

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

    pub fn get_password(&self) -> &str {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::string::String {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // string code = 3;

    pub fn clear_code(&mut self) {
        self.code.clear();
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::std::string::String) {
        self.code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code(&mut self) -> &mut ::std::string::String {
        &mut self.code
    }

    // Take field
    pub fn take_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.code, ::std::string::String::new())
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    fn get_code_for_reflect(&self) -> &::std::string::String {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.code
    }

    // string id = 4;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for LoginAdminRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.code)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
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
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.username);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        if !self.code.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.code);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.username.is_empty() {
            os.write_string(1, &self.username)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
        }
        if !self.code.is_empty() {
            os.write_string(3, &self.code)?;
        }
        if !self.id.is_empty() {
            os.write_string(4, &self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LoginAdminRequest {
    fn new() -> LoginAdminRequest {
        LoginAdminRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginAdminRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    LoginAdminRequest::get_username_for_reflect,
                    LoginAdminRequest::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "password",
                    LoginAdminRequest::get_password_for_reflect,
                    LoginAdminRequest::mut_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "code",
                    LoginAdminRequest::get_code_for_reflect,
                    LoginAdminRequest::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    LoginAdminRequest::get_id_for_reflect,
                    LoginAdminRequest::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginAdminRequest>(
                    "LoginAdminRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginAdminRequest {
    fn clear(&mut self) {
        self.clear_username();
        self.clear_password();
        self.clear_code();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginAdminRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginAdminRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginAdminResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginAdminResp {}

impl LoginAdminResp {
    pub fn new() -> LoginAdminResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginAdminResp {
        static mut instance: ::protobuf::lazy::Lazy<LoginAdminResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginAdminResp,
        };
        unsafe {
            instance.get(LoginAdminResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string token = 3;

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

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn get_token_for_reflect(&self) -> &::std::string::String {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }
}

impl ::protobuf::Message for LoginAdminResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.token.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.token);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.token.is_empty() {
            os.write_string(3, &self.token)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LoginAdminResp {
    fn new() -> LoginAdminResp {
        LoginAdminResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginAdminResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    LoginAdminResp::get_code_for_reflect,
                    LoginAdminResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    LoginAdminResp::get_msg_for_reflect,
                    LoginAdminResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    LoginAdminResp::get_token_for_reflect,
                    LoginAdminResp::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginAdminResp>(
                    "LoginAdminResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginAdminResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginAdminResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginAdminResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogoutAdminRequest {
    // message fields
    pub token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogoutAdminRequest {}

impl LogoutAdminRequest {
    pub fn new() -> LogoutAdminRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogoutAdminRequest {
        static mut instance: ::protobuf::lazy::Lazy<LogoutAdminRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogoutAdminRequest,
        };
        unsafe {
            instance.get(LogoutAdminRequest::new)
        }
    }

    // string token = 1;

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

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn get_token_for_reflect(&self) -> &::std::string::String {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }
}

impl ::protobuf::Message for LogoutAdminRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.token.is_empty() {
            os.write_string(1, &self.token)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogoutAdminRequest {
    fn new() -> LogoutAdminRequest {
        LogoutAdminRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogoutAdminRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    LogoutAdminRequest::get_token_for_reflect,
                    LogoutAdminRequest::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogoutAdminRequest>(
                    "LogoutAdminRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogoutAdminRequest {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogoutAdminRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogoutAdminRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminInfoResp {
    // message fields
    pub name: ::std::string::String,
    pub avatar: ::std::string::String,
    pub job: ::std::string::String,
    pub organization: ::std::string::String,
    pub location: ::std::string::String,
    pub email: ::std::string::String,
    pub introduction: ::std::string::String,
    pub personalWebsite: ::std::string::String,
    pub jobName: ::std::string::String,
    pub organizationName: ::std::string::String,
    pub locationName: ::std::string::String,
    pub phone: ::std::string::String,
    pub registrationDate: ::std::string::String,
    pub accountId: ::std::string::String,
    pub certification: ::std::string::String,
    pub role: ::std::string::String,
    pub code: u32,
    pub msg: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminInfoResp {}

impl AdminInfoResp {
    pub fn new() -> AdminInfoResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminInfoResp {
        static mut instance: ::protobuf::lazy::Lazy<AdminInfoResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminInfoResp,
        };
        unsafe {
            instance.get(AdminInfoResp::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string avatar = 2;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: ::std::string::String) {
        self.avatar = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // Take field
    pub fn take_avatar(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.avatar, ::std::string::String::new())
    }

    pub fn get_avatar(&self) -> &str {
        &self.avatar
    }

    fn get_avatar_for_reflect(&self) -> &::std::string::String {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // string job = 3;

    pub fn clear_job(&mut self) {
        self.job.clear();
    }

    // Param is passed by value, moved
    pub fn set_job(&mut self, v: ::std::string::String) {
        self.job = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_job(&mut self) -> &mut ::std::string::String {
        &mut self.job
    }

    // Take field
    pub fn take_job(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.job, ::std::string::String::new())
    }

    pub fn get_job(&self) -> &str {
        &self.job
    }

    fn get_job_for_reflect(&self) -> &::std::string::String {
        &self.job
    }

    fn mut_job_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.job
    }

    // string organization = 4;

    pub fn clear_organization(&mut self) {
        self.organization.clear();
    }

    // Param is passed by value, moved
    pub fn set_organization(&mut self, v: ::std::string::String) {
        self.organization = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_organization(&mut self) -> &mut ::std::string::String {
        &mut self.organization
    }

    // Take field
    pub fn take_organization(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.organization, ::std::string::String::new())
    }

    pub fn get_organization(&self) -> &str {
        &self.organization
    }

    fn get_organization_for_reflect(&self) -> &::std::string::String {
        &self.organization
    }

    fn mut_organization_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.organization
    }

    // string location = 5;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        &mut self.location
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.location, ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    fn get_location_for_reflect(&self) -> &::std::string::String {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.location
    }

    // string email = 6;

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

    pub fn get_email(&self) -> &str {
        &self.email
    }

    fn get_email_for_reflect(&self) -> &::std::string::String {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // string introduction = 7;

    pub fn clear_introduction(&mut self) {
        self.introduction.clear();
    }

    // Param is passed by value, moved
    pub fn set_introduction(&mut self, v: ::std::string::String) {
        self.introduction = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_introduction(&mut self) -> &mut ::std::string::String {
        &mut self.introduction
    }

    // Take field
    pub fn take_introduction(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.introduction, ::std::string::String::new())
    }

    pub fn get_introduction(&self) -> &str {
        &self.introduction
    }

    fn get_introduction_for_reflect(&self) -> &::std::string::String {
        &self.introduction
    }

    fn mut_introduction_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.introduction
    }

    // string personalWebsite = 8;

    pub fn clear_personalWebsite(&mut self) {
        self.personalWebsite.clear();
    }

    // Param is passed by value, moved
    pub fn set_personalWebsite(&mut self, v: ::std::string::String) {
        self.personalWebsite = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_personalWebsite(&mut self) -> &mut ::std::string::String {
        &mut self.personalWebsite
    }

    // Take field
    pub fn take_personalWebsite(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.personalWebsite, ::std::string::String::new())
    }

    pub fn get_personalWebsite(&self) -> &str {
        &self.personalWebsite
    }

    fn get_personalWebsite_for_reflect(&self) -> &::std::string::String {
        &self.personalWebsite
    }

    fn mut_personalWebsite_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.personalWebsite
    }

    // string jobName = 9;

    pub fn clear_jobName(&mut self) {
        self.jobName.clear();
    }

    // Param is passed by value, moved
    pub fn set_jobName(&mut self, v: ::std::string::String) {
        self.jobName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jobName(&mut self) -> &mut ::std::string::String {
        &mut self.jobName
    }

    // Take field
    pub fn take_jobName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.jobName, ::std::string::String::new())
    }

    pub fn get_jobName(&self) -> &str {
        &self.jobName
    }

    fn get_jobName_for_reflect(&self) -> &::std::string::String {
        &self.jobName
    }

    fn mut_jobName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.jobName
    }

    // string organizationName = 10;

    pub fn clear_organizationName(&mut self) {
        self.organizationName.clear();
    }

    // Param is passed by value, moved
    pub fn set_organizationName(&mut self, v: ::std::string::String) {
        self.organizationName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_organizationName(&mut self) -> &mut ::std::string::String {
        &mut self.organizationName
    }

    // Take field
    pub fn take_organizationName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.organizationName, ::std::string::String::new())
    }

    pub fn get_organizationName(&self) -> &str {
        &self.organizationName
    }

    fn get_organizationName_for_reflect(&self) -> &::std::string::String {
        &self.organizationName
    }

    fn mut_organizationName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.organizationName
    }

    // string locationName = 11;

    pub fn clear_locationName(&mut self) {
        self.locationName.clear();
    }

    // Param is passed by value, moved
    pub fn set_locationName(&mut self, v: ::std::string::String) {
        self.locationName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locationName(&mut self) -> &mut ::std::string::String {
        &mut self.locationName
    }

    // Take field
    pub fn take_locationName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.locationName, ::std::string::String::new())
    }

    pub fn get_locationName(&self) -> &str {
        &self.locationName
    }

    fn get_locationName_for_reflect(&self) -> &::std::string::String {
        &self.locationName
    }

    fn mut_locationName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.locationName
    }

    // string phone = 12;

    pub fn clear_phone(&mut self) {
        self.phone.clear();
    }

    // Param is passed by value, moved
    pub fn set_phone(&mut self, v: ::std::string::String) {
        self.phone = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_phone(&mut self) -> &mut ::std::string::String {
        &mut self.phone
    }

    // Take field
    pub fn take_phone(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.phone, ::std::string::String::new())
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    fn get_phone_for_reflect(&self) -> &::std::string::String {
        &self.phone
    }

    fn mut_phone_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.phone
    }

    // string registrationDate = 13;

    pub fn clear_registrationDate(&mut self) {
        self.registrationDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_registrationDate(&mut self, v: ::std::string::String) {
        self.registrationDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registrationDate(&mut self) -> &mut ::std::string::String {
        &mut self.registrationDate
    }

    // Take field
    pub fn take_registrationDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.registrationDate, ::std::string::String::new())
    }

    pub fn get_registrationDate(&self) -> &str {
        &self.registrationDate
    }

    fn get_registrationDate_for_reflect(&self) -> &::std::string::String {
        &self.registrationDate
    }

    fn mut_registrationDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.registrationDate
    }

    // string accountId = 14;

    pub fn clear_accountId(&mut self) {
        self.accountId.clear();
    }

    // Param is passed by value, moved
    pub fn set_accountId(&mut self, v: ::std::string::String) {
        self.accountId = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accountId(&mut self) -> &mut ::std::string::String {
        &mut self.accountId
    }

    // Take field
    pub fn take_accountId(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.accountId, ::std::string::String::new())
    }

    pub fn get_accountId(&self) -> &str {
        &self.accountId
    }

    fn get_accountId_for_reflect(&self) -> &::std::string::String {
        &self.accountId
    }

    fn mut_accountId_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.accountId
    }

    // string certification = 15;

    pub fn clear_certification(&mut self) {
        self.certification.clear();
    }

    // Param is passed by value, moved
    pub fn set_certification(&mut self, v: ::std::string::String) {
        self.certification = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_certification(&mut self) -> &mut ::std::string::String {
        &mut self.certification
    }

    // Take field
    pub fn take_certification(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.certification, ::std::string::String::new())
    }

    pub fn get_certification(&self) -> &str {
        &self.certification
    }

    fn get_certification_for_reflect(&self) -> &::std::string::String {
        &self.certification
    }

    fn mut_certification_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.certification
    }

    // string role = 16;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // uint32 code = 17;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 18;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }
}

impl ::protobuf::Message for AdminInfoResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.avatar)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.job)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.organization)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.location)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.introduction)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.personalWebsite)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.jobName)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.organizationName)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.locationName)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.phone)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.registrationDate)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.accountId)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.certification)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                18 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.avatar.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.avatar);
        }
        if !self.job.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.job);
        }
        if !self.organization.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.organization);
        }
        if !self.location.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.location);
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.email);
        }
        if !self.introduction.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.introduction);
        }
        if !self.personalWebsite.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.personalWebsite);
        }
        if !self.jobName.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.jobName);
        }
        if !self.organizationName.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.organizationName);
        }
        if !self.locationName.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.locationName);
        }
        if !self.phone.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.phone);
        }
        if !self.registrationDate.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.registrationDate);
        }
        if !self.accountId.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.accountId);
        }
        if !self.certification.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.certification);
        }
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.role);
        }
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(17, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(18, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.avatar.is_empty() {
            os.write_string(2, &self.avatar)?;
        }
        if !self.job.is_empty() {
            os.write_string(3, &self.job)?;
        }
        if !self.organization.is_empty() {
            os.write_string(4, &self.organization)?;
        }
        if !self.location.is_empty() {
            os.write_string(5, &self.location)?;
        }
        if !self.email.is_empty() {
            os.write_string(6, &self.email)?;
        }
        if !self.introduction.is_empty() {
            os.write_string(7, &self.introduction)?;
        }
        if !self.personalWebsite.is_empty() {
            os.write_string(8, &self.personalWebsite)?;
        }
        if !self.jobName.is_empty() {
            os.write_string(9, &self.jobName)?;
        }
        if !self.organizationName.is_empty() {
            os.write_string(10, &self.organizationName)?;
        }
        if !self.locationName.is_empty() {
            os.write_string(11, &self.locationName)?;
        }
        if !self.phone.is_empty() {
            os.write_string(12, &self.phone)?;
        }
        if !self.registrationDate.is_empty() {
            os.write_string(13, &self.registrationDate)?;
        }
        if !self.accountId.is_empty() {
            os.write_string(14, &self.accountId)?;
        }
        if !self.certification.is_empty() {
            os.write_string(15, &self.certification)?;
        }
        if !self.role.is_empty() {
            os.write_string(16, &self.role)?;
        }
        if self.code != 0 {
            os.write_uint32(17, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(18, &self.msg)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminInfoResp {
    fn new() -> AdminInfoResp {
        AdminInfoResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminInfoResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AdminInfoResp::get_name_for_reflect,
                    AdminInfoResp::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "avatar",
                    AdminInfoResp::get_avatar_for_reflect,
                    AdminInfoResp::mut_avatar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "job",
                    AdminInfoResp::get_job_for_reflect,
                    AdminInfoResp::mut_job_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "organization",
                    AdminInfoResp::get_organization_for_reflect,
                    AdminInfoResp::mut_organization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "location",
                    AdminInfoResp::get_location_for_reflect,
                    AdminInfoResp::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    AdminInfoResp::get_email_for_reflect,
                    AdminInfoResp::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "introduction",
                    AdminInfoResp::get_introduction_for_reflect,
                    AdminInfoResp::mut_introduction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "personalWebsite",
                    AdminInfoResp::get_personalWebsite_for_reflect,
                    AdminInfoResp::mut_personalWebsite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "jobName",
                    AdminInfoResp::get_jobName_for_reflect,
                    AdminInfoResp::mut_jobName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "organizationName",
                    AdminInfoResp::get_organizationName_for_reflect,
                    AdminInfoResp::mut_organizationName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "locationName",
                    AdminInfoResp::get_locationName_for_reflect,
                    AdminInfoResp::mut_locationName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "phone",
                    AdminInfoResp::get_phone_for_reflect,
                    AdminInfoResp::mut_phone_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "registrationDate",
                    AdminInfoResp::get_registrationDate_for_reflect,
                    AdminInfoResp::mut_registrationDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "accountId",
                    AdminInfoResp::get_accountId_for_reflect,
                    AdminInfoResp::mut_accountId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "certification",
                    AdminInfoResp::get_certification_for_reflect,
                    AdminInfoResp::mut_certification_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    AdminInfoResp::get_role_for_reflect,
                    AdminInfoResp::mut_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    AdminInfoResp::get_code_for_reflect,
                    AdminInfoResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    AdminInfoResp::get_msg_for_reflect,
                    AdminInfoResp::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminInfoResp>(
                    "AdminInfoResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminInfoResp {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_avatar();
        self.clear_job();
        self.clear_organization();
        self.clear_location();
        self.clear_email();
        self.clear_introduction();
        self.clear_personalWebsite();
        self.clear_jobName();
        self.clear_organizationName();
        self.clear_locationName();
        self.clear_phone();
        self.clear_registrationDate();
        self.clear_accountId();
        self.clear_certification();
        self.clear_role();
        self.clear_code();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminInfoResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminInfoResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Comment {
    // message fields
    pub _id: ::std::string::String,
    pub avatar: ::std::string::String,
    pub username: ::std::string::String,
    pub label: ::std::string::String,
    pub createDate: ::std::string::String,
    pub content: ::std::string::String,
    pub children: ::protobuf::RepeatedField<Comment>,
    pub parentUsername: ::std::string::String,
    pub ip: ::std::string::String,
    pub ua: ::std::string::String,
    pub location: ::std::string::String,
    pub os: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Comment {}

impl Comment {
    pub fn new() -> Comment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Comment {
        static mut instance: ::protobuf::lazy::Lazy<Comment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Comment,
        };
        unsafe {
            instance.get(Comment::new)
        }
    }

    // string _id = 1;

    pub fn clear__id(&mut self) {
        self._id.clear();
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: ::std::string::String) {
        self._id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut__id(&mut self) -> &mut ::std::string::String {
        &mut self._id
    }

    // Take field
    pub fn take__id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self._id, ::std::string::String::new())
    }

    pub fn get__id(&self) -> &str {
        &self._id
    }

    fn get__id_for_reflect(&self) -> &::std::string::String {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self._id
    }

    // string avatar = 2;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: ::std::string::String) {
        self.avatar = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // Take field
    pub fn take_avatar(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.avatar, ::std::string::String::new())
    }

    pub fn get_avatar(&self) -> &str {
        &self.avatar
    }

    fn get_avatar_for_reflect(&self) -> &::std::string::String {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // string username = 3;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // string label = 4;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.label, ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    fn get_label_for_reflect(&self) -> &::std::string::String {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // string createDate = 5;

    pub fn clear_createDate(&mut self) {
        self.createDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_createDate(&mut self, v: ::std::string::String) {
        self.createDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createDate(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // Take field
    pub fn take_createDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createDate, ::std::string::String::new())
    }

    pub fn get_createDate(&self) -> &str {
        &self.createDate
    }

    fn get_createDate_for_reflect(&self) -> &::std::string::String {
        &self.createDate
    }

    fn mut_createDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // string content = 6;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // repeated .proto.Comment children = 7;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Comment>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Comment> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Comment> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Comment] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<Comment> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Comment> {
        &mut self.children
    }

    // string parentUsername = 8;

    pub fn clear_parentUsername(&mut self) {
        self.parentUsername.clear();
    }

    // Param is passed by value, moved
    pub fn set_parentUsername(&mut self, v: ::std::string::String) {
        self.parentUsername = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parentUsername(&mut self) -> &mut ::std::string::String {
        &mut self.parentUsername
    }

    // Take field
    pub fn take_parentUsername(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.parentUsername, ::std::string::String::new())
    }

    pub fn get_parentUsername(&self) -> &str {
        &self.parentUsername
    }

    fn get_parentUsername_for_reflect(&self) -> &::std::string::String {
        &self.parentUsername
    }

    fn mut_parentUsername_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.parentUsername
    }

    // string ip = 9;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip, ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_ip_for_reflect(&self) -> &::std::string::String {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // string ua = 10;

    pub fn clear_ua(&mut self) {
        self.ua.clear();
    }

    // Param is passed by value, moved
    pub fn set_ua(&mut self, v: ::std::string::String) {
        self.ua = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ua(&mut self) -> &mut ::std::string::String {
        &mut self.ua
    }

    // Take field
    pub fn take_ua(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ua, ::std::string::String::new())
    }

    pub fn get_ua(&self) -> &str {
        &self.ua
    }

    fn get_ua_for_reflect(&self) -> &::std::string::String {
        &self.ua
    }

    fn mut_ua_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ua
    }

    // string location = 11;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        &mut self.location
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.location, ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    fn get_location_for_reflect(&self) -> &::std::string::String {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.location
    }

    // string os = 12;

    pub fn clear_os(&mut self) {
        self.os.clear();
    }

    // Param is passed by value, moved
    pub fn set_os(&mut self, v: ::std::string::String) {
        self.os = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_os(&mut self) -> &mut ::std::string::String {
        &mut self.os
    }

    // Take field
    pub fn take_os(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.os, ::std::string::String::new())
    }

    pub fn get_os(&self) -> &str {
        &self.os
    }

    fn get_os_for_reflect(&self) -> &::std::string::String {
        &self.os
    }

    fn mut_os_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.os
    }
}

impl ::protobuf::Message for Comment {
    fn is_initialized(&self) -> bool {
        for v in &self.children {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self._id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.avatar)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.label)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createDate)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.parentUsername)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ua)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.location)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.os)?;
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
        if !self._id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self._id);
        }
        if !self.avatar.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.avatar);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.username);
        }
        if !self.label.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.label);
        }
        if !self.createDate.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.createDate);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.content);
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.parentUsername.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.parentUsername);
        }
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.ip);
        }
        if !self.ua.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.ua);
        }
        if !self.location.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.location);
        }
        if !self.os.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.os);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self._id.is_empty() {
            os.write_string(1, &self._id)?;
        }
        if !self.avatar.is_empty() {
            os.write_string(2, &self.avatar)?;
        }
        if !self.username.is_empty() {
            os.write_string(3, &self.username)?;
        }
        if !self.label.is_empty() {
            os.write_string(4, &self.label)?;
        }
        if !self.createDate.is_empty() {
            os.write_string(5, &self.createDate)?;
        }
        if !self.content.is_empty() {
            os.write_string(6, &self.content)?;
        }
        for v in &self.children {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.parentUsername.is_empty() {
            os.write_string(8, &self.parentUsername)?;
        }
        if !self.ip.is_empty() {
            os.write_string(9, &self.ip)?;
        }
        if !self.ua.is_empty() {
            os.write_string(10, &self.ua)?;
        }
        if !self.location.is_empty() {
            os.write_string(11, &self.location)?;
        }
        if !self.os.is_empty() {
            os.write_string(12, &self.os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Comment {
    fn new() -> Comment {
        Comment::new()
    }

    fn descriptor_static(_: ::std::option::Option<Comment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "_id",
                    Comment::get__id_for_reflect,
                    Comment::mut__id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "avatar",
                    Comment::get_avatar_for_reflect,
                    Comment::mut_avatar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    Comment::get_username_for_reflect,
                    Comment::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    Comment::get_label_for_reflect,
                    Comment::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createDate",
                    Comment::get_createDate_for_reflect,
                    Comment::mut_createDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    Comment::get_content_for_reflect,
                    Comment::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Comment>>(
                    "children",
                    Comment::get_children_for_reflect,
                    Comment::mut_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "parentUsername",
                    Comment::get_parentUsername_for_reflect,
                    Comment::mut_parentUsername_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    Comment::get_ip_for_reflect,
                    Comment::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ua",
                    Comment::get_ua_for_reflect,
                    Comment::mut_ua_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "location",
                    Comment::get_location_for_reflect,
                    Comment::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "os",
                    Comment::get_os_for_reflect,
                    Comment::mut_os_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Comment>(
                    "Comment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Comment {
    fn clear(&mut self) {
        self.clear__id();
        self.clear_avatar();
        self.clear_username();
        self.clear_label();
        self.clear_createDate();
        self.clear_content();
        self.clear_children();
        self.clear_parentUsername();
        self.clear_ip();
        self.clear_ua();
        self.clear_location();
        self.clear_os();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Comment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Comment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Pagination {
    // message fields
    pub countTotal: u32,
    pub totalPage: u32,
    pub currentPage: u32,
    pub pageSize: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Pagination {}

impl Pagination {
    pub fn new() -> Pagination {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Pagination {
        static mut instance: ::protobuf::lazy::Lazy<Pagination> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Pagination,
        };
        unsafe {
            instance.get(Pagination::new)
        }
    }

    // uint32 countTotal = 1;

    pub fn clear_countTotal(&mut self) {
        self.countTotal = 0;
    }

    // Param is passed by value, moved
    pub fn set_countTotal(&mut self, v: u32) {
        self.countTotal = v;
    }

    pub fn get_countTotal(&self) -> u32 {
        self.countTotal
    }

    fn get_countTotal_for_reflect(&self) -> &u32 {
        &self.countTotal
    }

    fn mut_countTotal_for_reflect(&mut self) -> &mut u32 {
        &mut self.countTotal
    }

    // uint32 totalPage = 2;

    pub fn clear_totalPage(&mut self) {
        self.totalPage = 0;
    }

    // Param is passed by value, moved
    pub fn set_totalPage(&mut self, v: u32) {
        self.totalPage = v;
    }

    pub fn get_totalPage(&self) -> u32 {
        self.totalPage
    }

    fn get_totalPage_for_reflect(&self) -> &u32 {
        &self.totalPage
    }

    fn mut_totalPage_for_reflect(&mut self) -> &mut u32 {
        &mut self.totalPage
    }

    // uint32 currentPage = 3;

    pub fn clear_currentPage(&mut self) {
        self.currentPage = 0;
    }

    // Param is passed by value, moved
    pub fn set_currentPage(&mut self, v: u32) {
        self.currentPage = v;
    }

    pub fn get_currentPage(&self) -> u32 {
        self.currentPage
    }

    fn get_currentPage_for_reflect(&self) -> &u32 {
        &self.currentPage
    }

    fn mut_currentPage_for_reflect(&mut self) -> &mut u32 {
        &mut self.currentPage
    }

    // uint32 pageSize = 4;

    pub fn clear_pageSize(&mut self) {
        self.pageSize = 0;
    }

    // Param is passed by value, moved
    pub fn set_pageSize(&mut self, v: u32) {
        self.pageSize = v;
    }

    pub fn get_pageSize(&self) -> u32 {
        self.pageSize
    }

    fn get_pageSize_for_reflect(&self) -> &u32 {
        &self.pageSize
    }

    fn mut_pageSize_for_reflect(&mut self) -> &mut u32 {
        &mut self.pageSize
    }
}

impl ::protobuf::Message for Pagination {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.countTotal = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.totalPage = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.currentPage = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.pageSize = tmp;
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
        if self.countTotal != 0 {
            my_size += ::protobuf::rt::value_size(1, self.countTotal, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.totalPage != 0 {
            my_size += ::protobuf::rt::value_size(2, self.totalPage, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.currentPage != 0 {
            my_size += ::protobuf::rt::value_size(3, self.currentPage, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.pageSize != 0 {
            my_size += ::protobuf::rt::value_size(4, self.pageSize, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.countTotal != 0 {
            os.write_uint32(1, self.countTotal)?;
        }
        if self.totalPage != 0 {
            os.write_uint32(2, self.totalPage)?;
        }
        if self.currentPage != 0 {
            os.write_uint32(3, self.currentPage)?;
        }
        if self.pageSize != 0 {
            os.write_uint32(4, self.pageSize)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Pagination {
    fn new() -> Pagination {
        Pagination::new()
    }

    fn descriptor_static(_: ::std::option::Option<Pagination>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "countTotal",
                    Pagination::get_countTotal_for_reflect,
                    Pagination::mut_countTotal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "totalPage",
                    Pagination::get_totalPage_for_reflect,
                    Pagination::mut_totalPage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "currentPage",
                    Pagination::get_currentPage_for_reflect,
                    Pagination::mut_currentPage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pageSize",
                    Pagination::get_pageSize_for_reflect,
                    Pagination::mut_pageSize_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Pagination>(
                    "Pagination",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Pagination {
    fn clear(&mut self) {
        self.clear_countTotal();
        self.clear_totalPage();
        self.clear_currentPage();
        self.clear_pageSize();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Pagination {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Pagination {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommentListResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub list: ::protobuf::RepeatedField<Comment>,
    pub pagination: ::protobuf::SingularPtrField<Pagination>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommentListResp {}

impl CommentListResp {
    pub fn new() -> CommentListResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommentListResp {
        static mut instance: ::protobuf::lazy::Lazy<CommentListResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommentListResp,
        };
        unsafe {
            instance.get(CommentListResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated .proto.Comment list = 4;

    pub fn clear_list(&mut self) {
        self.list.clear();
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: ::protobuf::RepeatedField<Comment>) {
        self.list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_list(&mut self) -> &mut ::protobuf::RepeatedField<Comment> {
        &mut self.list
    }

    // Take field
    pub fn take_list(&mut self) -> ::protobuf::RepeatedField<Comment> {
        ::std::mem::replace(&mut self.list, ::protobuf::RepeatedField::new())
    }

    pub fn get_list(&self) -> &[Comment] {
        &self.list
    }

    fn get_list_for_reflect(&self) -> &::protobuf::RepeatedField<Comment> {
        &self.list
    }

    fn mut_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Comment> {
        &mut self.list
    }

    // .proto.Pagination pagination = 5;

    pub fn clear_pagination(&mut self) {
        self.pagination.clear();
    }

    pub fn has_pagination(&self) -> bool {
        self.pagination.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pagination(&mut self, v: Pagination) {
        self.pagination = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pagination(&mut self) -> &mut Pagination {
        if self.pagination.is_none() {
            self.pagination.set_default();
        }
        self.pagination.as_mut().unwrap()
    }

    // Take field
    pub fn take_pagination(&mut self) -> Pagination {
        self.pagination.take().unwrap_or_else(|| Pagination::new())
    }

    pub fn get_pagination(&self) -> &Pagination {
        self.pagination.as_ref().unwrap_or_else(|| Pagination::default_instance())
    }

    fn get_pagination_for_reflect(&self) -> &::protobuf::SingularPtrField<Pagination> {
        &self.pagination
    }

    fn mut_pagination_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Pagination> {
        &mut self.pagination
    }
}

impl ::protobuf::Message for CommentListResp {
    fn is_initialized(&self) -> bool {
        for v in &self.list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pagination {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.list)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pagination)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.pagination.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.list {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.pagination.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CommentListResp {
    fn new() -> CommentListResp {
        CommentListResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommentListResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    CommentListResp::get_code_for_reflect,
                    CommentListResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    CommentListResp::get_msg_for_reflect,
                    CommentListResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Comment>>(
                    "list",
                    CommentListResp::get_list_for_reflect,
                    CommentListResp::mut_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Pagination>>(
                    "pagination",
                    CommentListResp::get_pagination_for_reflect,
                    CommentListResp::mut_pagination_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommentListResp>(
                    "CommentListResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommentListResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_list();
        self.clear_pagination();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommentListResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommentListResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommentAddRequest {
    // message fields
    pub content: ::std::string::String,
    pub articleId: ::std::string::String,
    pub parentId: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommentAddRequest {}

impl CommentAddRequest {
    pub fn new() -> CommentAddRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommentAddRequest {
        static mut instance: ::protobuf::lazy::Lazy<CommentAddRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommentAddRequest,
        };
        unsafe {
            instance.get(CommentAddRequest::new)
        }
    }

    // string content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // string articleId = 2;

    pub fn clear_articleId(&mut self) {
        self.articleId.clear();
    }

    // Param is passed by value, moved
    pub fn set_articleId(&mut self, v: ::std::string::String) {
        self.articleId = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_articleId(&mut self) -> &mut ::std::string::String {
        &mut self.articleId
    }

    // Take field
    pub fn take_articleId(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.articleId, ::std::string::String::new())
    }

    pub fn get_articleId(&self) -> &str {
        &self.articleId
    }

    fn get_articleId_for_reflect(&self) -> &::std::string::String {
        &self.articleId
    }

    fn mut_articleId_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.articleId
    }

    // string parentId = 3;

    pub fn clear_parentId(&mut self) {
        self.parentId.clear();
    }

    // Param is passed by value, moved
    pub fn set_parentId(&mut self, v: ::std::string::String) {
        self.parentId = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parentId(&mut self) -> &mut ::std::string::String {
        &mut self.parentId
    }

    // Take field
    pub fn take_parentId(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.parentId, ::std::string::String::new())
    }

    pub fn get_parentId(&self) -> &str {
        &self.parentId
    }

    fn get_parentId_for_reflect(&self) -> &::std::string::String {
        &self.parentId
    }

    fn mut_parentId_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.parentId
    }
}

impl ::protobuf::Message for CommentAddRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.articleId)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.parentId)?;
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
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.content);
        }
        if !self.articleId.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.articleId);
        }
        if !self.parentId.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.parentId);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.content.is_empty() {
            os.write_string(1, &self.content)?;
        }
        if !self.articleId.is_empty() {
            os.write_string(2, &self.articleId)?;
        }
        if !self.parentId.is_empty() {
            os.write_string(3, &self.parentId)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CommentAddRequest {
    fn new() -> CommentAddRequest {
        CommentAddRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommentAddRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    CommentAddRequest::get_content_for_reflect,
                    CommentAddRequest::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "articleId",
                    CommentAddRequest::get_articleId_for_reflect,
                    CommentAddRequest::mut_articleId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "parentId",
                    CommentAddRequest::get_parentId_for_reflect,
                    CommentAddRequest::mut_parentId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommentAddRequest>(
                    "CommentAddRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommentAddRequest {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_articleId();
        self.clear_parentId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommentAddRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommentAddRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommentAddResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub data: ::protobuf::SingularPtrField<Comment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommentAddResp {}

impl CommentAddResp {
    pub fn new() -> CommentAddResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommentAddResp {
        static mut instance: ::protobuf::lazy::Lazy<CommentAddResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommentAddResp,
        };
        unsafe {
            instance.get(CommentAddResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // .proto.Comment data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: Comment) {
        self.data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut Comment {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> Comment {
        self.data.take().unwrap_or_else(|| Comment::new())
    }

    pub fn get_data(&self) -> &Comment {
        self.data.as_ref().unwrap_or_else(|| Comment::default_instance())
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularPtrField<Comment> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Comment> {
        &mut self.data
    }
}

impl ::protobuf::Message for CommentAddResp {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.data)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if let Some(ref v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CommentAddResp {
    fn new() -> CommentAddResp {
        CommentAddResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommentAddResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    CommentAddResp::get_code_for_reflect,
                    CommentAddResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    CommentAddResp::get_msg_for_reflect,
                    CommentAddResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Comment>>(
                    "data",
                    CommentAddResp::get_data_for_reflect,
                    CommentAddResp::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommentAddResp>(
                    "CommentAddResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommentAddResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommentAddResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommentAddResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CatchMe {
    // message fields
    pub git: ::std::string::String,
    pub job: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CatchMe {}

impl CatchMe {
    pub fn new() -> CatchMe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CatchMe {
        static mut instance: ::protobuf::lazy::Lazy<CatchMe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CatchMe,
        };
        unsafe {
            instance.get(CatchMe::new)
        }
    }

    // string git = 1;

    pub fn clear_git(&mut self) {
        self.git.clear();
    }

    // Param is passed by value, moved
    pub fn set_git(&mut self, v: ::std::string::String) {
        self.git = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_git(&mut self) -> &mut ::std::string::String {
        &mut self.git
    }

    // Take field
    pub fn take_git(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.git, ::std::string::String::new())
    }

    pub fn get_git(&self) -> &str {
        &self.git
    }

    fn get_git_for_reflect(&self) -> &::std::string::String {
        &self.git
    }

    fn mut_git_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.git
    }

    // string job = 2;

    pub fn clear_job(&mut self) {
        self.job.clear();
    }

    // Param is passed by value, moved
    pub fn set_job(&mut self, v: ::std::string::String) {
        self.job = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_job(&mut self) -> &mut ::std::string::String {
        &mut self.job
    }

    // Take field
    pub fn take_job(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.job, ::std::string::String::new())
    }

    pub fn get_job(&self) -> &str {
        &self.job
    }

    fn get_job_for_reflect(&self) -> &::std::string::String {
        &self.job
    }

    fn mut_job_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.job
    }
}

impl ::protobuf::Message for CatchMe {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.git)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.job)?;
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
        if !self.git.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.git);
        }
        if !self.job.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.job);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.git.is_empty() {
            os.write_string(1, &self.git)?;
        }
        if !self.job.is_empty() {
            os.write_string(2, &self.job)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CatchMe {
    fn new() -> CatchMe {
        CatchMe::new()
    }

    fn descriptor_static(_: ::std::option::Option<CatchMe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "git",
                    CatchMe::get_git_for_reflect,
                    CatchMe::mut_git_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "job",
                    CatchMe::get_job_for_reflect,
                    CatchMe::mut_job_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CatchMe>(
                    "CatchMe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CatchMe {
    fn clear(&mut self) {
        self.clear_git();
        self.clear_job();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CatchMe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CatchMe {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AboutResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub likeNum: ::std::string::String,
    pub catchMe: ::protobuf::SingularPtrField<CatchMe>,
    pub descriptions: ::std::string::String,
    pub id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AboutResp {}

impl AboutResp {
    pub fn new() -> AboutResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AboutResp {
        static mut instance: ::protobuf::lazy::Lazy<AboutResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AboutResp,
        };
        unsafe {
            instance.get(AboutResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string likeNum = 3;

    pub fn clear_likeNum(&mut self) {
        self.likeNum.clear();
    }

    // Param is passed by value, moved
    pub fn set_likeNum(&mut self, v: ::std::string::String) {
        self.likeNum = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_likeNum(&mut self) -> &mut ::std::string::String {
        &mut self.likeNum
    }

    // Take field
    pub fn take_likeNum(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.likeNum, ::std::string::String::new())
    }

    pub fn get_likeNum(&self) -> &str {
        &self.likeNum
    }

    fn get_likeNum_for_reflect(&self) -> &::std::string::String {
        &self.likeNum
    }

    fn mut_likeNum_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.likeNum
    }

    // .proto.CatchMe catchMe = 4;

    pub fn clear_catchMe(&mut self) {
        self.catchMe.clear();
    }

    pub fn has_catchMe(&self) -> bool {
        self.catchMe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_catchMe(&mut self, v: CatchMe) {
        self.catchMe = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_catchMe(&mut self) -> &mut CatchMe {
        if self.catchMe.is_none() {
            self.catchMe.set_default();
        }
        self.catchMe.as_mut().unwrap()
    }

    // Take field
    pub fn take_catchMe(&mut self) -> CatchMe {
        self.catchMe.take().unwrap_or_else(|| CatchMe::new())
    }

    pub fn get_catchMe(&self) -> &CatchMe {
        self.catchMe.as_ref().unwrap_or_else(|| CatchMe::default_instance())
    }

    fn get_catchMe_for_reflect(&self) -> &::protobuf::SingularPtrField<CatchMe> {
        &self.catchMe
    }

    fn mut_catchMe_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CatchMe> {
        &mut self.catchMe
    }

    // string descriptions = 5;

    pub fn clear_descriptions(&mut self) {
        self.descriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptions(&mut self, v: ::std::string::String) {
        self.descriptions = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descriptions(&mut self) -> &mut ::std::string::String {
        &mut self.descriptions
    }

    // Take field
    pub fn take_descriptions(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descriptions, ::std::string::String::new())
    }

    pub fn get_descriptions(&self) -> &str {
        &self.descriptions
    }

    fn get_descriptions_for_reflect(&self) -> &::std::string::String {
        &self.descriptions
    }

    fn mut_descriptions_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.descriptions
    }

    // uint32 id = 6;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }
}

impl ::protobuf::Message for AboutResp {
    fn is_initialized(&self) -> bool {
        for v in &self.catchMe {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.likeNum)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.catchMe)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descriptions)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.likeNum.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.likeNum);
        }
        if let Some(ref v) = self.catchMe.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.descriptions.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.descriptions);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(6, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.likeNum.is_empty() {
            os.write_string(3, &self.likeNum)?;
        }
        if let Some(ref v) = self.catchMe.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.descriptions.is_empty() {
            os.write_string(5, &self.descriptions)?;
        }
        if self.id != 0 {
            os.write_uint32(6, self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AboutResp {
    fn new() -> AboutResp {
        AboutResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AboutResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    AboutResp::get_code_for_reflect,
                    AboutResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    AboutResp::get_msg_for_reflect,
                    AboutResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "likeNum",
                    AboutResp::get_likeNum_for_reflect,
                    AboutResp::mut_likeNum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CatchMe>>(
                    "catchMe",
                    AboutResp::get_catchMe_for_reflect,
                    AboutResp::mut_catchMe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "descriptions",
                    AboutResp::get_descriptions_for_reflect,
                    AboutResp::mut_descriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    AboutResp::get_id_for_reflect,
                    AboutResp::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AboutResp>(
                    "AboutResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AboutResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_likeNum();
        self.clear_catchMe();
        self.clear_descriptions();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AboutResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AboutResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SiteInfoReq {
    // message fields
    pub copyright: ::std::string::String,
    pub descriptions: ::std::string::String,
    pub beian: ::std::string::String,
    pub title: ::std::string::String,
    pub id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SiteInfoReq {}

impl SiteInfoReq {
    pub fn new() -> SiteInfoReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SiteInfoReq {
        static mut instance: ::protobuf::lazy::Lazy<SiteInfoReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SiteInfoReq,
        };
        unsafe {
            instance.get(SiteInfoReq::new)
        }
    }

    // string copyright = 1;

    pub fn clear_copyright(&mut self) {
        self.copyright.clear();
    }

    // Param is passed by value, moved
    pub fn set_copyright(&mut self, v: ::std::string::String) {
        self.copyright = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_copyright(&mut self) -> &mut ::std::string::String {
        &mut self.copyright
    }

    // Take field
    pub fn take_copyright(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.copyright, ::std::string::String::new())
    }

    pub fn get_copyright(&self) -> &str {
        &self.copyright
    }

    fn get_copyright_for_reflect(&self) -> &::std::string::String {
        &self.copyright
    }

    fn mut_copyright_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.copyright
    }

    // string descriptions = 4;

    pub fn clear_descriptions(&mut self) {
        self.descriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptions(&mut self, v: ::std::string::String) {
        self.descriptions = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descriptions(&mut self) -> &mut ::std::string::String {
        &mut self.descriptions
    }

    // Take field
    pub fn take_descriptions(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descriptions, ::std::string::String::new())
    }

    pub fn get_descriptions(&self) -> &str {
        &self.descriptions
    }

    fn get_descriptions_for_reflect(&self) -> &::std::string::String {
        &self.descriptions
    }

    fn mut_descriptions_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.descriptions
    }

    // string beian = 5;

    pub fn clear_beian(&mut self) {
        self.beian.clear();
    }

    // Param is passed by value, moved
    pub fn set_beian(&mut self, v: ::std::string::String) {
        self.beian = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_beian(&mut self) -> &mut ::std::string::String {
        &mut self.beian
    }

    // Take field
    pub fn take_beian(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.beian, ::std::string::String::new())
    }

    pub fn get_beian(&self) -> &str {
        &self.beian
    }

    fn get_beian_for_reflect(&self) -> &::std::string::String {
        &self.beian
    }

    fn mut_beian_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.beian
    }

    // string title = 6;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // uint32 id = 7;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }
}

impl ::protobuf::Message for SiteInfoReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.copyright)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descriptions)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.beian)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
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
        if !self.copyright.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.copyright);
        }
        if !self.descriptions.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.descriptions);
        }
        if !self.beian.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.beian);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.title);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(7, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.copyright.is_empty() {
            os.write_string(1, &self.copyright)?;
        }
        if !self.descriptions.is_empty() {
            os.write_string(4, &self.descriptions)?;
        }
        if !self.beian.is_empty() {
            os.write_string(5, &self.beian)?;
        }
        if !self.title.is_empty() {
            os.write_string(6, &self.title)?;
        }
        if self.id != 0 {
            os.write_uint32(7, self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SiteInfoReq {
    fn new() -> SiteInfoReq {
        SiteInfoReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<SiteInfoReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "copyright",
                    SiteInfoReq::get_copyright_for_reflect,
                    SiteInfoReq::mut_copyright_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "descriptions",
                    SiteInfoReq::get_descriptions_for_reflect,
                    SiteInfoReq::mut_descriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "beian",
                    SiteInfoReq::get_beian_for_reflect,
                    SiteInfoReq::mut_beian_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    SiteInfoReq::get_title_for_reflect,
                    SiteInfoReq::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    SiteInfoReq::get_id_for_reflect,
                    SiteInfoReq::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SiteInfoReq>(
                    "SiteInfoReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SiteInfoReq {
    fn clear(&mut self) {
        self.clear_copyright();
        self.clear_descriptions();
        self.clear_beian();
        self.clear_title();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SiteInfoReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SiteInfoReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SiteInfoResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub author: ::std::string::String,
    pub github: ::std::string::String,
    pub beian: ::std::string::String,
    pub descriptions: ::std::string::String,
    pub selfDescriptions: ::std::string::String,
    pub id: u32,
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SiteInfoResp {}

impl SiteInfoResp {
    pub fn new() -> SiteInfoResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SiteInfoResp {
        static mut instance: ::protobuf::lazy::Lazy<SiteInfoResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SiteInfoResp,
        };
        unsafe {
            instance.get(SiteInfoResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string author = 3;

    pub fn clear_author(&mut self) {
        self.author.clear();
    }

    // Param is passed by value, moved
    pub fn set_author(&mut self, v: ::std::string::String) {
        self.author = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_author(&mut self) -> &mut ::std::string::String {
        &mut self.author
    }

    // Take field
    pub fn take_author(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.author, ::std::string::String::new())
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    fn get_author_for_reflect(&self) -> &::std::string::String {
        &self.author
    }

    fn mut_author_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.author
    }

    // string github = 4;

    pub fn clear_github(&mut self) {
        self.github.clear();
    }

    // Param is passed by value, moved
    pub fn set_github(&mut self, v: ::std::string::String) {
        self.github = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_github(&mut self) -> &mut ::std::string::String {
        &mut self.github
    }

    // Take field
    pub fn take_github(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.github, ::std::string::String::new())
    }

    pub fn get_github(&self) -> &str {
        &self.github
    }

    fn get_github_for_reflect(&self) -> &::std::string::String {
        &self.github
    }

    fn mut_github_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.github
    }

    // string beian = 5;

    pub fn clear_beian(&mut self) {
        self.beian.clear();
    }

    // Param is passed by value, moved
    pub fn set_beian(&mut self, v: ::std::string::String) {
        self.beian = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_beian(&mut self) -> &mut ::std::string::String {
        &mut self.beian
    }

    // Take field
    pub fn take_beian(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.beian, ::std::string::String::new())
    }

    pub fn get_beian(&self) -> &str {
        &self.beian
    }

    fn get_beian_for_reflect(&self) -> &::std::string::String {
        &self.beian
    }

    fn mut_beian_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.beian
    }

    // string descriptions = 6;

    pub fn clear_descriptions(&mut self) {
        self.descriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptions(&mut self, v: ::std::string::String) {
        self.descriptions = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descriptions(&mut self) -> &mut ::std::string::String {
        &mut self.descriptions
    }

    // Take field
    pub fn take_descriptions(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descriptions, ::std::string::String::new())
    }

    pub fn get_descriptions(&self) -> &str {
        &self.descriptions
    }

    fn get_descriptions_for_reflect(&self) -> &::std::string::String {
        &self.descriptions
    }

    fn mut_descriptions_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.descriptions
    }

    // string selfDescriptions = 7;

    pub fn clear_selfDescriptions(&mut self) {
        self.selfDescriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_selfDescriptions(&mut self, v: ::std::string::String) {
        self.selfDescriptions = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selfDescriptions(&mut self) -> &mut ::std::string::String {
        &mut self.selfDescriptions
    }

    // Take field
    pub fn take_selfDescriptions(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.selfDescriptions, ::std::string::String::new())
    }

    pub fn get_selfDescriptions(&self) -> &str {
        &self.selfDescriptions
    }

    fn get_selfDescriptions_for_reflect(&self) -> &::std::string::String {
        &self.selfDescriptions
    }

    fn mut_selfDescriptions_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.selfDescriptions
    }

    // uint32 id = 8;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // string title = 9;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }
}

impl ::protobuf::Message for SiteInfoResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.author)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.github)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.beian)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descriptions)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.selfDescriptions)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.author.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.author);
        }
        if !self.github.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.github);
        }
        if !self.beian.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.beian);
        }
        if !self.descriptions.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.descriptions);
        }
        if !self.selfDescriptions.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.selfDescriptions);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(8, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.title);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.author.is_empty() {
            os.write_string(3, &self.author)?;
        }
        if !self.github.is_empty() {
            os.write_string(4, &self.github)?;
        }
        if !self.beian.is_empty() {
            os.write_string(5, &self.beian)?;
        }
        if !self.descriptions.is_empty() {
            os.write_string(6, &self.descriptions)?;
        }
        if !self.selfDescriptions.is_empty() {
            os.write_string(7, &self.selfDescriptions)?;
        }
        if self.id != 0 {
            os.write_uint32(8, self.id)?;
        }
        if !self.title.is_empty() {
            os.write_string(9, &self.title)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SiteInfoResp {
    fn new() -> SiteInfoResp {
        SiteInfoResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<SiteInfoResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    SiteInfoResp::get_code_for_reflect,
                    SiteInfoResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    SiteInfoResp::get_msg_for_reflect,
                    SiteInfoResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "author",
                    SiteInfoResp::get_author_for_reflect,
                    SiteInfoResp::mut_author_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "github",
                    SiteInfoResp::get_github_for_reflect,
                    SiteInfoResp::mut_github_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "beian",
                    SiteInfoResp::get_beian_for_reflect,
                    SiteInfoResp::mut_beian_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "descriptions",
                    SiteInfoResp::get_descriptions_for_reflect,
                    SiteInfoResp::mut_descriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "selfDescriptions",
                    SiteInfoResp::get_selfDescriptions_for_reflect,
                    SiteInfoResp::mut_selfDescriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    SiteInfoResp::get_id_for_reflect,
                    SiteInfoResp::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    SiteInfoResp::get_title_for_reflect,
                    SiteInfoResp::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SiteInfoResp>(
                    "SiteInfoResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SiteInfoResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_author();
        self.clear_github();
        self.clear_beian();
        self.clear_descriptions();
        self.clear_selfDescriptions();
        self.clear_id();
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SiteInfoResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SiteInfoResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BrowseList {
    // message fields
    pub articleId: ::std::string::String,
    pub title: ::std::string::String,
    pub count: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BrowseList {}

impl BrowseList {
    pub fn new() -> BrowseList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BrowseList {
        static mut instance: ::protobuf::lazy::Lazy<BrowseList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BrowseList,
        };
        unsafe {
            instance.get(BrowseList::new)
        }
    }

    // string articleId = 1;

    pub fn clear_articleId(&mut self) {
        self.articleId.clear();
    }

    // Param is passed by value, moved
    pub fn set_articleId(&mut self, v: ::std::string::String) {
        self.articleId = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_articleId(&mut self) -> &mut ::std::string::String {
        &mut self.articleId
    }

    // Take field
    pub fn take_articleId(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.articleId, ::std::string::String::new())
    }

    pub fn get_articleId(&self) -> &str {
        &self.articleId
    }

    fn get_articleId_for_reflect(&self) -> &::std::string::String {
        &self.articleId
    }

    fn mut_articleId_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.articleId
    }

    // string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // uint32 count = 3;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u32 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u32 {
        &mut self.count
    }
}

impl ::protobuf::Message for BrowseList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.articleId)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.count = tmp;
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
        if !self.articleId.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.articleId);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.articleId.is_empty() {
            os.write_string(1, &self.articleId)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if self.count != 0 {
            os.write_uint32(3, self.count)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BrowseList {
    fn new() -> BrowseList {
        BrowseList::new()
    }

    fn descriptor_static(_: ::std::option::Option<BrowseList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "articleId",
                    BrowseList::get_articleId_for_reflect,
                    BrowseList::mut_articleId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    BrowseList::get_title_for_reflect,
                    BrowseList::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    BrowseList::get_count_for_reflect,
                    BrowseList::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BrowseList>(
                    "BrowseList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BrowseList {
    fn clear(&mut self) {
        self.clear_articleId();
        self.clear_title();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BrowseList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BrowseList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopCommentList {
    // message fields
    pub articleId: ::std::string::String,
    pub avatar: ::std::string::String,
    pub title: ::std::string::String,
    pub username: ::std::string::String,
    pub content: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopCommentList {}

impl TopCommentList {
    pub fn new() -> TopCommentList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopCommentList {
        static mut instance: ::protobuf::lazy::Lazy<TopCommentList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopCommentList,
        };
        unsafe {
            instance.get(TopCommentList::new)
        }
    }

    // string articleId = 1;

    pub fn clear_articleId(&mut self) {
        self.articleId.clear();
    }

    // Param is passed by value, moved
    pub fn set_articleId(&mut self, v: ::std::string::String) {
        self.articleId = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_articleId(&mut self) -> &mut ::std::string::String {
        &mut self.articleId
    }

    // Take field
    pub fn take_articleId(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.articleId, ::std::string::String::new())
    }

    pub fn get_articleId(&self) -> &str {
        &self.articleId
    }

    fn get_articleId_for_reflect(&self) -> &::std::string::String {
        &self.articleId
    }

    fn mut_articleId_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.articleId
    }

    // string avatar = 2;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: ::std::string::String) {
        self.avatar = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // Take field
    pub fn take_avatar(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.avatar, ::std::string::String::new())
    }

    pub fn get_avatar(&self) -> &str {
        &self.avatar
    }

    fn get_avatar_for_reflect(&self) -> &::std::string::String {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // string title = 3;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string username = 4;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // string content = 5;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }
}

impl ::protobuf::Message for TopCommentList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.articleId)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.avatar)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
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
        if !self.articleId.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.articleId);
        }
        if !self.avatar.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.avatar);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.title);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.username);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.articleId.is_empty() {
            os.write_string(1, &self.articleId)?;
        }
        if !self.avatar.is_empty() {
            os.write_string(2, &self.avatar)?;
        }
        if !self.title.is_empty() {
            os.write_string(3, &self.title)?;
        }
        if !self.username.is_empty() {
            os.write_string(4, &self.username)?;
        }
        if !self.content.is_empty() {
            os.write_string(5, &self.content)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopCommentList {
    fn new() -> TopCommentList {
        TopCommentList::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopCommentList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "articleId",
                    TopCommentList::get_articleId_for_reflect,
                    TopCommentList::mut_articleId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "avatar",
                    TopCommentList::get_avatar_for_reflect,
                    TopCommentList::mut_avatar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    TopCommentList::get_title_for_reflect,
                    TopCommentList::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    TopCommentList::get_username_for_reflect,
                    TopCommentList::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    TopCommentList::get_content_for_reflect,
                    TopCommentList::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopCommentList>(
                    "TopCommentList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopCommentList {
    fn clear(&mut self) {
        self.clear_articleId();
        self.clear_avatar();
        self.clear_title();
        self.clear_username();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopCommentList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopCommentList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopCommentResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub loveCount: ::std::string::String,
    pub browseList: ::protobuf::RepeatedField<BrowseList>,
    pub topCommentList: ::protobuf::RepeatedField<TopCommentList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopCommentResp {}

impl TopCommentResp {
    pub fn new() -> TopCommentResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopCommentResp {
        static mut instance: ::protobuf::lazy::Lazy<TopCommentResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopCommentResp,
        };
        unsafe {
            instance.get(TopCommentResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string loveCount = 3;

    pub fn clear_loveCount(&mut self) {
        self.loveCount.clear();
    }

    // Param is passed by value, moved
    pub fn set_loveCount(&mut self, v: ::std::string::String) {
        self.loveCount = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_loveCount(&mut self) -> &mut ::std::string::String {
        &mut self.loveCount
    }

    // Take field
    pub fn take_loveCount(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.loveCount, ::std::string::String::new())
    }

    pub fn get_loveCount(&self) -> &str {
        &self.loveCount
    }

    fn get_loveCount_for_reflect(&self) -> &::std::string::String {
        &self.loveCount
    }

    fn mut_loveCount_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.loveCount
    }

    // repeated .proto.BrowseList browseList = 4;

    pub fn clear_browseList(&mut self) {
        self.browseList.clear();
    }

    // Param is passed by value, moved
    pub fn set_browseList(&mut self, v: ::protobuf::RepeatedField<BrowseList>) {
        self.browseList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_browseList(&mut self) -> &mut ::protobuf::RepeatedField<BrowseList> {
        &mut self.browseList
    }

    // Take field
    pub fn take_browseList(&mut self) -> ::protobuf::RepeatedField<BrowseList> {
        ::std::mem::replace(&mut self.browseList, ::protobuf::RepeatedField::new())
    }

    pub fn get_browseList(&self) -> &[BrowseList] {
        &self.browseList
    }

    fn get_browseList_for_reflect(&self) -> &::protobuf::RepeatedField<BrowseList> {
        &self.browseList
    }

    fn mut_browseList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BrowseList> {
        &mut self.browseList
    }

    // repeated .proto.TopCommentList topCommentList = 5;

    pub fn clear_topCommentList(&mut self) {
        self.topCommentList.clear();
    }

    // Param is passed by value, moved
    pub fn set_topCommentList(&mut self, v: ::protobuf::RepeatedField<TopCommentList>) {
        self.topCommentList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_topCommentList(&mut self) -> &mut ::protobuf::RepeatedField<TopCommentList> {
        &mut self.topCommentList
    }

    // Take field
    pub fn take_topCommentList(&mut self) -> ::protobuf::RepeatedField<TopCommentList> {
        ::std::mem::replace(&mut self.topCommentList, ::protobuf::RepeatedField::new())
    }

    pub fn get_topCommentList(&self) -> &[TopCommentList] {
        &self.topCommentList
    }

    fn get_topCommentList_for_reflect(&self) -> &::protobuf::RepeatedField<TopCommentList> {
        &self.topCommentList
    }

    fn mut_topCommentList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TopCommentList> {
        &mut self.topCommentList
    }
}

impl ::protobuf::Message for TopCommentResp {
    fn is_initialized(&self) -> bool {
        for v in &self.browseList {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.topCommentList {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.loveCount)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.browseList)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.topCommentList)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.loveCount.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.loveCount);
        }
        for value in &self.browseList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.topCommentList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.loveCount.is_empty() {
            os.write_string(3, &self.loveCount)?;
        }
        for v in &self.browseList {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.topCommentList {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopCommentResp {
    fn new() -> TopCommentResp {
        TopCommentResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopCommentResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    TopCommentResp::get_code_for_reflect,
                    TopCommentResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    TopCommentResp::get_msg_for_reflect,
                    TopCommentResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "loveCount",
                    TopCommentResp::get_loveCount_for_reflect,
                    TopCommentResp::mut_loveCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BrowseList>>(
                    "browseList",
                    TopCommentResp::get_browseList_for_reflect,
                    TopCommentResp::mut_browseList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TopCommentList>>(
                    "topCommentList",
                    TopCommentResp::get_topCommentList_for_reflect,
                    TopCommentResp::mut_topCommentList_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopCommentResp>(
                    "TopCommentResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopCommentResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_loveCount();
        self.clear_browseList();
        self.clear_topCommentList();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopCommentResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopCommentResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClassList {
    // message fields
    pub count: u32,
    pub createDate: ::std::string::String,
    pub lastModifiedDate: ::std::string::String,
    pub name: ::std::string::String,
    pub state: u32,
    pub _id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClassList {}

impl ClassList {
    pub fn new() -> ClassList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClassList {
        static mut instance: ::protobuf::lazy::Lazy<ClassList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClassList,
        };
        unsafe {
            instance.get(ClassList::new)
        }
    }

    // uint32 count = 1;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u32 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u32 {
        &mut self.count
    }

    // string createDate = 2;

    pub fn clear_createDate(&mut self) {
        self.createDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_createDate(&mut self, v: ::std::string::String) {
        self.createDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createDate(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // Take field
    pub fn take_createDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createDate, ::std::string::String::new())
    }

    pub fn get_createDate(&self) -> &str {
        &self.createDate
    }

    fn get_createDate_for_reflect(&self) -> &::std::string::String {
        &self.createDate
    }

    fn mut_createDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // string lastModifiedDate = 3;

    pub fn clear_lastModifiedDate(&mut self) {
        self.lastModifiedDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_lastModifiedDate(&mut self, v: ::std::string::String) {
        self.lastModifiedDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastModifiedDate(&mut self) -> &mut ::std::string::String {
        &mut self.lastModifiedDate
    }

    // Take field
    pub fn take_lastModifiedDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.lastModifiedDate, ::std::string::String::new())
    }

    pub fn get_lastModifiedDate(&self) -> &str {
        &self.lastModifiedDate
    }

    fn get_lastModifiedDate_for_reflect(&self) -> &::std::string::String {
        &self.lastModifiedDate
    }

    fn mut_lastModifiedDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.lastModifiedDate
    }

    // string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // uint32 state = 5;

    pub fn clear_state(&mut self) {
        self.state = 0;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: u32) {
        self.state = v;
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }

    fn get_state_for_reflect(&self) -> &u32 {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut u32 {
        &mut self.state
    }

    // uint32 _id = 6;

    pub fn clear__id(&mut self) {
        self._id = 0;
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u32) {
        self._id = v;
    }

    pub fn get__id(&self) -> u32 {
        self._id
    }

    fn get__id_for_reflect(&self) -> &u32 {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut u32 {
        &mut self._id
    }
}

impl ::protobuf::Message for ClassList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.count = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createDate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.lastModifiedDate)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.state = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self._id = tmp;
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
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.createDate.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.createDate);
        }
        if !self.lastModifiedDate.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.lastModifiedDate);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.name);
        }
        if self.state != 0 {
            my_size += ::protobuf::rt::value_size(5, self.state, ::protobuf::wire_format::WireTypeVarint);
        }
        if self._id != 0 {
            my_size += ::protobuf::rt::value_size(6, self._id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.count != 0 {
            os.write_uint32(1, self.count)?;
        }
        if !self.createDate.is_empty() {
            os.write_string(2, &self.createDate)?;
        }
        if !self.lastModifiedDate.is_empty() {
            os.write_string(3, &self.lastModifiedDate)?;
        }
        if !self.name.is_empty() {
            os.write_string(4, &self.name)?;
        }
        if self.state != 0 {
            os.write_uint32(5, self.state)?;
        }
        if self._id != 0 {
            os.write_uint32(6, self._id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClassList {
    fn new() -> ClassList {
        ClassList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClassList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    ClassList::get_count_for_reflect,
                    ClassList::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createDate",
                    ClassList::get_createDate_for_reflect,
                    ClassList::mut_createDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lastModifiedDate",
                    ClassList::get_lastModifiedDate_for_reflect,
                    ClassList::mut_lastModifiedDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ClassList::get_name_for_reflect,
                    ClassList::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "state",
                    ClassList::get_state_for_reflect,
                    ClassList::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "_id",
                    ClassList::get__id_for_reflect,
                    ClassList::mut__id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClassList>(
                    "ClassList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClassList {
    fn clear(&mut self) {
        self.clear_count();
        self.clear_createDate();
        self.clear_lastModifiedDate();
        self.clear_name();
        self.clear_state();
        self.clear__id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClassList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClassList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListYear {
    // message fields
    pub list: ::protobuf::RepeatedField<ListYear_ListItem>,
    pub year: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListYear {}

impl ListYear {
    pub fn new() -> ListYear {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListYear {
        static mut instance: ::protobuf::lazy::Lazy<ListYear> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListYear,
        };
        unsafe {
            instance.get(ListYear::new)
        }
    }

    // repeated .proto.ListYear.ListItem list = 1;

    pub fn clear_list(&mut self) {
        self.list.clear();
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: ::protobuf::RepeatedField<ListYear_ListItem>) {
        self.list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_list(&mut self) -> &mut ::protobuf::RepeatedField<ListYear_ListItem> {
        &mut self.list
    }

    // Take field
    pub fn take_list(&mut self) -> ::protobuf::RepeatedField<ListYear_ListItem> {
        ::std::mem::replace(&mut self.list, ::protobuf::RepeatedField::new())
    }

    pub fn get_list(&self) -> &[ListYear_ListItem] {
        &self.list
    }

    fn get_list_for_reflect(&self) -> &::protobuf::RepeatedField<ListYear_ListItem> {
        &self.list
    }

    fn mut_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListYear_ListItem> {
        &mut self.list
    }

    // string year = 2;

    pub fn clear_year(&mut self) {
        self.year.clear();
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: ::std::string::String) {
        self.year = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_year(&mut self) -> &mut ::std::string::String {
        &mut self.year
    }

    // Take field
    pub fn take_year(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.year, ::std::string::String::new())
    }

    pub fn get_year(&self) -> &str {
        &self.year
    }

    fn get_year_for_reflect(&self) -> &::std::string::String {
        &self.year
    }

    fn mut_year_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.year
    }
}

impl ::protobuf::Message for ListYear {
    fn is_initialized(&self) -> bool {
        for v in &self.list {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.list)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.year)?;
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
        for value in &self.list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.year.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.year);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.list {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.year.is_empty() {
            os.write_string(2, &self.year)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListYear {
    fn new() -> ListYear {
        ListYear::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListYear>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListYear_ListItem>>(
                    "list",
                    ListYear::get_list_for_reflect,
                    ListYear::mut_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "year",
                    ListYear::get_year_for_reflect,
                    ListYear::mut_year_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListYear>(
                    "ListYear",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListYear {
    fn clear(&mut self) {
        self.clear_list();
        self.clear_year();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListYear {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListYear {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListYear_ListItem {
    // message fields
    pub createDate: ::std::string::String,
    pub title: ::std::string::String,
    pub _id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListYear_ListItem {}

impl ListYear_ListItem {
    pub fn new() -> ListYear_ListItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListYear_ListItem {
        static mut instance: ::protobuf::lazy::Lazy<ListYear_ListItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListYear_ListItem,
        };
        unsafe {
            instance.get(ListYear_ListItem::new)
        }
    }

    // string createDate = 1;

    pub fn clear_createDate(&mut self) {
        self.createDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_createDate(&mut self, v: ::std::string::String) {
        self.createDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createDate(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // Take field
    pub fn take_createDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createDate, ::std::string::String::new())
    }

    pub fn get_createDate(&self) -> &str {
        &self.createDate
    }

    fn get_createDate_for_reflect(&self) -> &::std::string::String {
        &self.createDate
    }

    fn mut_createDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // uint32 _id = 3;

    pub fn clear__id(&mut self) {
        self._id = 0;
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u32) {
        self._id = v;
    }

    pub fn get__id(&self) -> u32 {
        self._id
    }

    fn get__id_for_reflect(&self) -> &u32 {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut u32 {
        &mut self._id
    }
}

impl ::protobuf::Message for ListYear_ListItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createDate)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self._id = tmp;
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
        if !self.createDate.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.createDate);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if self._id != 0 {
            my_size += ::protobuf::rt::value_size(3, self._id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.createDate.is_empty() {
            os.write_string(1, &self.createDate)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if self._id != 0 {
            os.write_uint32(3, self._id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListYear_ListItem {
    fn new() -> ListYear_ListItem {
        ListYear_ListItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListYear_ListItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createDate",
                    ListYear_ListItem::get_createDate_for_reflect,
                    ListYear_ListItem::mut_createDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    ListYear_ListItem::get_title_for_reflect,
                    ListYear_ListItem::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "_id",
                    ListYear_ListItem::get__id_for_reflect,
                    ListYear_ListItem::mut__id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListYear_ListItem>(
                    "ListYear_ListItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListYear_ListItem {
    fn clear(&mut self) {
        self.clear_createDate();
        self.clear_title();
        self.clear__id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListYear_ListItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListYear_ListItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListByClass {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub articleList: ::protobuf::RepeatedField<ListYear>,
    pub classList: ::protobuf::RepeatedField<ClassList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListByClass {}

impl ListByClass {
    pub fn new() -> ListByClass {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListByClass {
        static mut instance: ::protobuf::lazy::Lazy<ListByClass> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListByClass,
        };
        unsafe {
            instance.get(ListByClass::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated .proto.ListYear articleList = 3;

    pub fn clear_articleList(&mut self) {
        self.articleList.clear();
    }

    // Param is passed by value, moved
    pub fn set_articleList(&mut self, v: ::protobuf::RepeatedField<ListYear>) {
        self.articleList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_articleList(&mut self) -> &mut ::protobuf::RepeatedField<ListYear> {
        &mut self.articleList
    }

    // Take field
    pub fn take_articleList(&mut self) -> ::protobuf::RepeatedField<ListYear> {
        ::std::mem::replace(&mut self.articleList, ::protobuf::RepeatedField::new())
    }

    pub fn get_articleList(&self) -> &[ListYear] {
        &self.articleList
    }

    fn get_articleList_for_reflect(&self) -> &::protobuf::RepeatedField<ListYear> {
        &self.articleList
    }

    fn mut_articleList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListYear> {
        &mut self.articleList
    }

    // repeated .proto.ClassList classList = 4;

    pub fn clear_classList(&mut self) {
        self.classList.clear();
    }

    // Param is passed by value, moved
    pub fn set_classList(&mut self, v: ::protobuf::RepeatedField<ClassList>) {
        self.classList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classList(&mut self) -> &mut ::protobuf::RepeatedField<ClassList> {
        &mut self.classList
    }

    // Take field
    pub fn take_classList(&mut self) -> ::protobuf::RepeatedField<ClassList> {
        ::std::mem::replace(&mut self.classList, ::protobuf::RepeatedField::new())
    }

    pub fn get_classList(&self) -> &[ClassList] {
        &self.classList
    }

    fn get_classList_for_reflect(&self) -> &::protobuf::RepeatedField<ClassList> {
        &self.classList
    }

    fn mut_classList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClassList> {
        &mut self.classList
    }
}

impl ::protobuf::Message for ListByClass {
    fn is_initialized(&self) -> bool {
        for v in &self.articleList {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.classList {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.articleList)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classList)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.articleList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.classList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.articleList {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.classList {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListByClass {
    fn new() -> ListByClass {
        ListByClass::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListByClass>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ListByClass::get_code_for_reflect,
                    ListByClass::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    ListByClass::get_msg_for_reflect,
                    ListByClass::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListYear>>(
                    "articleList",
                    ListByClass::get_articleList_for_reflect,
                    ListByClass::mut_articleList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClassList>>(
                    "classList",
                    ListByClass::get_classList_for_reflect,
                    ListByClass::mut_classList_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListByClass>(
                    "ListByClass",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListByClass {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_articleList();
        self.clear_classList();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListByClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListByClass {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Article {
    // message fields
    pub browseCount: u32,
    pub classId: u32,
    pub collectCount: u32,
    pub commentCount: u32,
    pub content: ::std::string::String,
    pub createDate: ::std::string::String,
    pub isHot: bool,
    pub isRecommend: bool,
    pub lastModifiedDate: ::std::string::String,
    pub likeCount: u32,
    pub state: u32,
    pub _id: u32,
    pub __v: u32,
    pub tags: ::std::vec::Vec<u32>,
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Article {}

impl Article {
    pub fn new() -> Article {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Article {
        static mut instance: ::protobuf::lazy::Lazy<Article> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Article,
        };
        unsafe {
            instance.get(Article::new)
        }
    }

    // uint32 browseCount = 1;

    pub fn clear_browseCount(&mut self) {
        self.browseCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_browseCount(&mut self, v: u32) {
        self.browseCount = v;
    }

    pub fn get_browseCount(&self) -> u32 {
        self.browseCount
    }

    fn get_browseCount_for_reflect(&self) -> &u32 {
        &self.browseCount
    }

    fn mut_browseCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.browseCount
    }

    // uint32 classId = 2;

    pub fn clear_classId(&mut self) {
        self.classId = 0;
    }

    // Param is passed by value, moved
    pub fn set_classId(&mut self, v: u32) {
        self.classId = v;
    }

    pub fn get_classId(&self) -> u32 {
        self.classId
    }

    fn get_classId_for_reflect(&self) -> &u32 {
        &self.classId
    }

    fn mut_classId_for_reflect(&mut self) -> &mut u32 {
        &mut self.classId
    }

    // uint32 collectCount = 3;

    pub fn clear_collectCount(&mut self) {
        self.collectCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_collectCount(&mut self, v: u32) {
        self.collectCount = v;
    }

    pub fn get_collectCount(&self) -> u32 {
        self.collectCount
    }

    fn get_collectCount_for_reflect(&self) -> &u32 {
        &self.collectCount
    }

    fn mut_collectCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.collectCount
    }

    // uint32 commentCount = 4;

    pub fn clear_commentCount(&mut self) {
        self.commentCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_commentCount(&mut self, v: u32) {
        self.commentCount = v;
    }

    pub fn get_commentCount(&self) -> u32 {
        self.commentCount
    }

    fn get_commentCount_for_reflect(&self) -> &u32 {
        &self.commentCount
    }

    fn mut_commentCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.commentCount
    }

    // string content = 5;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // string createDate = 6;

    pub fn clear_createDate(&mut self) {
        self.createDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_createDate(&mut self, v: ::std::string::String) {
        self.createDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createDate(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // Take field
    pub fn take_createDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createDate, ::std::string::String::new())
    }

    pub fn get_createDate(&self) -> &str {
        &self.createDate
    }

    fn get_createDate_for_reflect(&self) -> &::std::string::String {
        &self.createDate
    }

    fn mut_createDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // bool isHot = 7;

    pub fn clear_isHot(&mut self) {
        self.isHot = false;
    }

    // Param is passed by value, moved
    pub fn set_isHot(&mut self, v: bool) {
        self.isHot = v;
    }

    pub fn get_isHot(&self) -> bool {
        self.isHot
    }

    fn get_isHot_for_reflect(&self) -> &bool {
        &self.isHot
    }

    fn mut_isHot_for_reflect(&mut self) -> &mut bool {
        &mut self.isHot
    }

    // bool isRecommend = 8;

    pub fn clear_isRecommend(&mut self) {
        self.isRecommend = false;
    }

    // Param is passed by value, moved
    pub fn set_isRecommend(&mut self, v: bool) {
        self.isRecommend = v;
    }

    pub fn get_isRecommend(&self) -> bool {
        self.isRecommend
    }

    fn get_isRecommend_for_reflect(&self) -> &bool {
        &self.isRecommend
    }

    fn mut_isRecommend_for_reflect(&mut self) -> &mut bool {
        &mut self.isRecommend
    }

    // string lastModifiedDate = 9;

    pub fn clear_lastModifiedDate(&mut self) {
        self.lastModifiedDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_lastModifiedDate(&mut self, v: ::std::string::String) {
        self.lastModifiedDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastModifiedDate(&mut self) -> &mut ::std::string::String {
        &mut self.lastModifiedDate
    }

    // Take field
    pub fn take_lastModifiedDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.lastModifiedDate, ::std::string::String::new())
    }

    pub fn get_lastModifiedDate(&self) -> &str {
        &self.lastModifiedDate
    }

    fn get_lastModifiedDate_for_reflect(&self) -> &::std::string::String {
        &self.lastModifiedDate
    }

    fn mut_lastModifiedDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.lastModifiedDate
    }

    // uint32 likeCount = 10;

    pub fn clear_likeCount(&mut self) {
        self.likeCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_likeCount(&mut self, v: u32) {
        self.likeCount = v;
    }

    pub fn get_likeCount(&self) -> u32 {
        self.likeCount
    }

    fn get_likeCount_for_reflect(&self) -> &u32 {
        &self.likeCount
    }

    fn mut_likeCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.likeCount
    }

    // uint32 state = 11;

    pub fn clear_state(&mut self) {
        self.state = 0;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: u32) {
        self.state = v;
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }

    fn get_state_for_reflect(&self) -> &u32 {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut u32 {
        &mut self.state
    }

    // uint32 _id = 12;

    pub fn clear__id(&mut self) {
        self._id = 0;
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u32) {
        self._id = v;
    }

    pub fn get__id(&self) -> u32 {
        self._id
    }

    fn get__id_for_reflect(&self) -> &u32 {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut u32 {
        &mut self._id
    }

    // uint32 __v = 13;

    pub fn clear___v(&mut self) {
        self.__v = 0;
    }

    // Param is passed by value, moved
    pub fn set___v(&mut self, v: u32) {
        self.__v = v;
    }

    pub fn get___v(&self) -> u32 {
        self.__v
    }

    fn get___v_for_reflect(&self) -> &u32 {
        &self.__v
    }

    fn mut___v_for_reflect(&mut self) -> &mut u32 {
        &mut self.__v
    }

    // repeated uint32 tags = 14;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::std::vec::Vec<u32>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.tags, ::std::vec::Vec::new())
    }

    pub fn get_tags(&self) -> &[u32] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tags
    }

    // string title = 15;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }
}

impl ::protobuf::Message for Article {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.browseCount = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.classId = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.collectCount = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.commentCount = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createDate)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isHot = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isRecommend = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.lastModifiedDate)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.likeCount = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.state = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self._id = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.__v = tmp;
                },
                14 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.tags)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
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
        if self.browseCount != 0 {
            my_size += ::protobuf::rt::value_size(1, self.browseCount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.classId != 0 {
            my_size += ::protobuf::rt::value_size(2, self.classId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.collectCount != 0 {
            my_size += ::protobuf::rt::value_size(3, self.collectCount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commentCount != 0 {
            my_size += ::protobuf::rt::value_size(4, self.commentCount, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.content);
        }
        if !self.createDate.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.createDate);
        }
        if self.isHot != false {
            my_size += 2;
        }
        if self.isRecommend != false {
            my_size += 2;
        }
        if !self.lastModifiedDate.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.lastModifiedDate);
        }
        if self.likeCount != 0 {
            my_size += ::protobuf::rt::value_size(10, self.likeCount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.state != 0 {
            my_size += ::protobuf::rt::value_size(11, self.state, ::protobuf::wire_format::WireTypeVarint);
        }
        if self._id != 0 {
            my_size += ::protobuf::rt::value_size(12, self._id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.__v != 0 {
            my_size += ::protobuf::rt::value_size(13, self.__v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tags {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.title);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.browseCount != 0 {
            os.write_uint32(1, self.browseCount)?;
        }
        if self.classId != 0 {
            os.write_uint32(2, self.classId)?;
        }
        if self.collectCount != 0 {
            os.write_uint32(3, self.collectCount)?;
        }
        if self.commentCount != 0 {
            os.write_uint32(4, self.commentCount)?;
        }
        if !self.content.is_empty() {
            os.write_string(5, &self.content)?;
        }
        if !self.createDate.is_empty() {
            os.write_string(6, &self.createDate)?;
        }
        if self.isHot != false {
            os.write_bool(7, self.isHot)?;
        }
        if self.isRecommend != false {
            os.write_bool(8, self.isRecommend)?;
        }
        if !self.lastModifiedDate.is_empty() {
            os.write_string(9, &self.lastModifiedDate)?;
        }
        if self.likeCount != 0 {
            os.write_uint32(10, self.likeCount)?;
        }
        if self.state != 0 {
            os.write_uint32(11, self.state)?;
        }
        if self._id != 0 {
            os.write_uint32(12, self._id)?;
        }
        if self.__v != 0 {
            os.write_uint32(13, self.__v)?;
        }
        for v in &self.tags {
            os.write_uint32(14, *v)?;
        };
        if !self.title.is_empty() {
            os.write_string(15, &self.title)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Article {
    fn new() -> Article {
        Article::new()
    }

    fn descriptor_static(_: ::std::option::Option<Article>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "browseCount",
                    Article::get_browseCount_for_reflect,
                    Article::mut_browseCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "classId",
                    Article::get_classId_for_reflect,
                    Article::mut_classId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "collectCount",
                    Article::get_collectCount_for_reflect,
                    Article::mut_collectCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "commentCount",
                    Article::get_commentCount_for_reflect,
                    Article::mut_commentCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    Article::get_content_for_reflect,
                    Article::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createDate",
                    Article::get_createDate_for_reflect,
                    Article::mut_createDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isHot",
                    Article::get_isHot_for_reflect,
                    Article::mut_isHot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isRecommend",
                    Article::get_isRecommend_for_reflect,
                    Article::mut_isRecommend_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lastModifiedDate",
                    Article::get_lastModifiedDate_for_reflect,
                    Article::mut_lastModifiedDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "likeCount",
                    Article::get_likeCount_for_reflect,
                    Article::mut_likeCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "state",
                    Article::get_state_for_reflect,
                    Article::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "_id",
                    Article::get__id_for_reflect,
                    Article::mut__id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "__v",
                    Article::get___v_for_reflect,
                    Article::mut___v_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tags",
                    Article::get_tags_for_reflect,
                    Article::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    Article::get_title_for_reflect,
                    Article::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Article>(
                    "Article",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Article {
    fn clear(&mut self) {
        self.clear_browseCount();
        self.clear_classId();
        self.clear_collectCount();
        self.clear_commentCount();
        self.clear_content();
        self.clear_createDate();
        self.clear_isHot();
        self.clear_isRecommend();
        self.clear_lastModifiedDate();
        self.clear_likeCount();
        self.clear_state();
        self.clear__id();
        self.clear___v();
        self.clear_tags();
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Article {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Article {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ArticleListResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub list: ::protobuf::RepeatedField<Article>,
    pub pagination: ::protobuf::SingularPtrField<Pagination>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ArticleListResp {}

impl ArticleListResp {
    pub fn new() -> ArticleListResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ArticleListResp {
        static mut instance: ::protobuf::lazy::Lazy<ArticleListResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ArticleListResp,
        };
        unsafe {
            instance.get(ArticleListResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated .proto.Article list = 3;

    pub fn clear_list(&mut self) {
        self.list.clear();
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: ::protobuf::RepeatedField<Article>) {
        self.list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_list(&mut self) -> &mut ::protobuf::RepeatedField<Article> {
        &mut self.list
    }

    // Take field
    pub fn take_list(&mut self) -> ::protobuf::RepeatedField<Article> {
        ::std::mem::replace(&mut self.list, ::protobuf::RepeatedField::new())
    }

    pub fn get_list(&self) -> &[Article] {
        &self.list
    }

    fn get_list_for_reflect(&self) -> &::protobuf::RepeatedField<Article> {
        &self.list
    }

    fn mut_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Article> {
        &mut self.list
    }

    // .proto.Pagination pagination = 4;

    pub fn clear_pagination(&mut self) {
        self.pagination.clear();
    }

    pub fn has_pagination(&self) -> bool {
        self.pagination.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pagination(&mut self, v: Pagination) {
        self.pagination = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pagination(&mut self) -> &mut Pagination {
        if self.pagination.is_none() {
            self.pagination.set_default();
        }
        self.pagination.as_mut().unwrap()
    }

    // Take field
    pub fn take_pagination(&mut self) -> Pagination {
        self.pagination.take().unwrap_or_else(|| Pagination::new())
    }

    pub fn get_pagination(&self) -> &Pagination {
        self.pagination.as_ref().unwrap_or_else(|| Pagination::default_instance())
    }

    fn get_pagination_for_reflect(&self) -> &::protobuf::SingularPtrField<Pagination> {
        &self.pagination
    }

    fn mut_pagination_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Pagination> {
        &mut self.pagination
    }
}

impl ::protobuf::Message for ArticleListResp {
    fn is_initialized(&self) -> bool {
        for v in &self.list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pagination {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.list)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pagination)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.pagination.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.list {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.pagination.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ArticleListResp {
    fn new() -> ArticleListResp {
        ArticleListResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<ArticleListResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ArticleListResp::get_code_for_reflect,
                    ArticleListResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    ArticleListResp::get_msg_for_reflect,
                    ArticleListResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Article>>(
                    "list",
                    ArticleListResp::get_list_for_reflect,
                    ArticleListResp::mut_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Pagination>>(
                    "pagination",
                    ArticleListResp::get_pagination_for_reflect,
                    ArticleListResp::mut_pagination_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ArticleListResp>(
                    "ArticleListResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ArticleListResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_list();
        self.clear_pagination();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ArticleListResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ArticleListResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ArticleOneResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub obj: ::protobuf::SingularPtrField<Article>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ArticleOneResp {}

impl ArticleOneResp {
    pub fn new() -> ArticleOneResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ArticleOneResp {
        static mut instance: ::protobuf::lazy::Lazy<ArticleOneResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ArticleOneResp,
        };
        unsafe {
            instance.get(ArticleOneResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // .proto.Article obj = 3;

    pub fn clear_obj(&mut self) {
        self.obj.clear();
    }

    pub fn has_obj(&self) -> bool {
        self.obj.is_some()
    }

    // Param is passed by value, moved
    pub fn set_obj(&mut self, v: Article) {
        self.obj = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_obj(&mut self) -> &mut Article {
        if self.obj.is_none() {
            self.obj.set_default();
        }
        self.obj.as_mut().unwrap()
    }

    // Take field
    pub fn take_obj(&mut self) -> Article {
        self.obj.take().unwrap_or_else(|| Article::new())
    }

    pub fn get_obj(&self) -> &Article {
        self.obj.as_ref().unwrap_or_else(|| Article::default_instance())
    }

    fn get_obj_for_reflect(&self) -> &::protobuf::SingularPtrField<Article> {
        &self.obj
    }

    fn mut_obj_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Article> {
        &mut self.obj
    }
}

impl ::protobuf::Message for ArticleOneResp {
    fn is_initialized(&self) -> bool {
        for v in &self.obj {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.obj)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if let Some(ref v) = self.obj.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if let Some(ref v) = self.obj.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ArticleOneResp {
    fn new() -> ArticleOneResp {
        ArticleOneResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<ArticleOneResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ArticleOneResp::get_code_for_reflect,
                    ArticleOneResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    ArticleOneResp::get_msg_for_reflect,
                    ArticleOneResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Article>>(
                    "obj",
                    ArticleOneResp::get_obj_for_reflect,
                    ArticleOneResp::mut_obj_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ArticleOneResp>(
                    "ArticleOneResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ArticleOneResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_obj();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ArticleOneResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ArticleOneResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CaptchaResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub id: ::std::string::String,
    pub img: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CaptchaResp {}

impl CaptchaResp {
    pub fn new() -> CaptchaResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CaptchaResp {
        static mut instance: ::protobuf::lazy::Lazy<CaptchaResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CaptchaResp,
        };
        unsafe {
            instance.get(CaptchaResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string id = 3;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string img = 4;

    pub fn clear_img(&mut self) {
        self.img.clear();
    }

    // Param is passed by value, moved
    pub fn set_img(&mut self, v: ::std::string::String) {
        self.img = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_img(&mut self) -> &mut ::std::string::String {
        &mut self.img
    }

    // Take field
    pub fn take_img(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.img, ::std::string::String::new())
    }

    pub fn get_img(&self) -> &str {
        &self.img
    }

    fn get_img_for_reflect(&self) -> &::std::string::String {
        &self.img
    }

    fn mut_img_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.img
    }
}

impl ::protobuf::Message for CaptchaResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.img)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.id);
        }
        if !self.img.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.img);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.id.is_empty() {
            os.write_string(3, &self.id)?;
        }
        if !self.img.is_empty() {
            os.write_string(4, &self.img)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CaptchaResp {
    fn new() -> CaptchaResp {
        CaptchaResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CaptchaResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    CaptchaResp::get_code_for_reflect,
                    CaptchaResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    CaptchaResp::get_msg_for_reflect,
                    CaptchaResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    CaptchaResp::get_id_for_reflect,
                    CaptchaResp::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "img",
                    CaptchaResp::get_img_for_reflect,
                    CaptchaResp::mut_img_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CaptchaResp>(
                    "CaptchaResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CaptchaResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_id();
        self.clear_img();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CaptchaResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CaptchaResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ComponentMeta {
    // message fields
    pub title: ::std::string::String,
    pub name: ::std::string::String,
    pub icon: ::std::string::String,
    pub noCache: bool,
    pub affix: bool,
    pub activeMenu: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ComponentMeta {}

impl ComponentMeta {
    pub fn new() -> ComponentMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ComponentMeta {
        static mut instance: ::protobuf::lazy::Lazy<ComponentMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ComponentMeta,
        };
        unsafe {
            instance.get(ComponentMeta::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string icon = 3;

    pub fn clear_icon(&mut self) {
        self.icon.clear();
    }

    // Param is passed by value, moved
    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_icon(&mut self) -> &mut ::std::string::String {
        &mut self.icon
    }

    // Take field
    pub fn take_icon(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.icon, ::std::string::String::new())
    }

    pub fn get_icon(&self) -> &str {
        &self.icon
    }

    fn get_icon_for_reflect(&self) -> &::std::string::String {
        &self.icon
    }

    fn mut_icon_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.icon
    }

    // bool noCache = 4;

    pub fn clear_noCache(&mut self) {
        self.noCache = false;
    }

    // Param is passed by value, moved
    pub fn set_noCache(&mut self, v: bool) {
        self.noCache = v;
    }

    pub fn get_noCache(&self) -> bool {
        self.noCache
    }

    fn get_noCache_for_reflect(&self) -> &bool {
        &self.noCache
    }

    fn mut_noCache_for_reflect(&mut self) -> &mut bool {
        &mut self.noCache
    }

    // bool affix = 5;

    pub fn clear_affix(&mut self) {
        self.affix = false;
    }

    // Param is passed by value, moved
    pub fn set_affix(&mut self, v: bool) {
        self.affix = v;
    }

    pub fn get_affix(&self) -> bool {
        self.affix
    }

    fn get_affix_for_reflect(&self) -> &bool {
        &self.affix
    }

    fn mut_affix_for_reflect(&mut self) -> &mut bool {
        &mut self.affix
    }

    // string activeMenu = 6;

    pub fn clear_activeMenu(&mut self) {
        self.activeMenu.clear();
    }

    // Param is passed by value, moved
    pub fn set_activeMenu(&mut self, v: ::std::string::String) {
        self.activeMenu = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_activeMenu(&mut self) -> &mut ::std::string::String {
        &mut self.activeMenu
    }

    // Take field
    pub fn take_activeMenu(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.activeMenu, ::std::string::String::new())
    }

    pub fn get_activeMenu(&self) -> &str {
        &self.activeMenu
    }

    fn get_activeMenu_for_reflect(&self) -> &::std::string::String {
        &self.activeMenu
    }

    fn mut_activeMenu_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.activeMenu
    }
}

impl ::protobuf::Message for ComponentMeta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.icon)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.noCache = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.affix = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.activeMenu)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.icon.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.icon);
        }
        if self.noCache != false {
            my_size += 2;
        }
        if self.affix != false {
            my_size += 2;
        }
        if !self.activeMenu.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.activeMenu);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.icon.is_empty() {
            os.write_string(3, &self.icon)?;
        }
        if self.noCache != false {
            os.write_bool(4, self.noCache)?;
        }
        if self.affix != false {
            os.write_bool(5, self.affix)?;
        }
        if !self.activeMenu.is_empty() {
            os.write_string(6, &self.activeMenu)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ComponentMeta {
    fn new() -> ComponentMeta {
        ComponentMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<ComponentMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    ComponentMeta::get_title_for_reflect,
                    ComponentMeta::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ComponentMeta::get_name_for_reflect,
                    ComponentMeta::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "icon",
                    ComponentMeta::get_icon_for_reflect,
                    ComponentMeta::mut_icon_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "noCache",
                    ComponentMeta::get_noCache_for_reflect,
                    ComponentMeta::mut_noCache_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "affix",
                    ComponentMeta::get_affix_for_reflect,
                    ComponentMeta::mut_affix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "activeMenu",
                    ComponentMeta::get_activeMenu_for_reflect,
                    ComponentMeta::mut_activeMenu_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ComponentMeta>(
                    "ComponentMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ComponentMeta {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_name();
        self.clear_icon();
        self.clear_noCache();
        self.clear_affix();
        self.clear_activeMenu();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ComponentMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ComponentMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Component {
    // message fields
    pub component: ::std::string::String,
    pub path: ::std::string::String,
    pub name: ::std::string::String,
    pub meta: ::protobuf::SingularPtrField<ComponentMeta>,
    pub hidden: bool,
    pub children: ::protobuf::RepeatedField<Component>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Component {}

impl Component {
    pub fn new() -> Component {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Component {
        static mut instance: ::protobuf::lazy::Lazy<Component> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Component,
        };
        unsafe {
            instance.get(Component::new)
        }
    }

    // string component = 1;

    pub fn clear_component(&mut self) {
        self.component.clear();
    }

    // Param is passed by value, moved
    pub fn set_component(&mut self, v: ::std::string::String) {
        self.component = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_component(&mut self) -> &mut ::std::string::String {
        &mut self.component
    }

    // Take field
    pub fn take_component(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.component, ::std::string::String::new())
    }

    pub fn get_component(&self) -> &str {
        &self.component
    }

    fn get_component_for_reflect(&self) -> &::std::string::String {
        &self.component
    }

    fn mut_component_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.component
    }

    // string path = 3;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .proto.ComponentMeta meta = 5;

    pub fn clear_meta(&mut self) {
        self.meta.clear();
    }

    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta(&mut self, v: ComponentMeta) {
        self.meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta(&mut self) -> &mut ComponentMeta {
        if self.meta.is_none() {
            self.meta.set_default();
        }
        self.meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta(&mut self) -> ComponentMeta {
        self.meta.take().unwrap_or_else(|| ComponentMeta::new())
    }

    pub fn get_meta(&self) -> &ComponentMeta {
        self.meta.as_ref().unwrap_or_else(|| ComponentMeta::default_instance())
    }

    fn get_meta_for_reflect(&self) -> &::protobuf::SingularPtrField<ComponentMeta> {
        &self.meta
    }

    fn mut_meta_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ComponentMeta> {
        &mut self.meta
    }

    // bool hidden = 6;

    pub fn clear_hidden(&mut self) {
        self.hidden = false;
    }

    // Param is passed by value, moved
    pub fn set_hidden(&mut self, v: bool) {
        self.hidden = v;
    }

    pub fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_hidden_for_reflect(&self) -> &bool {
        &self.hidden
    }

    fn mut_hidden_for_reflect(&mut self) -> &mut bool {
        &mut self.hidden
    }

    // repeated .proto.Component children = 7;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Component>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Component> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Component> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Component] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<Component> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Component> {
        &mut self.children
    }
}

impl ::protobuf::Message for Component {
    fn is_initialized(&self) -> bool {
        for v in &self.meta {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.children {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.component)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hidden = tmp;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
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
        if !self.component.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.component);
        }
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.path);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.name);
        }
        if let Some(ref v) = self.meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.hidden != false {
            my_size += 2;
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.component.is_empty() {
            os.write_string(1, &self.component)?;
        }
        if !self.path.is_empty() {
            os.write_string(3, &self.path)?;
        }
        if !self.name.is_empty() {
            os.write_string(4, &self.name)?;
        }
        if let Some(ref v) = self.meta.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.hidden != false {
            os.write_bool(6, self.hidden)?;
        }
        for v in &self.children {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Component {
    fn new() -> Component {
        Component::new()
    }

    fn descriptor_static(_: ::std::option::Option<Component>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "component",
                    Component::get_component_for_reflect,
                    Component::mut_component_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    Component::get_path_for_reflect,
                    Component::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Component::get_name_for_reflect,
                    Component::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ComponentMeta>>(
                    "meta",
                    Component::get_meta_for_reflect,
                    Component::mut_meta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hidden",
                    Component::get_hidden_for_reflect,
                    Component::mut_hidden_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Component>>(
                    "children",
                    Component::get_children_for_reflect,
                    Component::mut_children_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Component>(
                    "Component",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Component {
    fn clear(&mut self) {
        self.clear_component();
        self.clear_path();
        self.clear_name();
        self.clear_meta();
        self.clear_hidden();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Component {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Component {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminRouterResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub data: ::protobuf::RepeatedField<Component>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminRouterResp {}

impl AdminRouterResp {
    pub fn new() -> AdminRouterResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminRouterResp {
        static mut instance: ::protobuf::lazy::Lazy<AdminRouterResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminRouterResp,
        };
        unsafe {
            instance.get(AdminRouterResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated .proto.Component data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::protobuf::RepeatedField<Component>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::protobuf::RepeatedField<Component> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::protobuf::RepeatedField<Component> {
        ::std::mem::replace(&mut self.data, ::protobuf::RepeatedField::new())
    }

    pub fn get_data(&self) -> &[Component] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::protobuf::RepeatedField<Component> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Component> {
        &mut self.data
    }
}

impl ::protobuf::Message for AdminRouterResp {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.data {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminRouterResp {
    fn new() -> AdminRouterResp {
        AdminRouterResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminRouterResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    AdminRouterResp::get_code_for_reflect,
                    AdminRouterResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    AdminRouterResp::get_msg_for_reflect,
                    AdminRouterResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Component>>(
                    "data",
                    AdminRouterResp::get_data_for_reflect,
                    AdminRouterResp::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminRouterResp>(
                    "AdminRouterResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminRouterResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminRouterResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminRouterResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminCategoryAddRequest {
    // message fields
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    pub support: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminCategoryAddRequest {}

impl AdminCategoryAddRequest {
    pub fn new() -> AdminCategoryAddRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminCategoryAddRequest {
        static mut instance: ::protobuf::lazy::Lazy<AdminCategoryAddRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminCategoryAddRequest,
        };
        unsafe {
            instance.get(AdminCategoryAddRequest::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // bool support = 3;

    pub fn clear_support(&mut self) {
        self.support = false;
    }

    // Param is passed by value, moved
    pub fn set_support(&mut self, v: bool) {
        self.support = v;
    }

    pub fn get_support(&self) -> bool {
        self.support
    }

    fn get_support_for_reflect(&self) -> &bool {
        &self.support
    }

    fn mut_support_for_reflect(&mut self) -> &mut bool {
        &mut self.support
    }
}

impl ::protobuf::Message for AdminCategoryAddRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.support = tmp;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if self.support != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if self.support != false {
            os.write_bool(3, self.support)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminCategoryAddRequest {
    fn new() -> AdminCategoryAddRequest {
        AdminCategoryAddRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminCategoryAddRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminCategoryAddRequest::get_title_for_reflect,
                    AdminCategoryAddRequest::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    AdminCategoryAddRequest::get_description_for_reflect,
                    AdminCategoryAddRequest::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "support",
                    AdminCategoryAddRequest::get_support_for_reflect,
                    AdminCategoryAddRequest::mut_support_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminCategoryAddRequest>(
                    "AdminCategoryAddRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminCategoryAddRequest {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_description();
        self.clear_support();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminCategoryAddRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminCategoryAddRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminCategoryListResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub total: u32,
    pub rows: ::protobuf::RepeatedField<AdminCategoryListResp_categoryBase>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminCategoryListResp {}

impl AdminCategoryListResp {
    pub fn new() -> AdminCategoryListResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminCategoryListResp {
        static mut instance: ::protobuf::lazy::Lazy<AdminCategoryListResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminCategoryListResp,
        };
        unsafe {
            instance.get(AdminCategoryListResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // uint32 total = 3;

    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u32) {
        self.total = v;
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    fn get_total_for_reflect(&self) -> &u32 {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut u32 {
        &mut self.total
    }

    // repeated .proto.AdminCategoryListResp.categoryBase rows = 4;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<AdminCategoryListResp_categoryBase>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<AdminCategoryListResp_categoryBase> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<AdminCategoryListResp_categoryBase> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[AdminCategoryListResp_categoryBase] {
        &self.rows
    }

    fn get_rows_for_reflect(&self) -> &::protobuf::RepeatedField<AdminCategoryListResp_categoryBase> {
        &self.rows
    }

    fn mut_rows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AdminCategoryListResp_categoryBase> {
        &mut self.rows
    }
}

impl ::protobuf::Message for AdminCategoryListResp {
    fn is_initialized(&self) -> bool {
        for v in &self.rows {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.total != 0 {
            os.write_uint32(3, self.total)?;
        }
        for v in &self.rows {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminCategoryListResp {
    fn new() -> AdminCategoryListResp {
        AdminCategoryListResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminCategoryListResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    AdminCategoryListResp::get_code_for_reflect,
                    AdminCategoryListResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    AdminCategoryListResp::get_msg_for_reflect,
                    AdminCategoryListResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total",
                    AdminCategoryListResp::get_total_for_reflect,
                    AdminCategoryListResp::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdminCategoryListResp_categoryBase>>(
                    "rows",
                    AdminCategoryListResp::get_rows_for_reflect,
                    AdminCategoryListResp::mut_rows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminCategoryListResp>(
                    "AdminCategoryListResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminCategoryListResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_total();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminCategoryListResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminCategoryListResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminCategoryListResp_blogBase {
    // message fields
    pub title: ::std::string::String,
    pub summary: ::std::string::String,
    pub headerImg: ::std::string::String,
    pub comment: ::std::string::String,
    pub weight: u32,
    pub support: bool,
    pub createTime: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminCategoryListResp_blogBase {}

impl AdminCategoryListResp_blogBase {
    pub fn new() -> AdminCategoryListResp_blogBase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminCategoryListResp_blogBase {
        static mut instance: ::protobuf::lazy::Lazy<AdminCategoryListResp_blogBase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminCategoryListResp_blogBase,
        };
        unsafe {
            instance.get(AdminCategoryListResp_blogBase::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string summary = 2;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.summary, ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    fn get_summary_for_reflect(&self) -> &::std::string::String {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // string headerImg = 3;

    pub fn clear_headerImg(&mut self) {
        self.headerImg.clear();
    }

    // Param is passed by value, moved
    pub fn set_headerImg(&mut self, v: ::std::string::String) {
        self.headerImg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headerImg(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // Take field
    pub fn take_headerImg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.headerImg, ::std::string::String::new())
    }

    pub fn get_headerImg(&self) -> &str {
        &self.headerImg
    }

    fn get_headerImg_for_reflect(&self) -> &::std::string::String {
        &self.headerImg
    }

    fn mut_headerImg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // string comment = 4;

    pub fn clear_comment(&mut self) {
        self.comment.clear();
    }

    // Param is passed by value, moved
    pub fn set_comment(&mut self, v: ::std::string::String) {
        self.comment = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_comment(&mut self) -> &mut ::std::string::String {
        &mut self.comment
    }

    // Take field
    pub fn take_comment(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.comment, ::std::string::String::new())
    }

    pub fn get_comment(&self) -> &str {
        &self.comment
    }

    fn get_comment_for_reflect(&self) -> &::std::string::String {
        &self.comment
    }

    fn mut_comment_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.comment
    }

    // uint32 weight = 5;

    pub fn clear_weight(&mut self) {
        self.weight = 0;
    }

    // Param is passed by value, moved
    pub fn set_weight(&mut self, v: u32) {
        self.weight = v;
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    fn get_weight_for_reflect(&self) -> &u32 {
        &self.weight
    }

    fn mut_weight_for_reflect(&mut self) -> &mut u32 {
        &mut self.weight
    }

    // bool support = 6;

    pub fn clear_support(&mut self) {
        self.support = false;
    }

    // Param is passed by value, moved
    pub fn set_support(&mut self, v: bool) {
        self.support = v;
    }

    pub fn get_support(&self) -> bool {
        self.support
    }

    fn get_support_for_reflect(&self) -> &bool {
        &self.support
    }

    fn mut_support_for_reflect(&mut self) -> &mut bool {
        &mut self.support
    }

    // string createTime = 7;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createTime, ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        &self.createTime
    }

    fn get_createTime_for_reflect(&self) -> &::std::string::String {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }
}

impl ::protobuf::Message for AdminCategoryListResp_blogBase {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.summary)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.headerImg)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.comment)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.weight = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.support = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createTime)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.summary.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.summary);
        }
        if !self.headerImg.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.headerImg);
        }
        if !self.comment.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.comment);
        }
        if self.weight != 0 {
            my_size += ::protobuf::rt::value_size(5, self.weight, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.support != false {
            my_size += 2;
        }
        if !self.createTime.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.createTime);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.summary.is_empty() {
            os.write_string(2, &self.summary)?;
        }
        if !self.headerImg.is_empty() {
            os.write_string(3, &self.headerImg)?;
        }
        if !self.comment.is_empty() {
            os.write_string(4, &self.comment)?;
        }
        if self.weight != 0 {
            os.write_uint32(5, self.weight)?;
        }
        if self.support != false {
            os.write_bool(6, self.support)?;
        }
        if !self.createTime.is_empty() {
            os.write_string(7, &self.createTime)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminCategoryListResp_blogBase {
    fn new() -> AdminCategoryListResp_blogBase {
        AdminCategoryListResp_blogBase::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminCategoryListResp_blogBase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminCategoryListResp_blogBase::get_title_for_reflect,
                    AdminCategoryListResp_blogBase::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    AdminCategoryListResp_blogBase::get_summary_for_reflect,
                    AdminCategoryListResp_blogBase::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headerImg",
                    AdminCategoryListResp_blogBase::get_headerImg_for_reflect,
                    AdminCategoryListResp_blogBase::mut_headerImg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "comment",
                    AdminCategoryListResp_blogBase::get_comment_for_reflect,
                    AdminCategoryListResp_blogBase::mut_comment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weight",
                    AdminCategoryListResp_blogBase::get_weight_for_reflect,
                    AdminCategoryListResp_blogBase::mut_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "support",
                    AdminCategoryListResp_blogBase::get_support_for_reflect,
                    AdminCategoryListResp_blogBase::mut_support_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    AdminCategoryListResp_blogBase::get_createTime_for_reflect,
                    AdminCategoryListResp_blogBase::mut_createTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminCategoryListResp_blogBase>(
                    "AdminCategoryListResp_blogBase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminCategoryListResp_blogBase {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_summary();
        self.clear_headerImg();
        self.clear_comment();
        self.clear_weight();
        self.clear_support();
        self.clear_createTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminCategoryListResp_blogBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminCategoryListResp_blogBase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminCategoryListResp_categoryBase {
    // message fields
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    pub createTime: ::std::string::String,
    pub support: bool,
    pub blogList: ::protobuf::RepeatedField<AdminCategoryListResp_blogBase>,
    pub id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminCategoryListResp_categoryBase {}

impl AdminCategoryListResp_categoryBase {
    pub fn new() -> AdminCategoryListResp_categoryBase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminCategoryListResp_categoryBase {
        static mut instance: ::protobuf::lazy::Lazy<AdminCategoryListResp_categoryBase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminCategoryListResp_categoryBase,
        };
        unsafe {
            instance.get(AdminCategoryListResp_categoryBase::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // string createTime = 3;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createTime, ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        &self.createTime
    }

    fn get_createTime_for_reflect(&self) -> &::std::string::String {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // bool support = 4;

    pub fn clear_support(&mut self) {
        self.support = false;
    }

    // Param is passed by value, moved
    pub fn set_support(&mut self, v: bool) {
        self.support = v;
    }

    pub fn get_support(&self) -> bool {
        self.support
    }

    fn get_support_for_reflect(&self) -> &bool {
        &self.support
    }

    fn mut_support_for_reflect(&mut self) -> &mut bool {
        &mut self.support
    }

    // repeated .proto.AdminCategoryListResp.blogBase blogList = 5;

    pub fn clear_blogList(&mut self) {
        self.blogList.clear();
    }

    // Param is passed by value, moved
    pub fn set_blogList(&mut self, v: ::protobuf::RepeatedField<AdminCategoryListResp_blogBase>) {
        self.blogList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blogList(&mut self) -> &mut ::protobuf::RepeatedField<AdminCategoryListResp_blogBase> {
        &mut self.blogList
    }

    // Take field
    pub fn take_blogList(&mut self) -> ::protobuf::RepeatedField<AdminCategoryListResp_blogBase> {
        ::std::mem::replace(&mut self.blogList, ::protobuf::RepeatedField::new())
    }

    pub fn get_blogList(&self) -> &[AdminCategoryListResp_blogBase] {
        &self.blogList
    }

    fn get_blogList_for_reflect(&self) -> &::protobuf::RepeatedField<AdminCategoryListResp_blogBase> {
        &self.blogList
    }

    fn mut_blogList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AdminCategoryListResp_blogBase> {
        &mut self.blogList
    }

    // uint32 id = 6;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }
}

impl ::protobuf::Message for AdminCategoryListResp_categoryBase {
    fn is_initialized(&self) -> bool {
        for v in &self.blogList {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createTime)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.support = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blogList)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if !self.createTime.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.createTime);
        }
        if self.support != false {
            my_size += 2;
        }
        for value in &self.blogList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(6, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if !self.createTime.is_empty() {
            os.write_string(3, &self.createTime)?;
        }
        if self.support != false {
            os.write_bool(4, self.support)?;
        }
        for v in &self.blogList {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.id != 0 {
            os.write_uint32(6, self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminCategoryListResp_categoryBase {
    fn new() -> AdminCategoryListResp_categoryBase {
        AdminCategoryListResp_categoryBase::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminCategoryListResp_categoryBase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminCategoryListResp_categoryBase::get_title_for_reflect,
                    AdminCategoryListResp_categoryBase::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    AdminCategoryListResp_categoryBase::get_description_for_reflect,
                    AdminCategoryListResp_categoryBase::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    AdminCategoryListResp_categoryBase::get_createTime_for_reflect,
                    AdminCategoryListResp_categoryBase::mut_createTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "support",
                    AdminCategoryListResp_categoryBase::get_support_for_reflect,
                    AdminCategoryListResp_categoryBase::mut_support_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdminCategoryListResp_blogBase>>(
                    "blogList",
                    AdminCategoryListResp_categoryBase::get_blogList_for_reflect,
                    AdminCategoryListResp_categoryBase::mut_blogList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    AdminCategoryListResp_categoryBase::get_id_for_reflect,
                    AdminCategoryListResp_categoryBase::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminCategoryListResp_categoryBase>(
                    "AdminCategoryListResp_categoryBase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminCategoryListResp_categoryBase {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_description();
        self.clear_createTime();
        self.clear_support();
        self.clear_blogList();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminCategoryListResp_categoryBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminCategoryListResp_categoryBase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminArticleAddRequest {
    // message fields
    pub title: ::std::string::String,
    pub summary: ::std::string::String,
    pub categoryId: u32,
    pub support: bool,
    pub comment: bool,
    pub headerImgType: u32,
    pub headerImg: ::std::string::String,
    pub weight: u32,
    pub tagTitleList: ::protobuf::RepeatedField<::std::string::String>,
    pub content: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminArticleAddRequest {}

impl AdminArticleAddRequest {
    pub fn new() -> AdminArticleAddRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminArticleAddRequest {
        static mut instance: ::protobuf::lazy::Lazy<AdminArticleAddRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminArticleAddRequest,
        };
        unsafe {
            instance.get(AdminArticleAddRequest::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string summary = 2;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.summary, ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    fn get_summary_for_reflect(&self) -> &::std::string::String {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // uint32 categoryId = 3;

    pub fn clear_categoryId(&mut self) {
        self.categoryId = 0;
    }

    // Param is passed by value, moved
    pub fn set_categoryId(&mut self, v: u32) {
        self.categoryId = v;
    }

    pub fn get_categoryId(&self) -> u32 {
        self.categoryId
    }

    fn get_categoryId_for_reflect(&self) -> &u32 {
        &self.categoryId
    }

    fn mut_categoryId_for_reflect(&mut self) -> &mut u32 {
        &mut self.categoryId
    }

    // bool support = 4;

    pub fn clear_support(&mut self) {
        self.support = false;
    }

    // Param is passed by value, moved
    pub fn set_support(&mut self, v: bool) {
        self.support = v;
    }

    pub fn get_support(&self) -> bool {
        self.support
    }

    fn get_support_for_reflect(&self) -> &bool {
        &self.support
    }

    fn mut_support_for_reflect(&mut self) -> &mut bool {
        &mut self.support
    }

    // bool comment = 5;

    pub fn clear_comment(&mut self) {
        self.comment = false;
    }

    // Param is passed by value, moved
    pub fn set_comment(&mut self, v: bool) {
        self.comment = v;
    }

    pub fn get_comment(&self) -> bool {
        self.comment
    }

    fn get_comment_for_reflect(&self) -> &bool {
        &self.comment
    }

    fn mut_comment_for_reflect(&mut self) -> &mut bool {
        &mut self.comment
    }

    // uint32 headerImgType = 6;

    pub fn clear_headerImgType(&mut self) {
        self.headerImgType = 0;
    }

    // Param is passed by value, moved
    pub fn set_headerImgType(&mut self, v: u32) {
        self.headerImgType = v;
    }

    pub fn get_headerImgType(&self) -> u32 {
        self.headerImgType
    }

    fn get_headerImgType_for_reflect(&self) -> &u32 {
        &self.headerImgType
    }

    fn mut_headerImgType_for_reflect(&mut self) -> &mut u32 {
        &mut self.headerImgType
    }

    // string headerImg = 7;

    pub fn clear_headerImg(&mut self) {
        self.headerImg.clear();
    }

    // Param is passed by value, moved
    pub fn set_headerImg(&mut self, v: ::std::string::String) {
        self.headerImg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headerImg(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // Take field
    pub fn take_headerImg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.headerImg, ::std::string::String::new())
    }

    pub fn get_headerImg(&self) -> &str {
        &self.headerImg
    }

    fn get_headerImg_for_reflect(&self) -> &::std::string::String {
        &self.headerImg
    }

    fn mut_headerImg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // uint32 weight = 8;

    pub fn clear_weight(&mut self) {
        self.weight = 0;
    }

    // Param is passed by value, moved
    pub fn set_weight(&mut self, v: u32) {
        self.weight = v;
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    fn get_weight_for_reflect(&self) -> &u32 {
        &self.weight
    }

    fn mut_weight_for_reflect(&mut self) -> &mut u32 {
        &mut self.weight
    }

    // repeated string tagTitleList = 9;

    pub fn clear_tagTitleList(&mut self) {
        self.tagTitleList.clear();
    }

    // Param is passed by value, moved
    pub fn set_tagTitleList(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tagTitleList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tagTitleList(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tagTitleList
    }

    // Take field
    pub fn take_tagTitleList(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tagTitleList, ::protobuf::RepeatedField::new())
    }

    pub fn get_tagTitleList(&self) -> &[::std::string::String] {
        &self.tagTitleList
    }

    fn get_tagTitleList_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tagTitleList
    }

    fn mut_tagTitleList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tagTitleList
    }

    // string content = 10;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }
}

impl ::protobuf::Message for AdminArticleAddRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.summary)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.categoryId = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.support = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.comment = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.headerImgType = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.headerImg)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.weight = tmp;
                },
                9 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tagTitleList)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.summary.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.summary);
        }
        if self.categoryId != 0 {
            my_size += ::protobuf::rt::value_size(3, self.categoryId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.support != false {
            my_size += 2;
        }
        if self.comment != false {
            my_size += 2;
        }
        if self.headerImgType != 0 {
            my_size += ::protobuf::rt::value_size(6, self.headerImgType, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.headerImg.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.headerImg);
        }
        if self.weight != 0 {
            my_size += ::protobuf::rt::value_size(8, self.weight, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tagTitleList {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.summary.is_empty() {
            os.write_string(2, &self.summary)?;
        }
        if self.categoryId != 0 {
            os.write_uint32(3, self.categoryId)?;
        }
        if self.support != false {
            os.write_bool(4, self.support)?;
        }
        if self.comment != false {
            os.write_bool(5, self.comment)?;
        }
        if self.headerImgType != 0 {
            os.write_uint32(6, self.headerImgType)?;
        }
        if !self.headerImg.is_empty() {
            os.write_string(7, &self.headerImg)?;
        }
        if self.weight != 0 {
            os.write_uint32(8, self.weight)?;
        }
        for v in &self.tagTitleList {
            os.write_string(9, &v)?;
        };
        if !self.content.is_empty() {
            os.write_string(10, &self.content)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminArticleAddRequest {
    fn new() -> AdminArticleAddRequest {
        AdminArticleAddRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminArticleAddRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminArticleAddRequest::get_title_for_reflect,
                    AdminArticleAddRequest::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    AdminArticleAddRequest::get_summary_for_reflect,
                    AdminArticleAddRequest::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "categoryId",
                    AdminArticleAddRequest::get_categoryId_for_reflect,
                    AdminArticleAddRequest::mut_categoryId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "support",
                    AdminArticleAddRequest::get_support_for_reflect,
                    AdminArticleAddRequest::mut_support_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "comment",
                    AdminArticleAddRequest::get_comment_for_reflect,
                    AdminArticleAddRequest::mut_comment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "headerImgType",
                    AdminArticleAddRequest::get_headerImgType_for_reflect,
                    AdminArticleAddRequest::mut_headerImgType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headerImg",
                    AdminArticleAddRequest::get_headerImg_for_reflect,
                    AdminArticleAddRequest::mut_headerImg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weight",
                    AdminArticleAddRequest::get_weight_for_reflect,
                    AdminArticleAddRequest::mut_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tagTitleList",
                    AdminArticleAddRequest::get_tagTitleList_for_reflect,
                    AdminArticleAddRequest::mut_tagTitleList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    AdminArticleAddRequest::get_content_for_reflect,
                    AdminArticleAddRequest::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminArticleAddRequest>(
                    "AdminArticleAddRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminArticleAddRequest {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_summary();
        self.clear_categoryId();
        self.clear_support();
        self.clear_comment();
        self.clear_headerImgType();
        self.clear_headerImg();
        self.clear_weight();
        self.clear_tagTitleList();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminArticleAddRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminArticleAddRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminArticleListResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub total: u32,
    pub rows: ::protobuf::RepeatedField<AdminArticleListResp_adminArticleListBase>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminArticleListResp {}

impl AdminArticleListResp {
    pub fn new() -> AdminArticleListResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminArticleListResp {
        static mut instance: ::protobuf::lazy::Lazy<AdminArticleListResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminArticleListResp,
        };
        unsafe {
            instance.get(AdminArticleListResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // uint32 total = 3;

    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u32) {
        self.total = v;
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    fn get_total_for_reflect(&self) -> &u32 {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut u32 {
        &mut self.total
    }

    // repeated .proto.AdminArticleListResp.adminArticleListBase rows = 4;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<AdminArticleListResp_adminArticleListBase>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<AdminArticleListResp_adminArticleListBase> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<AdminArticleListResp_adminArticleListBase> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[AdminArticleListResp_adminArticleListBase] {
        &self.rows
    }

    fn get_rows_for_reflect(&self) -> &::protobuf::RepeatedField<AdminArticleListResp_adminArticleListBase> {
        &self.rows
    }

    fn mut_rows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AdminArticleListResp_adminArticleListBase> {
        &mut self.rows
    }
}

impl ::protobuf::Message for AdminArticleListResp {
    fn is_initialized(&self) -> bool {
        for v in &self.rows {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.total != 0 {
            os.write_uint32(3, self.total)?;
        }
        for v in &self.rows {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminArticleListResp {
    fn new() -> AdminArticleListResp {
        AdminArticleListResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminArticleListResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    AdminArticleListResp::get_code_for_reflect,
                    AdminArticleListResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    AdminArticleListResp::get_msg_for_reflect,
                    AdminArticleListResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total",
                    AdminArticleListResp::get_total_for_reflect,
                    AdminArticleListResp::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdminArticleListResp_adminArticleListBase>>(
                    "rows",
                    AdminArticleListResp::get_rows_for_reflect,
                    AdminArticleListResp::mut_rows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminArticleListResp>(
                    "AdminArticleListResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminArticleListResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_total();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminArticleListResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminArticleListResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminArticleListResp_adminArticleListCategory {
    // message fields
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminArticleListResp_adminArticleListCategory {}

impl AdminArticleListResp_adminArticleListCategory {
    pub fn new() -> AdminArticleListResp_adminArticleListCategory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminArticleListResp_adminArticleListCategory {
        static mut instance: ::protobuf::lazy::Lazy<AdminArticleListResp_adminArticleListCategory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminArticleListResp_adminArticleListCategory,
        };
        unsafe {
            instance.get(AdminArticleListResp_adminArticleListCategory::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }
}

impl ::protobuf::Message for AdminArticleListResp_adminArticleListCategory {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminArticleListResp_adminArticleListCategory {
    fn new() -> AdminArticleListResp_adminArticleListCategory {
        AdminArticleListResp_adminArticleListCategory::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminArticleListResp_adminArticleListCategory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminArticleListResp_adminArticleListCategory::get_title_for_reflect,
                    AdminArticleListResp_adminArticleListCategory::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    AdminArticleListResp_adminArticleListCategory::get_description_for_reflect,
                    AdminArticleListResp_adminArticleListCategory::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminArticleListResp_adminArticleListCategory>(
                    "AdminArticleListResp_adminArticleListCategory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminArticleListResp_adminArticleListCategory {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminArticleListResp_adminArticleListCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminArticleListResp_adminArticleListCategory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminArticleListResp_adminArticleListBase {
    // message fields
    pub title: ::std::string::String,
    pub summary: ::std::string::String,
    pub headerImg: ::std::string::String,
    pub comment: bool,
    pub weight: u32,
    pub support: bool,
    pub createTime: ::std::string::String,
    pub id: u32,
    pub status: bool,
    pub category: ::protobuf::SingularPtrField<AdminArticleListResp_adminArticleListCategory>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminArticleListResp_adminArticleListBase {}

impl AdminArticleListResp_adminArticleListBase {
    pub fn new() -> AdminArticleListResp_adminArticleListBase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminArticleListResp_adminArticleListBase {
        static mut instance: ::protobuf::lazy::Lazy<AdminArticleListResp_adminArticleListBase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminArticleListResp_adminArticleListBase,
        };
        unsafe {
            instance.get(AdminArticleListResp_adminArticleListBase::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string summary = 2;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.summary, ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    fn get_summary_for_reflect(&self) -> &::std::string::String {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // string headerImg = 3;

    pub fn clear_headerImg(&mut self) {
        self.headerImg.clear();
    }

    // Param is passed by value, moved
    pub fn set_headerImg(&mut self, v: ::std::string::String) {
        self.headerImg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headerImg(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // Take field
    pub fn take_headerImg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.headerImg, ::std::string::String::new())
    }

    pub fn get_headerImg(&self) -> &str {
        &self.headerImg
    }

    fn get_headerImg_for_reflect(&self) -> &::std::string::String {
        &self.headerImg
    }

    fn mut_headerImg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // bool comment = 4;

    pub fn clear_comment(&mut self) {
        self.comment = false;
    }

    // Param is passed by value, moved
    pub fn set_comment(&mut self, v: bool) {
        self.comment = v;
    }

    pub fn get_comment(&self) -> bool {
        self.comment
    }

    fn get_comment_for_reflect(&self) -> &bool {
        &self.comment
    }

    fn mut_comment_for_reflect(&mut self) -> &mut bool {
        &mut self.comment
    }

    // uint32 weight = 5;

    pub fn clear_weight(&mut self) {
        self.weight = 0;
    }

    // Param is passed by value, moved
    pub fn set_weight(&mut self, v: u32) {
        self.weight = v;
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    fn get_weight_for_reflect(&self) -> &u32 {
        &self.weight
    }

    fn mut_weight_for_reflect(&mut self) -> &mut u32 {
        &mut self.weight
    }

    // bool support = 6;

    pub fn clear_support(&mut self) {
        self.support = false;
    }

    // Param is passed by value, moved
    pub fn set_support(&mut self, v: bool) {
        self.support = v;
    }

    pub fn get_support(&self) -> bool {
        self.support
    }

    fn get_support_for_reflect(&self) -> &bool {
        &self.support
    }

    fn mut_support_for_reflect(&mut self) -> &mut bool {
        &mut self.support
    }

    // string createTime = 7;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createTime, ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        &self.createTime
    }

    fn get_createTime_for_reflect(&self) -> &::std::string::String {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // uint32 id = 8;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // bool status = 9;

    pub fn clear_status(&mut self) {
        self.status = false;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: bool) {
        self.status = v;
    }

    pub fn get_status(&self) -> bool {
        self.status
    }

    fn get_status_for_reflect(&self) -> &bool {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut bool {
        &mut self.status
    }

    // .proto.AdminArticleListResp.adminArticleListCategory category = 10;

    pub fn clear_category(&mut self) {
        self.category.clear();
    }

    pub fn has_category(&self) -> bool {
        self.category.is_some()
    }

    // Param is passed by value, moved
    pub fn set_category(&mut self, v: AdminArticleListResp_adminArticleListCategory) {
        self.category = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_category(&mut self) -> &mut AdminArticleListResp_adminArticleListCategory {
        if self.category.is_none() {
            self.category.set_default();
        }
        self.category.as_mut().unwrap()
    }

    // Take field
    pub fn take_category(&mut self) -> AdminArticleListResp_adminArticleListCategory {
        self.category.take().unwrap_or_else(|| AdminArticleListResp_adminArticleListCategory::new())
    }

    pub fn get_category(&self) -> &AdminArticleListResp_adminArticleListCategory {
        self.category.as_ref().unwrap_or_else(|| AdminArticleListResp_adminArticleListCategory::default_instance())
    }

    fn get_category_for_reflect(&self) -> &::protobuf::SingularPtrField<AdminArticleListResp_adminArticleListCategory> {
        &self.category
    }

    fn mut_category_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AdminArticleListResp_adminArticleListCategory> {
        &mut self.category
    }
}

impl ::protobuf::Message for AdminArticleListResp_adminArticleListBase {
    fn is_initialized(&self) -> bool {
        for v in &self.category {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.summary)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.headerImg)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.comment = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.weight = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.support = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createTime)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.status = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.category)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.summary.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.summary);
        }
        if !self.headerImg.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.headerImg);
        }
        if self.comment != false {
            my_size += 2;
        }
        if self.weight != 0 {
            my_size += ::protobuf::rt::value_size(5, self.weight, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.support != false {
            my_size += 2;
        }
        if !self.createTime.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.createTime);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(8, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.status != false {
            my_size += 2;
        }
        if let Some(ref v) = self.category.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.summary.is_empty() {
            os.write_string(2, &self.summary)?;
        }
        if !self.headerImg.is_empty() {
            os.write_string(3, &self.headerImg)?;
        }
        if self.comment != false {
            os.write_bool(4, self.comment)?;
        }
        if self.weight != 0 {
            os.write_uint32(5, self.weight)?;
        }
        if self.support != false {
            os.write_bool(6, self.support)?;
        }
        if !self.createTime.is_empty() {
            os.write_string(7, &self.createTime)?;
        }
        if self.id != 0 {
            os.write_uint32(8, self.id)?;
        }
        if self.status != false {
            os.write_bool(9, self.status)?;
        }
        if let Some(ref v) = self.category.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminArticleListResp_adminArticleListBase {
    fn new() -> AdminArticleListResp_adminArticleListBase {
        AdminArticleListResp_adminArticleListBase::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminArticleListResp_adminArticleListBase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminArticleListResp_adminArticleListBase::get_title_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    AdminArticleListResp_adminArticleListBase::get_summary_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headerImg",
                    AdminArticleListResp_adminArticleListBase::get_headerImg_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_headerImg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "comment",
                    AdminArticleListResp_adminArticleListBase::get_comment_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_comment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weight",
                    AdminArticleListResp_adminArticleListBase::get_weight_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "support",
                    AdminArticleListResp_adminArticleListBase::get_support_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_support_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    AdminArticleListResp_adminArticleListBase::get_createTime_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_createTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    AdminArticleListResp_adminArticleListBase::get_id_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "status",
                    AdminArticleListResp_adminArticleListBase::get_status_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdminArticleListResp_adminArticleListCategory>>(
                    "category",
                    AdminArticleListResp_adminArticleListBase::get_category_for_reflect,
                    AdminArticleListResp_adminArticleListBase::mut_category_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminArticleListResp_adminArticleListBase>(
                    "AdminArticleListResp_adminArticleListBase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminArticleListResp_adminArticleListBase {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_summary();
        self.clear_headerImg();
        self.clear_comment();
        self.clear_weight();
        self.clear_support();
        self.clear_createTime();
        self.clear_id();
        self.clear_status();
        self.clear_category();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminArticleListResp_adminArticleListBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminArticleListResp_adminArticleListBase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminArticleOneResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub title: ::std::string::String,
    pub summary: ::std::string::String,
    pub categoryId: u32,
    pub support: bool,
    pub comment: bool,
    pub headerImgType: u32,
    pub headerImg: ::std::string::String,
    pub weight: u32,
    pub tagTitleList: ::protobuf::RepeatedField<::std::string::String>,
    pub content: ::std::string::String,
    pub id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminArticleOneResp {}

impl AdminArticleOneResp {
    pub fn new() -> AdminArticleOneResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminArticleOneResp {
        static mut instance: ::protobuf::lazy::Lazy<AdminArticleOneResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminArticleOneResp,
        };
        unsafe {
            instance.get(AdminArticleOneResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string title = 3;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string summary = 4;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.summary, ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    fn get_summary_for_reflect(&self) -> &::std::string::String {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // uint32 categoryId = 5;

    pub fn clear_categoryId(&mut self) {
        self.categoryId = 0;
    }

    // Param is passed by value, moved
    pub fn set_categoryId(&mut self, v: u32) {
        self.categoryId = v;
    }

    pub fn get_categoryId(&self) -> u32 {
        self.categoryId
    }

    fn get_categoryId_for_reflect(&self) -> &u32 {
        &self.categoryId
    }

    fn mut_categoryId_for_reflect(&mut self) -> &mut u32 {
        &mut self.categoryId
    }

    // bool support = 6;

    pub fn clear_support(&mut self) {
        self.support = false;
    }

    // Param is passed by value, moved
    pub fn set_support(&mut self, v: bool) {
        self.support = v;
    }

    pub fn get_support(&self) -> bool {
        self.support
    }

    fn get_support_for_reflect(&self) -> &bool {
        &self.support
    }

    fn mut_support_for_reflect(&mut self) -> &mut bool {
        &mut self.support
    }

    // bool comment = 7;

    pub fn clear_comment(&mut self) {
        self.comment = false;
    }

    // Param is passed by value, moved
    pub fn set_comment(&mut self, v: bool) {
        self.comment = v;
    }

    pub fn get_comment(&self) -> bool {
        self.comment
    }

    fn get_comment_for_reflect(&self) -> &bool {
        &self.comment
    }

    fn mut_comment_for_reflect(&mut self) -> &mut bool {
        &mut self.comment
    }

    // uint32 headerImgType = 8;

    pub fn clear_headerImgType(&mut self) {
        self.headerImgType = 0;
    }

    // Param is passed by value, moved
    pub fn set_headerImgType(&mut self, v: u32) {
        self.headerImgType = v;
    }

    pub fn get_headerImgType(&self) -> u32 {
        self.headerImgType
    }

    fn get_headerImgType_for_reflect(&self) -> &u32 {
        &self.headerImgType
    }

    fn mut_headerImgType_for_reflect(&mut self) -> &mut u32 {
        &mut self.headerImgType
    }

    // string headerImg = 9;

    pub fn clear_headerImg(&mut self) {
        self.headerImg.clear();
    }

    // Param is passed by value, moved
    pub fn set_headerImg(&mut self, v: ::std::string::String) {
        self.headerImg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headerImg(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // Take field
    pub fn take_headerImg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.headerImg, ::std::string::String::new())
    }

    pub fn get_headerImg(&self) -> &str {
        &self.headerImg
    }

    fn get_headerImg_for_reflect(&self) -> &::std::string::String {
        &self.headerImg
    }

    fn mut_headerImg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // uint32 weight = 10;

    pub fn clear_weight(&mut self) {
        self.weight = 0;
    }

    // Param is passed by value, moved
    pub fn set_weight(&mut self, v: u32) {
        self.weight = v;
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    fn get_weight_for_reflect(&self) -> &u32 {
        &self.weight
    }

    fn mut_weight_for_reflect(&mut self) -> &mut u32 {
        &mut self.weight
    }

    // repeated string tagTitleList = 11;

    pub fn clear_tagTitleList(&mut self) {
        self.tagTitleList.clear();
    }

    // Param is passed by value, moved
    pub fn set_tagTitleList(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tagTitleList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tagTitleList(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tagTitleList
    }

    // Take field
    pub fn take_tagTitleList(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tagTitleList, ::protobuf::RepeatedField::new())
    }

    pub fn get_tagTitleList(&self) -> &[::std::string::String] {
        &self.tagTitleList
    }

    fn get_tagTitleList_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tagTitleList
    }

    fn mut_tagTitleList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tagTitleList
    }

    // string content = 12;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // uint32 id = 13;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }
}

impl ::protobuf::Message for AdminArticleOneResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.summary)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.categoryId = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.support = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.comment = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.headerImgType = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.headerImg)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.weight = tmp;
                },
                11 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tagTitleList)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.title);
        }
        if !self.summary.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.summary);
        }
        if self.categoryId != 0 {
            my_size += ::protobuf::rt::value_size(5, self.categoryId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.support != false {
            my_size += 2;
        }
        if self.comment != false {
            my_size += 2;
        }
        if self.headerImgType != 0 {
            my_size += ::protobuf::rt::value_size(8, self.headerImgType, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.headerImg.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.headerImg);
        }
        if self.weight != 0 {
            my_size += ::protobuf::rt::value_size(10, self.weight, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tagTitleList {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.content);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(13, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.title.is_empty() {
            os.write_string(3, &self.title)?;
        }
        if !self.summary.is_empty() {
            os.write_string(4, &self.summary)?;
        }
        if self.categoryId != 0 {
            os.write_uint32(5, self.categoryId)?;
        }
        if self.support != false {
            os.write_bool(6, self.support)?;
        }
        if self.comment != false {
            os.write_bool(7, self.comment)?;
        }
        if self.headerImgType != 0 {
            os.write_uint32(8, self.headerImgType)?;
        }
        if !self.headerImg.is_empty() {
            os.write_string(9, &self.headerImg)?;
        }
        if self.weight != 0 {
            os.write_uint32(10, self.weight)?;
        }
        for v in &self.tagTitleList {
            os.write_string(11, &v)?;
        };
        if !self.content.is_empty() {
            os.write_string(12, &self.content)?;
        }
        if self.id != 0 {
            os.write_uint32(13, self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminArticleOneResp {
    fn new() -> AdminArticleOneResp {
        AdminArticleOneResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminArticleOneResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    AdminArticleOneResp::get_code_for_reflect,
                    AdminArticleOneResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    AdminArticleOneResp::get_msg_for_reflect,
                    AdminArticleOneResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminArticleOneResp::get_title_for_reflect,
                    AdminArticleOneResp::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    AdminArticleOneResp::get_summary_for_reflect,
                    AdminArticleOneResp::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "categoryId",
                    AdminArticleOneResp::get_categoryId_for_reflect,
                    AdminArticleOneResp::mut_categoryId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "support",
                    AdminArticleOneResp::get_support_for_reflect,
                    AdminArticleOneResp::mut_support_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "comment",
                    AdminArticleOneResp::get_comment_for_reflect,
                    AdminArticleOneResp::mut_comment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "headerImgType",
                    AdminArticleOneResp::get_headerImgType_for_reflect,
                    AdminArticleOneResp::mut_headerImgType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headerImg",
                    AdminArticleOneResp::get_headerImg_for_reflect,
                    AdminArticleOneResp::mut_headerImg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weight",
                    AdminArticleOneResp::get_weight_for_reflect,
                    AdminArticleOneResp::mut_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tagTitleList",
                    AdminArticleOneResp::get_tagTitleList_for_reflect,
                    AdminArticleOneResp::mut_tagTitleList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    AdminArticleOneResp::get_content_for_reflect,
                    AdminArticleOneResp::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    AdminArticleOneResp::get_id_for_reflect,
                    AdminArticleOneResp::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminArticleOneResp>(
                    "AdminArticleOneResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminArticleOneResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_title();
        self.clear_summary();
        self.clear_categoryId();
        self.clear_support();
        self.clear_comment();
        self.clear_headerImgType();
        self.clear_headerImg();
        self.clear_weight();
        self.clear_tagTitleList();
        self.clear_content();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminArticleOneResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminArticleOneResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tags {
    // message fields
    pub _id: u32,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tags {}

impl Tags {
    pub fn new() -> Tags {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tags {
        static mut instance: ::protobuf::lazy::Lazy<Tags> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tags,
        };
        unsafe {
            instance.get(Tags::new)
        }
    }

    // uint32 _id = 1;

    pub fn clear__id(&mut self) {
        self._id = 0;
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u32) {
        self._id = v;
    }

    pub fn get__id(&self) -> u32 {
        self._id
    }

    fn get__id_for_reflect(&self) -> &u32 {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut u32 {
        &mut self._id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for Tags {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self._id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if self._id != 0 {
            my_size += ::protobuf::rt::value_size(1, self._id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self._id != 0 {
            os.write_uint32(1, self._id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tags {
    fn new() -> Tags {
        Tags::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tags>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "_id",
                    Tags::get__id_for_reflect,
                    Tags::mut__id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Tags::get_name_for_reflect,
                    Tags::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tags>(
                    "Tags",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tags {
    fn clear(&mut self) {
        self.clear__id();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tags {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListByClassResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub classList: ::protobuf::RepeatedField<ListByClassResp_ClassList>,
    pub articleList: ::protobuf::RepeatedField<ListByClassResp_ArticleList>,
    pub tags: ::protobuf::RepeatedField<Tags>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListByClassResp {}

impl ListByClassResp {
    pub fn new() -> ListByClassResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListByClassResp {
        static mut instance: ::protobuf::lazy::Lazy<ListByClassResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListByClassResp,
        };
        unsafe {
            instance.get(ListByClassResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated .proto.ListByClassResp.ClassList classList = 3;

    pub fn clear_classList(&mut self) {
        self.classList.clear();
    }

    // Param is passed by value, moved
    pub fn set_classList(&mut self, v: ::protobuf::RepeatedField<ListByClassResp_ClassList>) {
        self.classList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classList(&mut self) -> &mut ::protobuf::RepeatedField<ListByClassResp_ClassList> {
        &mut self.classList
    }

    // Take field
    pub fn take_classList(&mut self) -> ::protobuf::RepeatedField<ListByClassResp_ClassList> {
        ::std::mem::replace(&mut self.classList, ::protobuf::RepeatedField::new())
    }

    pub fn get_classList(&self) -> &[ListByClassResp_ClassList] {
        &self.classList
    }

    fn get_classList_for_reflect(&self) -> &::protobuf::RepeatedField<ListByClassResp_ClassList> {
        &self.classList
    }

    fn mut_classList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListByClassResp_ClassList> {
        &mut self.classList
    }

    // repeated .proto.ListByClassResp.ArticleList articleList = 4;

    pub fn clear_articleList(&mut self) {
        self.articleList.clear();
    }

    // Param is passed by value, moved
    pub fn set_articleList(&mut self, v: ::protobuf::RepeatedField<ListByClassResp_ArticleList>) {
        self.articleList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_articleList(&mut self) -> &mut ::protobuf::RepeatedField<ListByClassResp_ArticleList> {
        &mut self.articleList
    }

    // Take field
    pub fn take_articleList(&mut self) -> ::protobuf::RepeatedField<ListByClassResp_ArticleList> {
        ::std::mem::replace(&mut self.articleList, ::protobuf::RepeatedField::new())
    }

    pub fn get_articleList(&self) -> &[ListByClassResp_ArticleList] {
        &self.articleList
    }

    fn get_articleList_for_reflect(&self) -> &::protobuf::RepeatedField<ListByClassResp_ArticleList> {
        &self.articleList
    }

    fn mut_articleList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListByClassResp_ArticleList> {
        &mut self.articleList
    }

    // repeated .proto.Tags tags = 5;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<Tags>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<Tags> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<Tags> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[Tags] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<Tags> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Tags> {
        &mut self.tags
    }
}

impl ::protobuf::Message for ListByClassResp {
    fn is_initialized(&self) -> bool {
        for v in &self.classList {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.articleList {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tags {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classList)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.articleList)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.classList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.articleList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.classList {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.articleList {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tags {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListByClassResp {
    fn new() -> ListByClassResp {
        ListByClassResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListByClassResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ListByClassResp::get_code_for_reflect,
                    ListByClassResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    ListByClassResp::get_msg_for_reflect,
                    ListByClassResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListByClassResp_ClassList>>(
                    "classList",
                    ListByClassResp::get_classList_for_reflect,
                    ListByClassResp::mut_classList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListByClassResp_ArticleList>>(
                    "articleList",
                    ListByClassResp::get_articleList_for_reflect,
                    ListByClassResp::mut_articleList_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Tags>>(
                    "tags",
                    ListByClassResp::get_tags_for_reflect,
                    ListByClassResp::mut_tags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListByClassResp>(
                    "ListByClassResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListByClassResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_classList();
        self.clear_articleList();
        self.clear_tags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListByClassResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListByClassResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListByClassResp_List {
    // message fields
    pub _id: u32,
    pub createDate: ::std::string::String,
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListByClassResp_List {}

impl ListByClassResp_List {
    pub fn new() -> ListByClassResp_List {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListByClassResp_List {
        static mut instance: ::protobuf::lazy::Lazy<ListByClassResp_List> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListByClassResp_List,
        };
        unsafe {
            instance.get(ListByClassResp_List::new)
        }
    }

    // uint32 _id = 1;

    pub fn clear__id(&mut self) {
        self._id = 0;
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u32) {
        self._id = v;
    }

    pub fn get__id(&self) -> u32 {
        self._id
    }

    fn get__id_for_reflect(&self) -> &u32 {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut u32 {
        &mut self._id
    }

    // string createDate = 2;

    pub fn clear_createDate(&mut self) {
        self.createDate.clear();
    }

    // Param is passed by value, moved
    pub fn set_createDate(&mut self, v: ::std::string::String) {
        self.createDate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createDate(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // Take field
    pub fn take_createDate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createDate, ::std::string::String::new())
    }

    pub fn get_createDate(&self) -> &str {
        &self.createDate
    }

    fn get_createDate_for_reflect(&self) -> &::std::string::String {
        &self.createDate
    }

    fn mut_createDate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createDate
    }

    // string title = 3;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }
}

impl ::protobuf::Message for ListByClassResp_List {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self._id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createDate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
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
        if self._id != 0 {
            my_size += ::protobuf::rt::value_size(1, self._id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.createDate.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.createDate);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.title);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self._id != 0 {
            os.write_uint32(1, self._id)?;
        }
        if !self.createDate.is_empty() {
            os.write_string(2, &self.createDate)?;
        }
        if !self.title.is_empty() {
            os.write_string(3, &self.title)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListByClassResp_List {
    fn new() -> ListByClassResp_List {
        ListByClassResp_List::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListByClassResp_List>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "_id",
                    ListByClassResp_List::get__id_for_reflect,
                    ListByClassResp_List::mut__id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createDate",
                    ListByClassResp_List::get_createDate_for_reflect,
                    ListByClassResp_List::mut_createDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    ListByClassResp_List::get_title_for_reflect,
                    ListByClassResp_List::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListByClassResp_List>(
                    "ListByClassResp_List",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListByClassResp_List {
    fn clear(&mut self) {
        self.clear__id();
        self.clear_createDate();
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListByClassResp_List {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListByClassResp_List {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListByClassResp_ClassList {
    // message fields
    pub _id: u32,
    pub name: ::std::string::String,
    pub count: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListByClassResp_ClassList {}

impl ListByClassResp_ClassList {
    pub fn new() -> ListByClassResp_ClassList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListByClassResp_ClassList {
        static mut instance: ::protobuf::lazy::Lazy<ListByClassResp_ClassList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListByClassResp_ClassList,
        };
        unsafe {
            instance.get(ListByClassResp_ClassList::new)
        }
    }

    // uint32 _id = 1;

    pub fn clear__id(&mut self) {
        self._id = 0;
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u32) {
        self._id = v;
    }

    pub fn get__id(&self) -> u32 {
        self._id
    }

    fn get__id_for_reflect(&self) -> &u32 {
        &self._id
    }

    fn mut__id_for_reflect(&mut self) -> &mut u32 {
        &mut self._id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // uint32 count = 3;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u32 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u32 {
        &mut self.count
    }
}

impl ::protobuf::Message for ListByClassResp_ClassList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self._id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.count = tmp;
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
        if self._id != 0 {
            my_size += ::protobuf::rt::value_size(1, self._id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self._id != 0 {
            os.write_uint32(1, self._id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.count != 0 {
            os.write_uint32(3, self.count)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListByClassResp_ClassList {
    fn new() -> ListByClassResp_ClassList {
        ListByClassResp_ClassList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListByClassResp_ClassList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "_id",
                    ListByClassResp_ClassList::get__id_for_reflect,
                    ListByClassResp_ClassList::mut__id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ListByClassResp_ClassList::get_name_for_reflect,
                    ListByClassResp_ClassList::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    ListByClassResp_ClassList::get_count_for_reflect,
                    ListByClassResp_ClassList::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListByClassResp_ClassList>(
                    "ListByClassResp_ClassList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListByClassResp_ClassList {
    fn clear(&mut self) {
        self.clear__id();
        self.clear_name();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListByClassResp_ClassList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListByClassResp_ClassList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListByClassResp_ArticleList {
    // message fields
    pub year: u32,
    pub list: ::protobuf::RepeatedField<ListByClassResp_List>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListByClassResp_ArticleList {}

impl ListByClassResp_ArticleList {
    pub fn new() -> ListByClassResp_ArticleList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListByClassResp_ArticleList {
        static mut instance: ::protobuf::lazy::Lazy<ListByClassResp_ArticleList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListByClassResp_ArticleList,
        };
        unsafe {
            instance.get(ListByClassResp_ArticleList::new)
        }
    }

    // uint32 year = 1;

    pub fn clear_year(&mut self) {
        self.year = 0;
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: u32) {
        self.year = v;
    }

    pub fn get_year(&self) -> u32 {
        self.year
    }

    fn get_year_for_reflect(&self) -> &u32 {
        &self.year
    }

    fn mut_year_for_reflect(&mut self) -> &mut u32 {
        &mut self.year
    }

    // repeated .proto.ListByClassResp.List list = 2;

    pub fn clear_list(&mut self) {
        self.list.clear();
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: ::protobuf::RepeatedField<ListByClassResp_List>) {
        self.list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_list(&mut self) -> &mut ::protobuf::RepeatedField<ListByClassResp_List> {
        &mut self.list
    }

    // Take field
    pub fn take_list(&mut self) -> ::protobuf::RepeatedField<ListByClassResp_List> {
        ::std::mem::replace(&mut self.list, ::protobuf::RepeatedField::new())
    }

    pub fn get_list(&self) -> &[ListByClassResp_List] {
        &self.list
    }

    fn get_list_for_reflect(&self) -> &::protobuf::RepeatedField<ListByClassResp_List> {
        &self.list
    }

    fn mut_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListByClassResp_List> {
        &mut self.list
    }
}

impl ::protobuf::Message for ListByClassResp_ArticleList {
    fn is_initialized(&self) -> bool {
        for v in &self.list {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.year = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.list)?;
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
        if self.year != 0 {
            my_size += ::protobuf::rt::value_size(1, self.year, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.year != 0 {
            os.write_uint32(1, self.year)?;
        }
        for v in &self.list {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListByClassResp_ArticleList {
    fn new() -> ListByClassResp_ArticleList {
        ListByClassResp_ArticleList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListByClassResp_ArticleList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "year",
                    ListByClassResp_ArticleList::get_year_for_reflect,
                    ListByClassResp_ArticleList::mut_year_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListByClassResp_List>>(
                    "list",
                    ListByClassResp_ArticleList::get_list_for_reflect,
                    ListByClassResp_ArticleList::mut_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListByClassResp_ArticleList>(
                    "ListByClassResp_ArticleList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListByClassResp_ArticleList {
    fn clear(&mut self) {
        self.clear_year();
        self.clear_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListByClassResp_ArticleList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListByClassResp_ArticleList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminEditCategoryRequest {
    // message fields
    pub id: u32,
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminEditCategoryRequest {}

impl AdminEditCategoryRequest {
    pub fn new() -> AdminEditCategoryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminEditCategoryRequest {
        static mut instance: ::protobuf::lazy::Lazy<AdminEditCategoryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminEditCategoryRequest,
        };
        unsafe {
            instance.get(AdminEditCategoryRequest::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string description = 3;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }
}

impl ::protobuf::Message for AdminEditCategoryRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminEditCategoryRequest {
    fn new() -> AdminEditCategoryRequest {
        AdminEditCategoryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminEditCategoryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    AdminEditCategoryRequest::get_id_for_reflect,
                    AdminEditCategoryRequest::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AdminEditCategoryRequest::get_title_for_reflect,
                    AdminEditCategoryRequest::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    AdminEditCategoryRequest::get_description_for_reflect,
                    AdminEditCategoryRequest::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminEditCategoryRequest>(
                    "AdminEditCategoryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminEditCategoryRequest {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_title();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminEditCategoryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminEditCategoryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LinkBase {
    // message fields
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    pub email: ::std::string::String,
    pub url: ::std::string::String,
    pub headerImg: ::std::string::String,
    pub display: bool,
    pub id: u32,
    pub createTime: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LinkBase {}

impl LinkBase {
    pub fn new() -> LinkBase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LinkBase {
        static mut instance: ::protobuf::lazy::Lazy<LinkBase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LinkBase,
        };
        unsafe {
            instance.get(LinkBase::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // string email = 3;

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

    pub fn get_email(&self) -> &str {
        &self.email
    }

    fn get_email_for_reflect(&self) -> &::std::string::String {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // string url = 4;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.url, ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    fn get_url_for_reflect(&self) -> &::std::string::String {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // string headerImg = 5;

    pub fn clear_headerImg(&mut self) {
        self.headerImg.clear();
    }

    // Param is passed by value, moved
    pub fn set_headerImg(&mut self, v: ::std::string::String) {
        self.headerImg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headerImg(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // Take field
    pub fn take_headerImg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.headerImg, ::std::string::String::new())
    }

    pub fn get_headerImg(&self) -> &str {
        &self.headerImg
    }

    fn get_headerImg_for_reflect(&self) -> &::std::string::String {
        &self.headerImg
    }

    fn mut_headerImg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // bool display = 6;

    pub fn clear_display(&mut self) {
        self.display = false;
    }

    // Param is passed by value, moved
    pub fn set_display(&mut self, v: bool) {
        self.display = v;
    }

    pub fn get_display(&self) -> bool {
        self.display
    }

    fn get_display_for_reflect(&self) -> &bool {
        &self.display
    }

    fn mut_display_for_reflect(&mut self) -> &mut bool {
        &mut self.display
    }

    // uint32 id = 7;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // string createTime = 8;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createTime, ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        &self.createTime
    }

    fn get_createTime_for_reflect(&self) -> &::std::string::String {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }
}

impl ::protobuf::Message for LinkBase {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.url)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.headerImg)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.display = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createTime)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.email);
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.url);
        }
        if !self.headerImg.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.headerImg);
        }
        if self.display != false {
            my_size += 2;
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(7, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.createTime.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.createTime);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if !self.email.is_empty() {
            os.write_string(3, &self.email)?;
        }
        if !self.url.is_empty() {
            os.write_string(4, &self.url)?;
        }
        if !self.headerImg.is_empty() {
            os.write_string(5, &self.headerImg)?;
        }
        if self.display != false {
            os.write_bool(6, self.display)?;
        }
        if self.id != 0 {
            os.write_uint32(7, self.id)?;
        }
        if !self.createTime.is_empty() {
            os.write_string(8, &self.createTime)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LinkBase {
    fn new() -> LinkBase {
        LinkBase::new()
    }

    fn descriptor_static(_: ::std::option::Option<LinkBase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    LinkBase::get_title_for_reflect,
                    LinkBase::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    LinkBase::get_description_for_reflect,
                    LinkBase::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    LinkBase::get_email_for_reflect,
                    LinkBase::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    LinkBase::get_url_for_reflect,
                    LinkBase::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headerImg",
                    LinkBase::get_headerImg_for_reflect,
                    LinkBase::mut_headerImg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "display",
                    LinkBase::get_display_for_reflect,
                    LinkBase::mut_display_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    LinkBase::get_id_for_reflect,
                    LinkBase::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    LinkBase::get_createTime_for_reflect,
                    LinkBase::mut_createTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LinkBase>(
                    "LinkBase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LinkBase {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_description();
        self.clear_email();
        self.clear_url();
        self.clear_headerImg();
        self.clear_display();
        self.clear_id();
        self.clear_createTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LinkBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LinkBase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LinkRequest {
    // message fields
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    pub email: ::std::string::String,
    pub url: ::std::string::String,
    pub headerImg: ::std::string::String,
    pub display: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LinkRequest {}

impl LinkRequest {
    pub fn new() -> LinkRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LinkRequest {
        static mut instance: ::protobuf::lazy::Lazy<LinkRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LinkRequest,
        };
        unsafe {
            instance.get(LinkRequest::new)
        }
    }

    // string title = 3;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // string description = 4;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // string email = 5;

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

    pub fn get_email(&self) -> &str {
        &self.email
    }

    fn get_email_for_reflect(&self) -> &::std::string::String {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // string url = 6;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.url, ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    fn get_url_for_reflect(&self) -> &::std::string::String {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // string headerImg = 7;

    pub fn clear_headerImg(&mut self) {
        self.headerImg.clear();
    }

    // Param is passed by value, moved
    pub fn set_headerImg(&mut self, v: ::std::string::String) {
        self.headerImg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headerImg(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // Take field
    pub fn take_headerImg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.headerImg, ::std::string::String::new())
    }

    pub fn get_headerImg(&self) -> &str {
        &self.headerImg
    }

    fn get_headerImg_for_reflect(&self) -> &::std::string::String {
        &self.headerImg
    }

    fn mut_headerImg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.headerImg
    }

    // string display = 8;

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    // Param is passed by value, moved
    pub fn set_display(&mut self, v: ::std::string::String) {
        self.display = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display(&mut self) -> &mut ::std::string::String {
        &mut self.display
    }

    // Take field
    pub fn take_display(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display, ::std::string::String::new())
    }

    pub fn get_display(&self) -> &str {
        &self.display
    }

    fn get_display_for_reflect(&self) -> &::std::string::String {
        &self.display
    }

    fn mut_display_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.display
    }
}

impl ::protobuf::Message for LinkRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.url)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.headerImg)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.description);
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.email);
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.url);
        }
        if !self.headerImg.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.headerImg);
        }
        if !self.display.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.display);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(3, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(4, &self.description)?;
        }
        if !self.email.is_empty() {
            os.write_string(5, &self.email)?;
        }
        if !self.url.is_empty() {
            os.write_string(6, &self.url)?;
        }
        if !self.headerImg.is_empty() {
            os.write_string(7, &self.headerImg)?;
        }
        if !self.display.is_empty() {
            os.write_string(8, &self.display)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LinkRequest {
    fn new() -> LinkRequest {
        LinkRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LinkRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    LinkRequest::get_title_for_reflect,
                    LinkRequest::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    LinkRequest::get_description_for_reflect,
                    LinkRequest::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    LinkRequest::get_email_for_reflect,
                    LinkRequest::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    LinkRequest::get_url_for_reflect,
                    LinkRequest::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headerImg",
                    LinkRequest::get_headerImg_for_reflect,
                    LinkRequest::mut_headerImg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "display",
                    LinkRequest::get_display_for_reflect,
                    LinkRequest::mut_display_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LinkRequest>(
                    "LinkRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LinkRequest {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_description();
        self.clear_email();
        self.clear_url();
        self.clear_headerImg();
        self.clear_display();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LinkRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LinkRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LinkListResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub total: u32,
    pub rows: ::protobuf::RepeatedField<LinkBase>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LinkListResp {}

impl LinkListResp {
    pub fn new() -> LinkListResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LinkListResp {
        static mut instance: ::protobuf::lazy::Lazy<LinkListResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LinkListResp,
        };
        unsafe {
            instance.get(LinkListResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // uint32 total = 3;

    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u32) {
        self.total = v;
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    fn get_total_for_reflect(&self) -> &u32 {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut u32 {
        &mut self.total
    }

    // repeated .proto.LinkBase rows = 4;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<LinkBase>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<LinkBase> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<LinkBase> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[LinkBase] {
        &self.rows
    }

    fn get_rows_for_reflect(&self) -> &::protobuf::RepeatedField<LinkBase> {
        &self.rows
    }

    fn mut_rows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LinkBase> {
        &mut self.rows
    }
}

impl ::protobuf::Message for LinkListResp {
    fn is_initialized(&self) -> bool {
        for v in &self.rows {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.total != 0 {
            os.write_uint32(3, self.total)?;
        }
        for v in &self.rows {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LinkListResp {
    fn new() -> LinkListResp {
        LinkListResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<LinkListResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    LinkListResp::get_code_for_reflect,
                    LinkListResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    LinkListResp::get_msg_for_reflect,
                    LinkListResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total",
                    LinkListResp::get_total_for_reflect,
                    LinkListResp::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LinkBase>>(
                    "rows",
                    LinkListResp::get_rows_for_reflect,
                    LinkListResp::mut_rows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LinkListResp>(
                    "LinkListResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LinkListResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_total();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LinkListResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LinkListResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserInfoResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub userId: u32,
    pub username: ::std::string::String,
    pub status: u32,
    pub avatar: ::std::string::String,
    pub linkname: ::std::string::String,
    pub linkUrl: ::std::string::String,
    pub linkDesc: ::std::string::String,
    pub logoUrl: ::std::string::String,
    pub state: bool,
    pub label: u32,
    pub receive_update: bool,
    pub token: ::std::string::String,
    pub verify_status: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserInfoResp {}

impl UserInfoResp {
    pub fn new() -> UserInfoResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserInfoResp {
        static mut instance: ::protobuf::lazy::Lazy<UserInfoResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserInfoResp,
        };
        unsafe {
            instance.get(UserInfoResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // uint32 userId = 3;

    pub fn clear_userId(&mut self) {
        self.userId = 0;
    }

    // Param is passed by value, moved
    pub fn set_userId(&mut self, v: u32) {
        self.userId = v;
    }

    pub fn get_userId(&self) -> u32 {
        self.userId
    }

    fn get_userId_for_reflect(&self) -> &u32 {
        &self.userId
    }

    fn mut_userId_for_reflect(&mut self) -> &mut u32 {
        &mut self.userId
    }

    // string username = 4;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // uint32 status = 5;

    pub fn clear_status(&mut self) {
        self.status = 0;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = v;
    }

    pub fn get_status(&self) -> u32 {
        self.status
    }

    fn get_status_for_reflect(&self) -> &u32 {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut u32 {
        &mut self.status
    }

    // string avatar = 6;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: ::std::string::String) {
        self.avatar = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // Take field
    pub fn take_avatar(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.avatar, ::std::string::String::new())
    }

    pub fn get_avatar(&self) -> &str {
        &self.avatar
    }

    fn get_avatar_for_reflect(&self) -> &::std::string::String {
        &self.avatar
    }

    fn mut_avatar_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.avatar
    }

    // string linkname = 7;

    pub fn clear_linkname(&mut self) {
        self.linkname.clear();
    }

    // Param is passed by value, moved
    pub fn set_linkname(&mut self, v: ::std::string::String) {
        self.linkname = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkname(&mut self) -> &mut ::std::string::String {
        &mut self.linkname
    }

    // Take field
    pub fn take_linkname(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linkname, ::std::string::String::new())
    }

    pub fn get_linkname(&self) -> &str {
        &self.linkname
    }

    fn get_linkname_for_reflect(&self) -> &::std::string::String {
        &self.linkname
    }

    fn mut_linkname_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.linkname
    }

    // string linkUrl = 8;

    pub fn clear_linkUrl(&mut self) {
        self.linkUrl.clear();
    }

    // Param is passed by value, moved
    pub fn set_linkUrl(&mut self, v: ::std::string::String) {
        self.linkUrl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkUrl(&mut self) -> &mut ::std::string::String {
        &mut self.linkUrl
    }

    // Take field
    pub fn take_linkUrl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linkUrl, ::std::string::String::new())
    }

    pub fn get_linkUrl(&self) -> &str {
        &self.linkUrl
    }

    fn get_linkUrl_for_reflect(&self) -> &::std::string::String {
        &self.linkUrl
    }

    fn mut_linkUrl_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.linkUrl
    }

    // string linkDesc = 9;

    pub fn clear_linkDesc(&mut self) {
        self.linkDesc.clear();
    }

    // Param is passed by value, moved
    pub fn set_linkDesc(&mut self, v: ::std::string::String) {
        self.linkDesc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkDesc(&mut self) -> &mut ::std::string::String {
        &mut self.linkDesc
    }

    // Take field
    pub fn take_linkDesc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linkDesc, ::std::string::String::new())
    }

    pub fn get_linkDesc(&self) -> &str {
        &self.linkDesc
    }

    fn get_linkDesc_for_reflect(&self) -> &::std::string::String {
        &self.linkDesc
    }

    fn mut_linkDesc_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.linkDesc
    }

    // string logoUrl = 10;

    pub fn clear_logoUrl(&mut self) {
        self.logoUrl.clear();
    }

    // Param is passed by value, moved
    pub fn set_logoUrl(&mut self, v: ::std::string::String) {
        self.logoUrl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logoUrl(&mut self) -> &mut ::std::string::String {
        &mut self.logoUrl
    }

    // Take field
    pub fn take_logoUrl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.logoUrl, ::std::string::String::new())
    }

    pub fn get_logoUrl(&self) -> &str {
        &self.logoUrl
    }

    fn get_logoUrl_for_reflect(&self) -> &::std::string::String {
        &self.logoUrl
    }

    fn mut_logoUrl_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.logoUrl
    }

    // bool state = 11;

    pub fn clear_state(&mut self) {
        self.state = false;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: bool) {
        self.state = v;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    fn get_state_for_reflect(&self) -> &bool {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut bool {
        &mut self.state
    }

    // uint32 label = 12;

    pub fn clear_label(&mut self) {
        self.label = 0;
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: u32) {
        self.label = v;
    }

    pub fn get_label(&self) -> u32 {
        self.label
    }

    fn get_label_for_reflect(&self) -> &u32 {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut u32 {
        &mut self.label
    }

    // bool receive_update = 13;

    pub fn clear_receive_update(&mut self) {
        self.receive_update = false;
    }

    // Param is passed by value, moved
    pub fn set_receive_update(&mut self, v: bool) {
        self.receive_update = v;
    }

    pub fn get_receive_update(&self) -> bool {
        self.receive_update
    }

    fn get_receive_update_for_reflect(&self) -> &bool {
        &self.receive_update
    }

    fn mut_receive_update_for_reflect(&mut self) -> &mut bool {
        &mut self.receive_update
    }

    // string token = 14;

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

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn get_token_for_reflect(&self) -> &::std::string::String {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }

    // string verify_status = 15;

    pub fn clear_verify_status(&mut self) {
        self.verify_status.clear();
    }

    // Param is passed by value, moved
    pub fn set_verify_status(&mut self, v: ::std::string::String) {
        self.verify_status = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_verify_status(&mut self) -> &mut ::std::string::String {
        &mut self.verify_status
    }

    // Take field
    pub fn take_verify_status(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.verify_status, ::std::string::String::new())
    }

    pub fn get_verify_status(&self) -> &str {
        &self.verify_status
    }

    fn get_verify_status_for_reflect(&self) -> &::std::string::String {
        &self.verify_status
    }

    fn mut_verify_status_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.verify_status
    }
}

impl ::protobuf::Message for UserInfoResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.userId = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.status = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.avatar)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.linkname)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.linkUrl)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.linkDesc)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.logoUrl)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.state = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.label = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.receive_update = tmp;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.verify_status)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.userId != 0 {
            my_size += ::protobuf::rt::value_size(3, self.userId, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.username);
        }
        if self.status != 0 {
            my_size += ::protobuf::rt::value_size(5, self.status, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.avatar.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.avatar);
        }
        if !self.linkname.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.linkname);
        }
        if !self.linkUrl.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.linkUrl);
        }
        if !self.linkDesc.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.linkDesc);
        }
        if !self.logoUrl.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.logoUrl);
        }
        if self.state != false {
            my_size += 2;
        }
        if self.label != 0 {
            my_size += ::protobuf::rt::value_size(12, self.label, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.receive_update != false {
            my_size += 2;
        }
        if !self.token.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.token);
        }
        if !self.verify_status.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.verify_status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.userId != 0 {
            os.write_uint32(3, self.userId)?;
        }
        if !self.username.is_empty() {
            os.write_string(4, &self.username)?;
        }
        if self.status != 0 {
            os.write_uint32(5, self.status)?;
        }
        if !self.avatar.is_empty() {
            os.write_string(6, &self.avatar)?;
        }
        if !self.linkname.is_empty() {
            os.write_string(7, &self.linkname)?;
        }
        if !self.linkUrl.is_empty() {
            os.write_string(8, &self.linkUrl)?;
        }
        if !self.linkDesc.is_empty() {
            os.write_string(9, &self.linkDesc)?;
        }
        if !self.logoUrl.is_empty() {
            os.write_string(10, &self.logoUrl)?;
        }
        if self.state != false {
            os.write_bool(11, self.state)?;
        }
        if self.label != 0 {
            os.write_uint32(12, self.label)?;
        }
        if self.receive_update != false {
            os.write_bool(13, self.receive_update)?;
        }
        if !self.token.is_empty() {
            os.write_string(14, &self.token)?;
        }
        if !self.verify_status.is_empty() {
            os.write_string(15, &self.verify_status)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserInfoResp {
    fn new() -> UserInfoResp {
        UserInfoResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserInfoResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    UserInfoResp::get_code_for_reflect,
                    UserInfoResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    UserInfoResp::get_msg_for_reflect,
                    UserInfoResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "userId",
                    UserInfoResp::get_userId_for_reflect,
                    UserInfoResp::mut_userId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    UserInfoResp::get_username_for_reflect,
                    UserInfoResp::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    UserInfoResp::get_status_for_reflect,
                    UserInfoResp::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "avatar",
                    UserInfoResp::get_avatar_for_reflect,
                    UserInfoResp::mut_avatar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "linkname",
                    UserInfoResp::get_linkname_for_reflect,
                    UserInfoResp::mut_linkname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "linkUrl",
                    UserInfoResp::get_linkUrl_for_reflect,
                    UserInfoResp::mut_linkUrl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "linkDesc",
                    UserInfoResp::get_linkDesc_for_reflect,
                    UserInfoResp::mut_linkDesc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "logoUrl",
                    UserInfoResp::get_logoUrl_for_reflect,
                    UserInfoResp::mut_logoUrl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "state",
                    UserInfoResp::get_state_for_reflect,
                    UserInfoResp::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "label",
                    UserInfoResp::get_label_for_reflect,
                    UserInfoResp::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "receive_update",
                    UserInfoResp::get_receive_update_for_reflect,
                    UserInfoResp::mut_receive_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    UserInfoResp::get_token_for_reflect,
                    UserInfoResp::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verify_status",
                    UserInfoResp::get_verify_status_for_reflect,
                    UserInfoResp::mut_verify_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserInfoResp>(
                    "UserInfoResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserInfoResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_userId();
        self.clear_username();
        self.clear_status();
        self.clear_avatar();
        self.clear_linkname();
        self.clear_linkUrl();
        self.clear_linkDesc();
        self.clear_logoUrl();
        self.clear_state();
        self.clear_label();
        self.clear_receive_update();
        self.clear_token();
        self.clear_verify_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserInfoResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserInfoResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadFileResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub url: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UploadFileResp {}

impl UploadFileResp {
    pub fn new() -> UploadFileResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UploadFileResp {
        static mut instance: ::protobuf::lazy::Lazy<UploadFileResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UploadFileResp,
        };
        unsafe {
            instance.get(UploadFileResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string url = 3;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.url, ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    fn get_url_for_reflect(&self) -> &::std::string::String {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }
}

impl ::protobuf::Message for UploadFileResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.url)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.url.is_empty() {
            os.write_string(3, &self.url)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UploadFileResp {
    fn new() -> UploadFileResp {
        UploadFileResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<UploadFileResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    UploadFileResp::get_code_for_reflect,
                    UploadFileResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    UploadFileResp::get_msg_for_reflect,
                    UploadFileResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    UploadFileResp::get_url_for_reflect,
                    UploadFileResp::mut_url_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UploadFileResp>(
                    "UploadFileResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UploadFileResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadFileResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadFileResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EditUserInfoRequest {
    // message fields
    pub userId: u32,
    pub label: u32,
    pub state: bool,
    pub linkUrl: ::std::string::String,
    pub linkname: ::std::string::String,
    pub linkDesc: ::std::string::String,
    pub receive_update: bool,
    pub logoUrl: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EditUserInfoRequest {}

impl EditUserInfoRequest {
    pub fn new() -> EditUserInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EditUserInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<EditUserInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EditUserInfoRequest,
        };
        unsafe {
            instance.get(EditUserInfoRequest::new)
        }
    }

    // uint32 userId = 1;

    pub fn clear_userId(&mut self) {
        self.userId = 0;
    }

    // Param is passed by value, moved
    pub fn set_userId(&mut self, v: u32) {
        self.userId = v;
    }

    pub fn get_userId(&self) -> u32 {
        self.userId
    }

    fn get_userId_for_reflect(&self) -> &u32 {
        &self.userId
    }

    fn mut_userId_for_reflect(&mut self) -> &mut u32 {
        &mut self.userId
    }

    // uint32 label = 2;

    pub fn clear_label(&mut self) {
        self.label = 0;
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: u32) {
        self.label = v;
    }

    pub fn get_label(&self) -> u32 {
        self.label
    }

    fn get_label_for_reflect(&self) -> &u32 {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut u32 {
        &mut self.label
    }

    // bool state = 3;

    pub fn clear_state(&mut self) {
        self.state = false;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: bool) {
        self.state = v;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    fn get_state_for_reflect(&self) -> &bool {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut bool {
        &mut self.state
    }

    // string linkUrl = 4;

    pub fn clear_linkUrl(&mut self) {
        self.linkUrl.clear();
    }

    // Param is passed by value, moved
    pub fn set_linkUrl(&mut self, v: ::std::string::String) {
        self.linkUrl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkUrl(&mut self) -> &mut ::std::string::String {
        &mut self.linkUrl
    }

    // Take field
    pub fn take_linkUrl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linkUrl, ::std::string::String::new())
    }

    pub fn get_linkUrl(&self) -> &str {
        &self.linkUrl
    }

    fn get_linkUrl_for_reflect(&self) -> &::std::string::String {
        &self.linkUrl
    }

    fn mut_linkUrl_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.linkUrl
    }

    // string linkname = 5;

    pub fn clear_linkname(&mut self) {
        self.linkname.clear();
    }

    // Param is passed by value, moved
    pub fn set_linkname(&mut self, v: ::std::string::String) {
        self.linkname = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkname(&mut self) -> &mut ::std::string::String {
        &mut self.linkname
    }

    // Take field
    pub fn take_linkname(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linkname, ::std::string::String::new())
    }

    pub fn get_linkname(&self) -> &str {
        &self.linkname
    }

    fn get_linkname_for_reflect(&self) -> &::std::string::String {
        &self.linkname
    }

    fn mut_linkname_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.linkname
    }

    // string linkDesc = 6;

    pub fn clear_linkDesc(&mut self) {
        self.linkDesc.clear();
    }

    // Param is passed by value, moved
    pub fn set_linkDesc(&mut self, v: ::std::string::String) {
        self.linkDesc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkDesc(&mut self) -> &mut ::std::string::String {
        &mut self.linkDesc
    }

    // Take field
    pub fn take_linkDesc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linkDesc, ::std::string::String::new())
    }

    pub fn get_linkDesc(&self) -> &str {
        &self.linkDesc
    }

    fn get_linkDesc_for_reflect(&self) -> &::std::string::String {
        &self.linkDesc
    }

    fn mut_linkDesc_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.linkDesc
    }

    // bool receive_update = 7;

    pub fn clear_receive_update(&mut self) {
        self.receive_update = false;
    }

    // Param is passed by value, moved
    pub fn set_receive_update(&mut self, v: bool) {
        self.receive_update = v;
    }

    pub fn get_receive_update(&self) -> bool {
        self.receive_update
    }

    fn get_receive_update_for_reflect(&self) -> &bool {
        &self.receive_update
    }

    fn mut_receive_update_for_reflect(&mut self) -> &mut bool {
        &mut self.receive_update
    }

    // string logoUrl = 8;

    pub fn clear_logoUrl(&mut self) {
        self.logoUrl.clear();
    }

    // Param is passed by value, moved
    pub fn set_logoUrl(&mut self, v: ::std::string::String) {
        self.logoUrl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logoUrl(&mut self) -> &mut ::std::string::String {
        &mut self.logoUrl
    }

    // Take field
    pub fn take_logoUrl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.logoUrl, ::std::string::String::new())
    }

    pub fn get_logoUrl(&self) -> &str {
        &self.logoUrl
    }

    fn get_logoUrl_for_reflect(&self) -> &::std::string::String {
        &self.logoUrl
    }

    fn mut_logoUrl_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.logoUrl
    }
}

impl ::protobuf::Message for EditUserInfoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.userId = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.label = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.state = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.linkUrl)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.linkname)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.linkDesc)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.receive_update = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.logoUrl)?;
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
        if self.userId != 0 {
            my_size += ::protobuf::rt::value_size(1, self.userId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.label != 0 {
            my_size += ::protobuf::rt::value_size(2, self.label, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.state != false {
            my_size += 2;
        }
        if !self.linkUrl.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.linkUrl);
        }
        if !self.linkname.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.linkname);
        }
        if !self.linkDesc.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.linkDesc);
        }
        if self.receive_update != false {
            my_size += 2;
        }
        if !self.logoUrl.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.logoUrl);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.userId != 0 {
            os.write_uint32(1, self.userId)?;
        }
        if self.label != 0 {
            os.write_uint32(2, self.label)?;
        }
        if self.state != false {
            os.write_bool(3, self.state)?;
        }
        if !self.linkUrl.is_empty() {
            os.write_string(4, &self.linkUrl)?;
        }
        if !self.linkname.is_empty() {
            os.write_string(5, &self.linkname)?;
        }
        if !self.linkDesc.is_empty() {
            os.write_string(6, &self.linkDesc)?;
        }
        if self.receive_update != false {
            os.write_bool(7, self.receive_update)?;
        }
        if !self.logoUrl.is_empty() {
            os.write_string(8, &self.logoUrl)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EditUserInfoRequest {
    fn new() -> EditUserInfoRequest {
        EditUserInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<EditUserInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "userId",
                    EditUserInfoRequest::get_userId_for_reflect,
                    EditUserInfoRequest::mut_userId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "label",
                    EditUserInfoRequest::get_label_for_reflect,
                    EditUserInfoRequest::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "state",
                    EditUserInfoRequest::get_state_for_reflect,
                    EditUserInfoRequest::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "linkUrl",
                    EditUserInfoRequest::get_linkUrl_for_reflect,
                    EditUserInfoRequest::mut_linkUrl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "linkname",
                    EditUserInfoRequest::get_linkname_for_reflect,
                    EditUserInfoRequest::mut_linkname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "linkDesc",
                    EditUserInfoRequest::get_linkDesc_for_reflect,
                    EditUserInfoRequest::mut_linkDesc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "receive_update",
                    EditUserInfoRequest::get_receive_update_for_reflect,
                    EditUserInfoRequest::mut_receive_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "logoUrl",
                    EditUserInfoRequest::get_logoUrl_for_reflect,
                    EditUserInfoRequest::mut_logoUrl_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EditUserInfoRequest>(
                    "EditUserInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EditUserInfoRequest {
    fn clear(&mut self) {
        self.clear_userId();
        self.clear_label();
        self.clear_state();
        self.clear_linkUrl();
        self.clear_linkname();
        self.clear_linkDesc();
        self.clear_receive_update();
        self.clear_logoUrl();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EditUserInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EditUserInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateAboutRequest {
    // message fields
    pub content: ::std::string::String,
    pub htmlContent: ::std::string::String,
    pub id: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateAboutRequest {}

impl UpdateAboutRequest {
    pub fn new() -> UpdateAboutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateAboutRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateAboutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateAboutRequest,
        };
        unsafe {
            instance.get(UpdateAboutRequest::new)
        }
    }

    // string content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // string htmlContent = 2;

    pub fn clear_htmlContent(&mut self) {
        self.htmlContent.clear();
    }

    // Param is passed by value, moved
    pub fn set_htmlContent(&mut self, v: ::std::string::String) {
        self.htmlContent = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_htmlContent(&mut self) -> &mut ::std::string::String {
        &mut self.htmlContent
    }

    // Take field
    pub fn take_htmlContent(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.htmlContent, ::std::string::String::new())
    }

    pub fn get_htmlContent(&self) -> &str {
        &self.htmlContent
    }

    fn get_htmlContent_for_reflect(&self) -> &::std::string::String {
        &self.htmlContent
    }

    fn mut_htmlContent_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.htmlContent
    }

    // uint32 id = 3;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }
}

impl ::protobuf::Message for UpdateAboutRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.htmlContent)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
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
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.content);
        }
        if !self.htmlContent.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.htmlContent);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.content.is_empty() {
            os.write_string(1, &self.content)?;
        }
        if !self.htmlContent.is_empty() {
            os.write_string(2, &self.htmlContent)?;
        }
        if self.id != 0 {
            os.write_uint32(3, self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateAboutRequest {
    fn new() -> UpdateAboutRequest {
        UpdateAboutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateAboutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    UpdateAboutRequest::get_content_for_reflect,
                    UpdateAboutRequest::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "htmlContent",
                    UpdateAboutRequest::get_htmlContent_for_reflect,
                    UpdateAboutRequest::mut_htmlContent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    UpdateAboutRequest::get_id_for_reflect,
                    UpdateAboutRequest::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateAboutRequest>(
                    "UpdateAboutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateAboutRequest {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_htmlContent();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateAboutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateAboutRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PanelGroupResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub blogCount: u32,
    pub visitorCount: u32,
    pub userCount: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PanelGroupResp {}

impl PanelGroupResp {
    pub fn new() -> PanelGroupResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PanelGroupResp {
        static mut instance: ::protobuf::lazy::Lazy<PanelGroupResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PanelGroupResp,
        };
        unsafe {
            instance.get(PanelGroupResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // uint32 blogCount = 3;

    pub fn clear_blogCount(&mut self) {
        self.blogCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_blogCount(&mut self, v: u32) {
        self.blogCount = v;
    }

    pub fn get_blogCount(&self) -> u32 {
        self.blogCount
    }

    fn get_blogCount_for_reflect(&self) -> &u32 {
        &self.blogCount
    }

    fn mut_blogCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.blogCount
    }

    // uint32 visitorCount = 4;

    pub fn clear_visitorCount(&mut self) {
        self.visitorCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_visitorCount(&mut self, v: u32) {
        self.visitorCount = v;
    }

    pub fn get_visitorCount(&self) -> u32 {
        self.visitorCount
    }

    fn get_visitorCount_for_reflect(&self) -> &u32 {
        &self.visitorCount
    }

    fn mut_visitorCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.visitorCount
    }

    // uint32 userCount = 5;

    pub fn clear_userCount(&mut self) {
        self.userCount = 0;
    }

    // Param is passed by value, moved
    pub fn set_userCount(&mut self, v: u32) {
        self.userCount = v;
    }

    pub fn get_userCount(&self) -> u32 {
        self.userCount
    }

    fn get_userCount_for_reflect(&self) -> &u32 {
        &self.userCount
    }

    fn mut_userCount_for_reflect(&mut self) -> &mut u32 {
        &mut self.userCount
    }
}

impl ::protobuf::Message for PanelGroupResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.blogCount = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.visitorCount = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.userCount = tmp;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.blogCount != 0 {
            my_size += ::protobuf::rt::value_size(3, self.blogCount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.visitorCount != 0 {
            my_size += ::protobuf::rt::value_size(4, self.visitorCount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.userCount != 0 {
            my_size += ::protobuf::rt::value_size(5, self.userCount, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.blogCount != 0 {
            os.write_uint32(3, self.blogCount)?;
        }
        if self.visitorCount != 0 {
            os.write_uint32(4, self.visitorCount)?;
        }
        if self.userCount != 0 {
            os.write_uint32(5, self.userCount)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PanelGroupResp {
    fn new() -> PanelGroupResp {
        PanelGroupResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<PanelGroupResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    PanelGroupResp::get_code_for_reflect,
                    PanelGroupResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    PanelGroupResp::get_msg_for_reflect,
                    PanelGroupResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "blogCount",
                    PanelGroupResp::get_blogCount_for_reflect,
                    PanelGroupResp::mut_blogCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "visitorCount",
                    PanelGroupResp::get_visitorCount_for_reflect,
                    PanelGroupResp::mut_visitorCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "userCount",
                    PanelGroupResp::get_userCount_for_reflect,
                    PanelGroupResp::mut_userCount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PanelGroupResp>(
                    "PanelGroupResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PanelGroupResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_blogCount();
        self.clear_visitorCount();
        self.clear_userCount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PanelGroupResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PanelGroupResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LineChartDataResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub axisData: ::std::vec::Vec<u32>,
    pub expectedData: ::std::vec::Vec<u32>,
    pub actualData: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LineChartDataResp {}

impl LineChartDataResp {
    pub fn new() -> LineChartDataResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LineChartDataResp {
        static mut instance: ::protobuf::lazy::Lazy<LineChartDataResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LineChartDataResp,
        };
        unsafe {
            instance.get(LineChartDataResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated uint32 axisData = 3;

    pub fn clear_axisData(&mut self) {
        self.axisData.clear();
    }

    // Param is passed by value, moved
    pub fn set_axisData(&mut self, v: ::std::vec::Vec<u32>) {
        self.axisData = v;
    }

    // Mutable pointer to the field.
    pub fn mut_axisData(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.axisData
    }

    // Take field
    pub fn take_axisData(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.axisData, ::std::vec::Vec::new())
    }

    pub fn get_axisData(&self) -> &[u32] {
        &self.axisData
    }

    fn get_axisData_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.axisData
    }

    fn mut_axisData_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.axisData
    }

    // repeated uint32 expectedData = 4;

    pub fn clear_expectedData(&mut self) {
        self.expectedData.clear();
    }

    // Param is passed by value, moved
    pub fn set_expectedData(&mut self, v: ::std::vec::Vec<u32>) {
        self.expectedData = v;
    }

    // Mutable pointer to the field.
    pub fn mut_expectedData(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.expectedData
    }

    // Take field
    pub fn take_expectedData(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.expectedData, ::std::vec::Vec::new())
    }

    pub fn get_expectedData(&self) -> &[u32] {
        &self.expectedData
    }

    fn get_expectedData_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.expectedData
    }

    fn mut_expectedData_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.expectedData
    }

    // repeated uint32 actualData = 5;

    pub fn clear_actualData(&mut self) {
        self.actualData.clear();
    }

    // Param is passed by value, moved
    pub fn set_actualData(&mut self, v: ::std::vec::Vec<u32>) {
        self.actualData = v;
    }

    // Mutable pointer to the field.
    pub fn mut_actualData(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.actualData
    }

    // Take field
    pub fn take_actualData(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.actualData, ::std::vec::Vec::new())
    }

    pub fn get_actualData(&self) -> &[u32] {
        &self.actualData
    }

    fn get_actualData_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.actualData
    }

    fn mut_actualData_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.actualData
    }
}

impl ::protobuf::Message for LineChartDataResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.axisData)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.expectedData)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.actualData)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.axisData {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.expectedData {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.actualData {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.axisData {
            os.write_uint32(3, *v)?;
        };
        for v in &self.expectedData {
            os.write_uint32(4, *v)?;
        };
        for v in &self.actualData {
            os.write_uint32(5, *v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LineChartDataResp {
    fn new() -> LineChartDataResp {
        LineChartDataResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<LineChartDataResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    LineChartDataResp::get_code_for_reflect,
                    LineChartDataResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    LineChartDataResp::get_msg_for_reflect,
                    LineChartDataResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "axisData",
                    LineChartDataResp::get_axisData_for_reflect,
                    LineChartDataResp::mut_axisData_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "expectedData",
                    LineChartDataResp::get_expectedData_for_reflect,
                    LineChartDataResp::mut_expectedData_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "actualData",
                    LineChartDataResp::get_actualData_for_reflect,
                    LineChartDataResp::mut_actualData_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LineChartDataResp>(
                    "LineChartDataResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LineChartDataResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_axisData();
        self.clear_expectedData();
        self.clear_actualData();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LineChartDataResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineChartDataResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LikeOrCollectRequest {
    // message fields
    pub id: u32,
    pub flag: bool,
    pub isLike: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LikeOrCollectRequest {}

impl LikeOrCollectRequest {
    pub fn new() -> LikeOrCollectRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LikeOrCollectRequest {
        static mut instance: ::protobuf::lazy::Lazy<LikeOrCollectRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LikeOrCollectRequest,
        };
        unsafe {
            instance.get(LikeOrCollectRequest::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // bool flag = 2;

    pub fn clear_flag(&mut self) {
        self.flag = false;
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: bool) {
        self.flag = v;
    }

    pub fn get_flag(&self) -> bool {
        self.flag
    }

    fn get_flag_for_reflect(&self) -> &bool {
        &self.flag
    }

    fn mut_flag_for_reflect(&mut self) -> &mut bool {
        &mut self.flag
    }

    // bool isLike = 3;

    pub fn clear_isLike(&mut self) {
        self.isLike = false;
    }

    // Param is passed by value, moved
    pub fn set_isLike(&mut self, v: bool) {
        self.isLike = v;
    }

    pub fn get_isLike(&self) -> bool {
        self.isLike
    }

    fn get_isLike_for_reflect(&self) -> &bool {
        &self.isLike
    }

    fn mut_isLike_for_reflect(&mut self) -> &mut bool {
        &mut self.isLike
    }
}

impl ::protobuf::Message for LikeOrCollectRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.flag = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isLike = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.flag != false {
            my_size += 2;
        }
        if self.isLike != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.flag != false {
            os.write_bool(2, self.flag)?;
        }
        if self.isLike != false {
            os.write_bool(3, self.isLike)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LikeOrCollectRequest {
    fn new() -> LikeOrCollectRequest {
        LikeOrCollectRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LikeOrCollectRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    LikeOrCollectRequest::get_id_for_reflect,
                    LikeOrCollectRequest::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "flag",
                    LikeOrCollectRequest::get_flag_for_reflect,
                    LikeOrCollectRequest::mut_flag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isLike",
                    LikeOrCollectRequest::get_isLike_for_reflect,
                    LikeOrCollectRequest::mut_isLike_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LikeOrCollectRequest>(
                    "LikeOrCollectRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LikeOrCollectRequest {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_flag();
        self.clear_isLike();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LikeOrCollectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LikeOrCollectRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsLikeOrCollectResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub like: bool,
    pub collect: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsLikeOrCollectResp {}

impl IsLikeOrCollectResp {
    pub fn new() -> IsLikeOrCollectResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsLikeOrCollectResp {
        static mut instance: ::protobuf::lazy::Lazy<IsLikeOrCollectResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsLikeOrCollectResp,
        };
        unsafe {
            instance.get(IsLikeOrCollectResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // bool like = 3;

    pub fn clear_like(&mut self) {
        self.like = false;
    }

    // Param is passed by value, moved
    pub fn set_like(&mut self, v: bool) {
        self.like = v;
    }

    pub fn get_like(&self) -> bool {
        self.like
    }

    fn get_like_for_reflect(&self) -> &bool {
        &self.like
    }

    fn mut_like_for_reflect(&mut self) -> &mut bool {
        &mut self.like
    }

    // bool collect = 4;

    pub fn clear_collect(&mut self) {
        self.collect = false;
    }

    // Param is passed by value, moved
    pub fn set_collect(&mut self, v: bool) {
        self.collect = v;
    }

    pub fn get_collect(&self) -> bool {
        self.collect
    }

    fn get_collect_for_reflect(&self) -> &bool {
        &self.collect
    }

    fn mut_collect_for_reflect(&mut self) -> &mut bool {
        &mut self.collect
    }
}

impl ::protobuf::Message for IsLikeOrCollectResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.like = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.collect = tmp;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.like != false {
            my_size += 2;
        }
        if self.collect != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.like != false {
            os.write_bool(3, self.like)?;
        }
        if self.collect != false {
            os.write_bool(4, self.collect)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsLikeOrCollectResp {
    fn new() -> IsLikeOrCollectResp {
        IsLikeOrCollectResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsLikeOrCollectResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    IsLikeOrCollectResp::get_code_for_reflect,
                    IsLikeOrCollectResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    IsLikeOrCollectResp::get_msg_for_reflect,
                    IsLikeOrCollectResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "like",
                    IsLikeOrCollectResp::get_like_for_reflect,
                    IsLikeOrCollectResp::mut_like_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "collect",
                    IsLikeOrCollectResp::get_collect_for_reflect,
                    IsLikeOrCollectResp::mut_collect_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsLikeOrCollectResp>(
                    "IsLikeOrCollectResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsLikeOrCollectResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_like();
        self.clear_collect();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IsLikeOrCollectResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsLikeOrCollectResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RewardResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub wechatImage: ::std::string::String,
    pub aliPayImage: ::std::string::String,
    pub rewards: ::protobuf::RepeatedField<RewardResp_Rewards>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RewardResp {}

impl RewardResp {
    pub fn new() -> RewardResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RewardResp {
        static mut instance: ::protobuf::lazy::Lazy<RewardResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RewardResp,
        };
        unsafe {
            instance.get(RewardResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // string wechatImage = 3;

    pub fn clear_wechatImage(&mut self) {
        self.wechatImage.clear();
    }

    // Param is passed by value, moved
    pub fn set_wechatImage(&mut self, v: ::std::string::String) {
        self.wechatImage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wechatImage(&mut self) -> &mut ::std::string::String {
        &mut self.wechatImage
    }

    // Take field
    pub fn take_wechatImage(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.wechatImage, ::std::string::String::new())
    }

    pub fn get_wechatImage(&self) -> &str {
        &self.wechatImage
    }

    fn get_wechatImage_for_reflect(&self) -> &::std::string::String {
        &self.wechatImage
    }

    fn mut_wechatImage_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.wechatImage
    }

    // string aliPayImage = 4;

    pub fn clear_aliPayImage(&mut self) {
        self.aliPayImage.clear();
    }

    // Param is passed by value, moved
    pub fn set_aliPayImage(&mut self, v: ::std::string::String) {
        self.aliPayImage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_aliPayImage(&mut self) -> &mut ::std::string::String {
        &mut self.aliPayImage
    }

    // Take field
    pub fn take_aliPayImage(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.aliPayImage, ::std::string::String::new())
    }

    pub fn get_aliPayImage(&self) -> &str {
        &self.aliPayImage
    }

    fn get_aliPayImage_for_reflect(&self) -> &::std::string::String {
        &self.aliPayImage
    }

    fn mut_aliPayImage_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.aliPayImage
    }

    // repeated .proto.RewardResp.Rewards rewards = 5;

    pub fn clear_rewards(&mut self) {
        self.rewards.clear();
    }

    // Param is passed by value, moved
    pub fn set_rewards(&mut self, v: ::protobuf::RepeatedField<RewardResp_Rewards>) {
        self.rewards = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rewards(&mut self) -> &mut ::protobuf::RepeatedField<RewardResp_Rewards> {
        &mut self.rewards
    }

    // Take field
    pub fn take_rewards(&mut self) -> ::protobuf::RepeatedField<RewardResp_Rewards> {
        ::std::mem::replace(&mut self.rewards, ::protobuf::RepeatedField::new())
    }

    pub fn get_rewards(&self) -> &[RewardResp_Rewards] {
        &self.rewards
    }

    fn get_rewards_for_reflect(&self) -> &::protobuf::RepeatedField<RewardResp_Rewards> {
        &self.rewards
    }

    fn mut_rewards_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RewardResp_Rewards> {
        &mut self.rewards
    }
}

impl ::protobuf::Message for RewardResp {
    fn is_initialized(&self) -> bool {
        for v in &self.rewards {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.wechatImage)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.aliPayImage)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rewards)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.wechatImage.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.wechatImage);
        }
        if !self.aliPayImage.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.aliPayImage);
        }
        for value in &self.rewards {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.wechatImage.is_empty() {
            os.write_string(3, &self.wechatImage)?;
        }
        if !self.aliPayImage.is_empty() {
            os.write_string(4, &self.aliPayImage)?;
        }
        for v in &self.rewards {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RewardResp {
    fn new() -> RewardResp {
        RewardResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RewardResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    RewardResp::get_code_for_reflect,
                    RewardResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    RewardResp::get_msg_for_reflect,
                    RewardResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "wechatImage",
                    RewardResp::get_wechatImage_for_reflect,
                    RewardResp::mut_wechatImage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "aliPayImage",
                    RewardResp::get_aliPayImage_for_reflect,
                    RewardResp::mut_aliPayImage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RewardResp_Rewards>>(
                    "rewards",
                    RewardResp::get_rewards_for_reflect,
                    RewardResp::mut_rewards_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RewardResp>(
                    "RewardResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RewardResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_wechatImage();
        self.clear_aliPayImage();
        self.clear_rewards();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RewardResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RewardResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RewardResp_Rewards {
    // message fields
    pub payTime: ::std::string::String,
    pub name: ::std::string::String,
    pub money: ::std::string::String,
    pub source: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RewardResp_Rewards {}

impl RewardResp_Rewards {
    pub fn new() -> RewardResp_Rewards {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RewardResp_Rewards {
        static mut instance: ::protobuf::lazy::Lazy<RewardResp_Rewards> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RewardResp_Rewards,
        };
        unsafe {
            instance.get(RewardResp_Rewards::new)
        }
    }

    // string payTime = 1;

    pub fn clear_payTime(&mut self) {
        self.payTime.clear();
    }

    // Param is passed by value, moved
    pub fn set_payTime(&mut self, v: ::std::string::String) {
        self.payTime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payTime(&mut self) -> &mut ::std::string::String {
        &mut self.payTime
    }

    // Take field
    pub fn take_payTime(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payTime, ::std::string::String::new())
    }

    pub fn get_payTime(&self) -> &str {
        &self.payTime
    }

    fn get_payTime_for_reflect(&self) -> &::std::string::String {
        &self.payTime
    }

    fn mut_payTime_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payTime
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string money = 3;

    pub fn clear_money(&mut self) {
        self.money.clear();
    }

    // Param is passed by value, moved
    pub fn set_money(&mut self, v: ::std::string::String) {
        self.money = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_money(&mut self) -> &mut ::std::string::String {
        &mut self.money
    }

    // Take field
    pub fn take_money(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.money, ::std::string::String::new())
    }

    pub fn get_money(&self) -> &str {
        &self.money
    }

    fn get_money_for_reflect(&self) -> &::std::string::String {
        &self.money
    }

    fn mut_money_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.money
    }

    // string source = 4;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        &mut self.source
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.source, ::std::string::String::new())
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    fn get_source_for_reflect(&self) -> &::std::string::String {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.source
    }
}

impl ::protobuf::Message for RewardResp_Rewards {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payTime)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.money)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.source)?;
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
        if !self.payTime.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.payTime);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.money.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.money);
        }
        if !self.source.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.source);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.payTime.is_empty() {
            os.write_string(1, &self.payTime)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.money.is_empty() {
            os.write_string(3, &self.money)?;
        }
        if !self.source.is_empty() {
            os.write_string(4, &self.source)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RewardResp_Rewards {
    fn new() -> RewardResp_Rewards {
        RewardResp_Rewards::new()
    }

    fn descriptor_static(_: ::std::option::Option<RewardResp_Rewards>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payTime",
                    RewardResp_Rewards::get_payTime_for_reflect,
                    RewardResp_Rewards::mut_payTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    RewardResp_Rewards::get_name_for_reflect,
                    RewardResp_Rewards::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "money",
                    RewardResp_Rewards::get_money_for_reflect,
                    RewardResp_Rewards::mut_money_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source",
                    RewardResp_Rewards::get_source_for_reflect,
                    RewardResp_Rewards::mut_source_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RewardResp_Rewards>(
                    "RewardResp_Rewards",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RewardResp_Rewards {
    fn clear(&mut self) {
        self.clear_payTime();
        self.clear_name();
        self.clear_money();
        self.clear_source();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RewardResp_Rewards {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RewardResp_Rewards {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QiNiuListResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub total: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QiNiuListResp {}

impl QiNiuListResp {
    pub fn new() -> QiNiuListResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QiNiuListResp {
        static mut instance: ::protobuf::lazy::Lazy<QiNiuListResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QiNiuListResp,
        };
        unsafe {
            instance.get(QiNiuListResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // uint32 total = 3;

    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u32) {
        self.total = v;
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    fn get_total_for_reflect(&self) -> &u32 {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut u32 {
        &mut self.total
    }
}

impl ::protobuf::Message for QiNiuListResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total = tmp;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if self.total != 0 {
            os.write_uint32(3, self.total)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QiNiuListResp {
    fn new() -> QiNiuListResp {
        QiNiuListResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<QiNiuListResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    QiNiuListResp::get_code_for_reflect,
                    QiNiuListResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    QiNiuListResp::get_msg_for_reflect,
                    QiNiuListResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total",
                    QiNiuListResp::get_total_for_reflect,
                    QiNiuListResp::mut_total_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QiNiuListResp>(
                    "QiNiuListResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QiNiuListResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_total();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QiNiuListResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QiNiuListResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QiNiuListResp_qiNiuItem {
    // message fields
    pub name: ::std::string::String,
    pub url: ::std::string::String,
    pub createTime: ::std::string::String,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QiNiuListResp_qiNiuItem {}

impl QiNiuListResp_qiNiuItem {
    pub fn new() -> QiNiuListResp_qiNiuItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QiNiuListResp_qiNiuItem {
        static mut instance: ::protobuf::lazy::Lazy<QiNiuListResp_qiNiuItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QiNiuListResp_qiNiuItem,
        };
        unsafe {
            instance.get(QiNiuListResp_qiNiuItem::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string url = 2;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.url, ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    fn get_url_for_reflect(&self) -> &::std::string::String {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.url
    }

    // string createTime = 3;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.createTime, ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        &self.createTime
    }

    fn get_createTime_for_reflect(&self) -> &::std::string::String {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.createTime
    }

    // string id = 4;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for QiNiuListResp_qiNiuItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.url)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.createTime)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.url);
        }
        if !self.createTime.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.createTime);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.url.is_empty() {
            os.write_string(2, &self.url)?;
        }
        if !self.createTime.is_empty() {
            os.write_string(3, &self.createTime)?;
        }
        if !self.id.is_empty() {
            os.write_string(4, &self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QiNiuListResp_qiNiuItem {
    fn new() -> QiNiuListResp_qiNiuItem {
        QiNiuListResp_qiNiuItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<QiNiuListResp_qiNiuItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    QiNiuListResp_qiNiuItem::get_name_for_reflect,
                    QiNiuListResp_qiNiuItem::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    QiNiuListResp_qiNiuItem::get_url_for_reflect,
                    QiNiuListResp_qiNiuItem::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    QiNiuListResp_qiNiuItem::get_createTime_for_reflect,
                    QiNiuListResp_qiNiuItem::mut_createTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    QiNiuListResp_qiNiuItem::get_id_for_reflect,
                    QiNiuListResp_qiNiuItem::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QiNiuListResp_qiNiuItem>(
                    "QiNiuListResp_qiNiuItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QiNiuListResp_qiNiuItem {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_url();
        self.clear_createTime();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QiNiuListResp_qiNiuItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QiNiuListResp_qiNiuItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogResp {
    // message fields
    pub code: u32,
    pub msg: ::std::string::String,
    pub rows: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogResp {}

impl LogResp {
    pub fn new() -> LogResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogResp {
        static mut instance: ::protobuf::lazy::Lazy<LogResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogResp,
        };
        unsafe {
            instance.get(LogResp::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // repeated string rows = 3;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[::std::string::String] {
        &self.rows
    }

    fn get_rows_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.rows
    }

    fn mut_rows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.rows
    }
}

impl ::protobuf::Message for LogResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.rows)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        for value in &self.rows {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        for v in &self.rows {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogResp {
    fn new() -> LogResp {
        LogResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    LogResp::get_code_for_reflect,
                    LogResp::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    LogResp::get_msg_for_reflect,
                    LogResp::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rows",
                    LogResp::get_rows_for_reflect,
                    LogResp::mut_rows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogResp>(
                    "LogResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogResp {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tall.proto\x12\x05proto\"0\n\x08BaseResp\x12\x12\n\x04code\x18\x01\
    \x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\"o\n\
    \x11LoginAdminRequest\x12\x1a\n\x08username\x18\x01\x20\x01(\tR\x08usern\
    ame\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08password\x12\x12\n\x04c\
    ode\x18\x03\x20\x01(\tR\x04code\x12\x0e\n\x02id\x18\x04\x20\x01(\tR\x02i\
    d\"L\n\x0eLoginAdminResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\
    \x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x14\n\x05token\x18\x03\
    \x20\x01(\tR\x05token\"*\n\x12LogoutAdminRequest\x12\x14\n\x05token\x18\
    \x01\x20\x01(\tR\x05token\"\x9b\x04\n\rAdminInfoResp\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x12\x16\n\x06avatar\x18\x02\x20\x01(\tR\x06\
    avatar\x12\x10\n\x03job\x18\x03\x20\x01(\tR\x03job\x12\"\n\x0corganizati\
    on\x18\x04\x20\x01(\tR\x0corganization\x12\x1a\n\x08location\x18\x05\x20\
    \x01(\tR\x08location\x12\x14\n\x05email\x18\x06\x20\x01(\tR\x05email\x12\
    \"\n\x0cintroduction\x18\x07\x20\x01(\tR\x0cintroduction\x12(\n\x0fperso\
    nalWebsite\x18\x08\x20\x01(\tR\x0fpersonalWebsite\x12\x18\n\x07jobName\
    \x18\t\x20\x01(\tR\x07jobName\x12*\n\x10organizationName\x18\n\x20\x01(\
    \tR\x10organizationName\x12\"\n\x0clocationName\x18\x0b\x20\x01(\tR\x0cl\
    ocationName\x12\x14\n\x05phone\x18\x0c\x20\x01(\tR\x05phone\x12*\n\x10re\
    gistrationDate\x18\r\x20\x01(\tR\x10registrationDate\x12\x1c\n\taccountI\
    d\x18\x0e\x20\x01(\tR\taccountId\x12$\n\rcertification\x18\x0f\x20\x01(\
    \tR\rcertification\x12\x12\n\x04role\x18\x10\x20\x01(\tR\x04role\x12\x12\
    \n\x04code\x18\x11\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x12\x20\x01(\
    \tR\x03msg\"\xbe\x02\n\x07Comment\x12\x0f\n\x03_id\x18\x01\x20\x01(\tR\
    \x02Id\x12\x16\n\x06avatar\x18\x02\x20\x01(\tR\x06avatar\x12\x1a\n\x08us\
    ername\x18\x03\x20\x01(\tR\x08username\x12\x14\n\x05label\x18\x04\x20\
    \x01(\tR\x05label\x12\x1e\n\ncreateDate\x18\x05\x20\x01(\tR\ncreateDate\
    \x12\x18\n\x07content\x18\x06\x20\x01(\tR\x07content\x12*\n\x08children\
    \x18\x07\x20\x03(\x0b2\x0e.proto.CommentR\x08children\x12&\n\x0eparentUs\
    ername\x18\x08\x20\x01(\tR\x0eparentUsername\x12\x0e\n\x02ip\x18\t\x20\
    \x01(\tR\x02ip\x12\x0e\n\x02ua\x18\n\x20\x01(\tR\x02ua\x12\x1a\n\x08loca\
    tion\x18\x0b\x20\x01(\tR\x08location\x12\x0e\n\x02os\x18\x0c\x20\x01(\tR\
    \x02os\"\x88\x01\n\nPagination\x12\x1e\n\ncountTotal\x18\x01\x20\x01(\rR\
    \ncountTotal\x12\x1c\n\ttotalPage\x18\x02\x20\x01(\rR\ttotalPage\x12\x20\
    \n\x0bcurrentPage\x18\x03\x20\x01(\rR\x0bcurrentPage\x12\x1a\n\x08pageSi\
    ze\x18\x04\x20\x01(\rR\x08pageSize\"\x8e\x01\n\x0fCommentListResp\x12\
    \x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\
    \x01(\tR\x03msg\x12\"\n\x04list\x18\x04\x20\x03(\x0b2\x0e.proto.CommentR\
    \x04list\x121\n\npagination\x18\x05\x20\x01(\x0b2\x11.proto.PaginationR\
    \npagination\"g\n\x11CommentAddRequest\x12\x18\n\x07content\x18\x01\x20\
    \x01(\tR\x07content\x12\x1c\n\tarticleId\x18\x02\x20\x01(\tR\tarticleId\
    \x12\x1a\n\x08parentId\x18\x03\x20\x01(\tR\x08parentId\"Z\n\x0eCommentAd\
    dResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\
    \x02\x20\x01(\tR\x03msg\x12\"\n\x04data\x18\x03\x20\x01(\x0b2\x0e.proto.\
    CommentR\x04data\"-\n\x07CatchMe\x12\x10\n\x03git\x18\x01\x20\x01(\tR\
    \x03git\x12\x10\n\x03job\x18\x02\x20\x01(\tR\x03job\"\xa9\x01\n\tAboutRe\
    sp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\
    \x02\x20\x01(\tR\x03msg\x12\x18\n\x07likeNum\x18\x03\x20\x01(\tR\x07like\
    Num\x12(\n\x07catchMe\x18\x04\x20\x01(\x0b2\x0e.proto.CatchMeR\x07catchM\
    e\x12\"\n\x0cdescriptions\x18\x05\x20\x01(\tR\x0cdescriptions\x12\x0e\n\
    \x02id\x18\x06\x20\x01(\rR\x02id\"\x8b\x01\n\x0bSiteInfoReq\x12\x1c\n\tc\
    opyright\x18\x01\x20\x01(\tR\tcopyright\x12\"\n\x0cdescriptions\x18\x04\
    \x20\x01(\tR\x0cdescriptions\x12\x14\n\x05beian\x18\x05\x20\x01(\tR\x05b\
    eian\x12\x14\n\x05title\x18\x06\x20\x01(\tR\x05title\x12\x0e\n\x02id\x18\
    \x07\x20\x01(\rR\x02id\"\xf0\x01\n\x0cSiteInfoResp\x12\x12\n\x04code\x18\
    \x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\
    \x16\n\x06author\x18\x03\x20\x01(\tR\x06author\x12\x16\n\x06github\x18\
    \x04\x20\x01(\tR\x06github\x12\x14\n\x05beian\x18\x05\x20\x01(\tR\x05bei\
    an\x12\"\n\x0cdescriptions\x18\x06\x20\x01(\tR\x0cdescriptions\x12*\n\
    \x10selfDescriptions\x18\x07\x20\x01(\tR\x10selfDescriptions\x12\x0e\n\
    \x02id\x18\x08\x20\x01(\rR\x02id\x12\x14\n\x05title\x18\t\x20\x01(\tR\
    \x05title\"V\n\nBrowseList\x12\x1c\n\tarticleId\x18\x01\x20\x01(\tR\tart\
    icleId\x12\x14\n\x05title\x18\x02\x20\x01(\tR\x05title\x12\x14\n\x05coun\
    t\x18\x03\x20\x01(\rR\x05count\"\x92\x01\n\x0eTopCommentList\x12\x1c\n\t\
    articleId\x18\x01\x20\x01(\tR\tarticleId\x12\x16\n\x06avatar\x18\x02\x20\
    \x01(\tR\x06avatar\x12\x14\n\x05title\x18\x03\x20\x01(\tR\x05title\x12\
    \x1a\n\x08username\x18\x04\x20\x01(\tR\x08username\x12\x18\n\x07content\
    \x18\x05\x20\x01(\tR\x07content\"\xc6\x01\n\x0eTopCommentResp\x12\x12\n\
    \x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\t\
    R\x03msg\x12\x1c\n\tloveCount\x18\x03\x20\x01(\tR\tloveCount\x121\n\nbro\
    wseList\x18\x04\x20\x03(\x0b2\x11.proto.BrowseListR\nbrowseList\x12=\n\
    \x0etopCommentList\x18\x05\x20\x03(\x0b2\x15.proto.TopCommentListR\x0eto\
    pCommentList\"\xa8\x01\n\tClassList\x12\x14\n\x05count\x18\x01\x20\x01(\
    \rR\x05count\x12\x1e\n\ncreateDate\x18\x02\x20\x01(\tR\ncreateDate\x12*\
    \n\x10lastModifiedDate\x18\x03\x20\x01(\tR\x10lastModifiedDate\x12\x12\n\
    \x04name\x18\x04\x20\x01(\tR\x04name\x12\x14\n\x05state\x18\x05\x20\x01(\
    \rR\x05state\x12\x0f\n\x03_id\x18\x06\x20\x01(\rR\x02Id\"\x9f\x01\n\x08L\
    istYear\x12,\n\x04list\x18\x01\x20\x03(\x0b2\x18.proto.ListYear.ListItem\
    R\x04list\x12\x12\n\x04year\x18\x02\x20\x01(\tR\x04year\x1aQ\n\x08ListIt\
    em\x12\x1e\n\ncreateDate\x18\x01\x20\x01(\tR\ncreateDate\x12\x14\n\x05ti\
    tle\x18\x02\x20\x01(\tR\x05title\x12\x0f\n\x03_id\x18\x03\x20\x01(\rR\
    \x02Id\"\x96\x01\n\x0bListByClass\x12\x12\n\x04code\x18\x01\x20\x01(\rR\
    \x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x121\n\x0barticleLi\
    st\x18\x03\x20\x03(\x0b2\x0f.proto.ListYearR\x0barticleList\x12.\n\tclas\
    sList\x18\x04\x20\x03(\x0b2\x10.proto.ClassListR\tclassList\"\xaa\x03\n\
    \x07Article\x12\x20\n\x0bbrowseCount\x18\x01\x20\x01(\rR\x0bbrowseCount\
    \x12\x18\n\x07classId\x18\x02\x20\x01(\rR\x07classId\x12\"\n\x0ccollectC\
    ount\x18\x03\x20\x01(\rR\x0ccollectCount\x12\"\n\x0ccommentCount\x18\x04\
    \x20\x01(\rR\x0ccommentCount\x12\x18\n\x07content\x18\x05\x20\x01(\tR\
    \x07content\x12\x1e\n\ncreateDate\x18\x06\x20\x01(\tR\ncreateDate\x12\
    \x14\n\x05isHot\x18\x07\x20\x01(\x08R\x05isHot\x12\x20\n\x0bisRecommend\
    \x18\x08\x20\x01(\x08R\x0bisRecommend\x12*\n\x10lastModifiedDate\x18\t\
    \x20\x01(\tR\x10lastModifiedDate\x12\x1c\n\tlikeCount\x18\n\x20\x01(\rR\
    \tlikeCount\x12\x14\n\x05state\x18\x0b\x20\x01(\rR\x05state\x12\x0f\n\
    \x03_id\x18\x0c\x20\x01(\rR\x02Id\x12\x0e\n\x03__v\x18\r\x20\x01(\rR\x01\
    V\x12\x12\n\x04tags\x18\x0e\x20\x03(\rR\x04tags\x12\x14\n\x05title\x18\
    \x0f\x20\x01(\tR\x05title\"\x8e\x01\n\x0fArticleListResp\x12\x12\n\x04co\
    de\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03m\
    sg\x12\"\n\x04list\x18\x03\x20\x03(\x0b2\x0e.proto.ArticleR\x04list\x121\
    \n\npagination\x18\x04\x20\x01(\x0b2\x11.proto.PaginationR\npagination\"\
    X\n\x0eArticleOneResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\
    \x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x20\n\x03obj\x18\x03\x20\
    \x01(\x0b2\x0e.proto.ArticleR\x03obj\"U\n\x0bCaptchaResp\x12\x12\n\x04co\
    de\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03m\
    sg\x12\x0e\n\x02id\x18\x03\x20\x01(\tR\x02id\x12\x10\n\x03img\x18\x04\
    \x20\x01(\tR\x03img\"\x9d\x01\n\rComponentMeta\x12\x14\n\x05title\x18\
    \x01\x20\x01(\tR\x05title\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\
    \x12\x12\n\x04icon\x18\x03\x20\x01(\tR\x04icon\x12\x18\n\x07noCache\x18\
    \x04\x20\x01(\x08R\x07noCache\x12\x14\n\x05affix\x18\x05\x20\x01(\x08R\
    \x05affix\x12\x1e\n\nactiveMenu\x18\x06\x20\x01(\tR\nactiveMenu\"\xc1\
    \x01\n\tComponent\x12\x1c\n\tcomponent\x18\x01\x20\x01(\tR\tcomponent\
    \x12\x12\n\x04path\x18\x03\x20\x01(\tR\x04path\x12\x12\n\x04name\x18\x04\
    \x20\x01(\tR\x04name\x12(\n\x04meta\x18\x05\x20\x01(\x0b2\x14.proto.Comp\
    onentMetaR\x04meta\x12\x16\n\x06hidden\x18\x06\x20\x01(\x08R\x06hidden\
    \x12,\n\x08children\x18\x07\x20\x03(\x0b2\x10.proto.ComponentR\x08childr\
    en\"]\n\x0fAdminRouterResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\
    \x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12$\n\x04data\x18\x03\x20\
    \x03(\x0b2\x10.proto.ComponentR\x04data\"k\n\x17AdminCategoryAddRequest\
    \x12\x14\n\x05title\x18\x01\x20\x01(\tR\x05title\x12\x20\n\x0bdescriptio\
    n\x18\x02\x20\x01(\tR\x0bdescription\x12\x18\n\x07support\x18\x03\x20\
    \x01(\x08R\x07support\"\xaf\x04\n\x15AdminCategoryListResp\x12\x12\n\x04\
    code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\
    \x03msg\x12\x14\n\x05total\x18\x03\x20\x01(\rR\x05total\x12=\n\x04rows\
    \x18\x04\x20\x03(\x0b2).proto.AdminCategoryListResp.categoryBaseR\x04row\
    s\x1a\xc4\x01\n\x08blogBase\x12\x14\n\x05title\x18\x01\x20\x01(\tR\x05ti\
    tle\x12\x18\n\x07summary\x18\x02\x20\x01(\tR\x07summary\x12\x1c\n\theade\
    rImg\x18\x03\x20\x01(\tR\theaderImg\x12\x18\n\x07comment\x18\x04\x20\x01\
    (\tR\x07comment\x12\x16\n\x06weight\x18\x05\x20\x01(\rR\x06weight\x12\
    \x18\n\x07support\x18\x06\x20\x01(\x08R\x07support\x12\x1e\n\ncreateTime\
    \x18\x07\x20\x01(\tR\ncreateTime\x1a\xd3\x01\n\x0ccategoryBase\x12\x14\n\
    \x05title\x18\x01\x20\x01(\tR\x05title\x12\x20\n\x0bdescription\x18\x02\
    \x20\x01(\tR\x0bdescription\x12\x1e\n\ncreateTime\x18\x03\x20\x01(\tR\nc\
    reateTime\x12\x18\n\x07support\x18\x04\x20\x01(\x08R\x07support\x12A\n\
    \x08blogList\x18\x05\x20\x03(\x0b2%.proto.AdminCategoryListResp.blogBase\
    R\x08blogList\x12\x0e\n\x02id\x18\x06\x20\x01(\rR\x02id\"\xb6\x02\n\x16A\
    dminArticleAddRequest\x12\x14\n\x05title\x18\x01\x20\x01(\tR\x05title\
    \x12\x18\n\x07summary\x18\x02\x20\x01(\tR\x07summary\x12\x1e\n\ncategory\
    Id\x18\x03\x20\x01(\rR\ncategoryId\x12\x18\n\x07support\x18\x04\x20\x01(\
    \x08R\x07support\x12\x18\n\x07comment\x18\x05\x20\x01(\x08R\x07comment\
    \x12$\n\rheaderImgType\x18\x06\x20\x01(\rR\rheaderImgType\x12\x1c\n\thea\
    derImg\x18\x07\x20\x01(\tR\theaderImg\x12\x16\n\x06weight\x18\x08\x20\
    \x01(\rR\x06weight\x12\"\n\x0ctagTitleList\x18\t\x20\x03(\tR\x0ctagTitle\
    List\x12\x18\n\x07content\x18\n\x20\x01(\tR\x07content\"\xb9\x04\n\x14Ad\
    minArticleListResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\
    \n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x14\n\x05total\x18\x03\x20\x01(\
    \rR\x05total\x12D\n\x04rows\x18\x04\x20\x03(\x0b20.proto.AdminArticleLis\
    tResp.adminArticleListBaseR\x04rows\x1aR\n\x18adminArticleListCategory\
    \x12\x14\n\x05title\x18\x01\x20\x01(\tR\x05title\x12\x20\n\x0bdescriptio\
    n\x18\x02\x20\x01(\tR\x0bdescription\x1a\xca\x02\n\x14adminArticleListBa\
    se\x12\x14\n\x05title\x18\x01\x20\x01(\tR\x05title\x12\x18\n\x07summary\
    \x18\x02\x20\x01(\tR\x07summary\x12\x1c\n\theaderImg\x18\x03\x20\x01(\tR\
    \theaderImg\x12\x18\n\x07comment\x18\x04\x20\x01(\x08R\x07comment\x12\
    \x16\n\x06weight\x18\x05\x20\x01(\rR\x06weight\x12\x18\n\x07support\x18\
    \x06\x20\x01(\x08R\x07support\x12\x1e\n\ncreateTime\x18\x07\x20\x01(\tR\
    \ncreateTime\x12\x0e\n\x02id\x18\x08\x20\x01(\rR\x02id\x12\x16\n\x06stat\
    us\x18\t\x20\x01(\x08R\x06status\x12P\n\x08category\x18\n\x20\x01(\x0b24\
    .proto.AdminArticleListResp.adminArticleListCategoryR\x08category\"\xe9\
    \x02\n\x13AdminArticleOneResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04c\
    ode\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x14\n\x05title\x18\
    \x03\x20\x01(\tR\x05title\x12\x18\n\x07summary\x18\x04\x20\x01(\tR\x07su\
    mmary\x12\x1e\n\ncategoryId\x18\x05\x20\x01(\rR\ncategoryId\x12\x18\n\
    \x07support\x18\x06\x20\x01(\x08R\x07support\x12\x18\n\x07comment\x18\
    \x07\x20\x01(\x08R\x07comment\x12$\n\rheaderImgType\x18\x08\x20\x01(\rR\
    \rheaderImgType\x12\x1c\n\theaderImg\x18\t\x20\x01(\tR\theaderImg\x12\
    \x16\n\x06weight\x18\n\x20\x01(\rR\x06weight\x12\"\n\x0ctagTitleList\x18\
    \x0b\x20\x03(\tR\x0ctagTitleList\x12\x18\n\x07content\x18\x0c\x20\x01(\t\
    R\x07content\x12\x0e\n\x02id\x18\r\x20\x01(\rR\x02id\"+\n\x04Tags\x12\
    \x0f\n\x03_id\x18\x01\x20\x01(\rR\x02Id\x12\x12\n\x04name\x18\x02\x20\
    \x01(\tR\x04name\"\xc9\x03\n\x0fListByClassResp\x12\x12\n\x04code\x18\
    \x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\
    >\n\tclassList\x18\x03\x20\x03(\x0b2\x20.proto.ListByClassResp.ClassList\
    R\tclassList\x12D\n\x0barticleList\x18\x04\x20\x03(\x0b2\".proto.ListByC\
    lassResp.ArticleListR\x0barticleList\x12\x1f\n\x04tags\x18\x05\x20\x03(\
    \x0b2\x0b.proto.TagsR\x04tags\x1aM\n\x04List\x12\x0f\n\x03_id\x18\x01\
    \x20\x01(\rR\x02Id\x12\x1e\n\ncreateDate\x18\x02\x20\x01(\tR\ncreateDate\
    \x12\x14\n\x05title\x18\x03\x20\x01(\tR\x05title\x1aF\n\tClassList\x12\
    \x0f\n\x03_id\x18\x01\x20\x01(\rR\x02Id\x12\x12\n\x04name\x18\x02\x20\
    \x01(\tR\x04name\x12\x14\n\x05count\x18\x03\x20\x01(\rR\x05count\x1aR\n\
    \x0bArticleList\x12\x12\n\x04year\x18\x01\x20\x01(\rR\x04year\x12/\n\x04\
    list\x18\x02\x20\x03(\x0b2\x1b.proto.ListByClassResp.ListR\x04list\"b\n\
    \x18AdminEditCategoryRequest\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\
    \x12\x14\n\x05title\x18\x02\x20\x01(\tR\x05title\x12\x20\n\x0bdescriptio\
    n\x18\x03\x20\x01(\tR\x0bdescription\"\xd2\x01\n\x08LinkBase\x12\x14\n\
    \x05title\x18\x01\x20\x01(\tR\x05title\x12\x20\n\x0bdescription\x18\x02\
    \x20\x01(\tR\x0bdescription\x12\x14\n\x05email\x18\x03\x20\x01(\tR\x05em\
    ail\x12\x10\n\x03url\x18\x04\x20\x01(\tR\x03url\x12\x1c\n\theaderImg\x18\
    \x05\x20\x01(\tR\theaderImg\x12\x18\n\x07display\x18\x06\x20\x01(\x08R\
    \x07display\x12\x0e\n\x02id\x18\x07\x20\x01(\rR\x02id\x12\x1e\n\ncreateT\
    ime\x18\x08\x20\x01(\tR\ncreateTime\"\xa5\x01\n\x0bLinkRequest\x12\x14\n\
    \x05title\x18\x03\x20\x01(\tR\x05title\x12\x20\n\x0bdescription\x18\x04\
    \x20\x01(\tR\x0bdescription\x12\x14\n\x05email\x18\x05\x20\x01(\tR\x05em\
    ail\x12\x10\n\x03url\x18\x06\x20\x01(\tR\x03url\x12\x1c\n\theaderImg\x18\
    \x07\x20\x01(\tR\theaderImg\x12\x18\n\x07display\x18\x08\x20\x01(\tR\x07\
    display\"o\n\x0cLinkListResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04co\
    de\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x14\n\x05total\x18\
    \x03\x20\x01(\rR\x05total\x12#\n\x04rows\x18\x04\x20\x03(\x0b2\x0f.proto\
    .LinkBaseR\x04rows\"\x92\x03\n\x0cUserInfoResp\x12\x12\n\x04code\x18\x01\
    \x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x16\
    \n\x06userId\x18\x03\x20\x01(\rR\x06userId\x12\x1a\n\x08username\x18\x04\
    \x20\x01(\tR\x08username\x12\x16\n\x06status\x18\x05\x20\x01(\rR\x06stat\
    us\x12\x16\n\x06avatar\x18\x06\x20\x01(\tR\x06avatar\x12\x1a\n\x08linkna\
    me\x18\x07\x20\x01(\tR\x08linkname\x12\x18\n\x07linkUrl\x18\x08\x20\x01(\
    \tR\x07linkUrl\x12\x1a\n\x08linkDesc\x18\t\x20\x01(\tR\x08linkDesc\x12\
    \x18\n\x07logoUrl\x18\n\x20\x01(\tR\x07logoUrl\x12\x14\n\x05state\x18\
    \x0b\x20\x01(\x08R\x05state\x12\x14\n\x05label\x18\x0c\x20\x01(\rR\x05la\
    bel\x12%\n\x0ereceive_update\x18\r\x20\x01(\x08R\rreceiveUpdate\x12\x14\
    \n\x05token\x18\x0e\x20\x01(\tR\x05token\x12#\n\rverify_status\x18\x0f\
    \x20\x01(\tR\x0cverifyStatus\"H\n\x0eUploadFileResp\x12\x12\n\x04code\
    \x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\
    \x12\x10\n\x03url\x18\x03\x20\x01(\tR\x03url\"\xec\x01\n\x13EditUserInfo\
    Request\x12\x16\n\x06userId\x18\x01\x20\x01(\rR\x06userId\x12\x14\n\x05l\
    abel\x18\x02\x20\x01(\rR\x05label\x12\x14\n\x05state\x18\x03\x20\x01(\
    \x08R\x05state\x12\x18\n\x07linkUrl\x18\x04\x20\x01(\tR\x07linkUrl\x12\
    \x1a\n\x08linkname\x18\x05\x20\x01(\tR\x08linkname\x12\x1a\n\x08linkDesc\
    \x18\x06\x20\x01(\tR\x08linkDesc\x12%\n\x0ereceive_update\x18\x07\x20\
    \x01(\x08R\rreceiveUpdate\x12\x18\n\x07logoUrl\x18\x08\x20\x01(\tR\x07lo\
    goUrl\"`\n\x12UpdateAboutRequest\x12\x18\n\x07content\x18\x01\x20\x01(\t\
    R\x07content\x12\x20\n\x0bhtmlContent\x18\x02\x20\x01(\tR\x0bhtmlContent\
    \x12\x0e\n\x02id\x18\x03\x20\x01(\rR\x02id\"\x96\x01\n\x0ePanelGroupResp\
    \x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\
    \x20\x01(\tR\x03msg\x12\x1c\n\tblogCount\x18\x03\x20\x01(\rR\tblogCount\
    \x12\"\n\x0cvisitorCount\x18\x04\x20\x01(\rR\x0cvisitorCount\x12\x1c\n\t\
    userCount\x18\x05\x20\x01(\rR\tuserCount\"\x99\x01\n\x11LineChartDataRes\
    p\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\
    \x20\x01(\tR\x03msg\x12\x1a\n\x08axisData\x18\x03\x20\x03(\rR\x08axisDat\
    a\x12\"\n\x0cexpectedData\x18\x04\x20\x03(\rR\x0cexpectedData\x12\x1e\n\
    \nactualData\x18\x05\x20\x03(\rR\nactualData\"R\n\x14LikeOrCollectReques\
    t\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x12\x12\n\x04flag\x18\x02\
    \x20\x01(\x08R\x04flag\x12\x16\n\x06isLike\x18\x03\x20\x01(\x08R\x06isLi\
    ke\"i\n\x13IsLikeOrCollectResp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04\
    code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x12\n\x04like\x18\
    \x03\x20\x01(\x08R\x04like\x12\x18\n\x07collect\x18\x04\x20\x01(\x08R\
    \x07collect\"\x92\x02\n\nRewardResp\x12\x12\n\x04code\x18\x01\x20\x01(\r\
    R\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg\x12\x20\n\x0bwecha\
    tImage\x18\x03\x20\x01(\tR\x0bwechatImage\x12\x20\n\x0baliPayImage\x18\
    \x04\x20\x01(\tR\x0baliPayImage\x123\n\x07rewards\x18\x05\x20\x03(\x0b2\
    \x19.proto.RewardResp.RewardsR\x07rewards\x1ae\n\x07Rewards\x12\x18\n\
    \x07payTime\x18\x01\x20\x01(\tR\x07payTime\x12\x12\n\x04name\x18\x02\x20\
    \x01(\tR\x04name\x12\x14\n\x05money\x18\x03\x20\x01(\tR\x05money\x12\x16\
    \n\x06source\x18\x04\x20\x01(\tR\x06source\"\xae\x01\n\rQiNiuListResp\
    \x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\x02\
    \x20\x01(\tR\x03msg\x12\x14\n\x05total\x18\x03\x20\x01(\rR\x05total\x1aa\
    \n\tqiNiuItem\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x10\n\
    \x03url\x18\x02\x20\x01(\tR\x03url\x12\x1e\n\ncreateTime\x18\x03\x20\x01\
    (\tR\ncreateTime\x12\x0e\n\x02id\x18\x04\x20\x01(\tR\x02id\"C\n\x07LogRe\
    sp\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03msg\x18\
    \x02\x20\x01(\tR\x03msg\x12\x12\n\x04rows\x18\x03\x20\x03(\tR\x04rowsB\n\
    Z\x08../protoJ\x8e\x9d\x01\n\x07\x12\x05\0\0\xed\x03\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x08\x12\x03\x02\0\x1f\n\t\n\x02\x08\x0b\x12\
    \x03\x02\0\x1f\n\x08\n\x01\x02\x12\x03\x03\0\x0e\n\n\n\x02\x04\0\x12\x04\
    \x05\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x10\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x06\x02\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x02\
    \x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\t\r\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x06\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x02\x11\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x07\t\x0c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\x0f\
    \x10\n\n\n\x02\x04\x01\x12\x04\n\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \n\x08\x19\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x02\x16\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x0b\t\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x14\x15\n\x0b\n\x04\
    \x04\x01\x02\x01\x12\x03\x0c\x02\x16\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\
    \x03\x0c\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\t\x11\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\x0c\x14\x15\n\x0b\n\x04\x04\x01\x02\
    \x02\x12\x03\r\x02\x12\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\r\x02\x08\
    \n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\r\t\r\n\x0c\n\x05\x04\x01\x02\
    \x02\x03\x12\x03\r\x10\x11\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x0e\x02\
    \x10\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\
    \x01\x02\x03\x01\x12\x03\x0e\t\x0b\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\
    \x03\x0e\x0e\x0f\n\n\n\x02\x04\x02\x12\x04\x11\0\x15\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x11\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x02\
    \x12\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03\x12\t\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x12\
    \x10\x11\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x13\x02\x11\n\x0c\n\x05\x04\
    \x02\x02\x01\x05\x12\x03\x13\x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03\x13\t\x0c\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x13\x0f\x10\n\x0b\
    \n\x04\x04\x02\x02\x02\x12\x03\x14\x02\x13\n\x0c\n\x05\x04\x02\x02\x02\
    \x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x14\t\
    \x0e\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x14\x11\x12\n\n\n\x02\x04\
    \x03\x12\x04\x17\0\x19\x01\n\n\n\x03\x04\x03\x01\x12\x03\x17\x08\x1a\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x03\x18\x02\x13\n\x0c\n\x05\x04\x03\x02\0\
    \x05\x12\x03\x18\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x18\t\x0e\
    \n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x18\x11\x12\n\n\n\x02\x04\x04\x12\
    \x04\x1b\0.\x01\n\n\n\x03\x04\x04\x01\x12\x03\x1b\x08\x15\n\x0b\n\x04\
    \x04\x04\x02\0\x12\x03\x1c\x02\x12\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03\
    \x1c\x02\x08\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x1c\t\r\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03\x1c\x10\x11\n\x0b\n\x04\x04\x04\x02\x01\x12\
    \x03\x1d\x02\x14\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x1d\x02\x08\n\
    \x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1d\t\x0f\n\x0c\n\x05\x04\x04\x02\
    \x01\x03\x12\x03\x1d\x12\x13\n\x0b\n\x04\x04\x04\x02\x02\x12\x03\x1e\x02\
    \x11\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03\x1e\x02\x08\n\x0c\n\x05\x04\
    \x04\x02\x02\x01\x12\x03\x1e\t\x0c\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\
    \x03\x1e\x0f\x10\n\x0b\n\x04\x04\x04\x02\x03\x12\x03\x1f\x02\x1a\n\x0c\n\
    \x05\x04\x04\x02\x03\x05\x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\x04\x02\x03\
    \x01\x12\x03\x1f\t\x15\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03\x1f\x18\
    \x19\n\x0b\n\x04\x04\x04\x02\x04\x12\x03\x20\x02\x16\n\x0c\n\x05\x04\x04\
    \x02\x04\x05\x12\x03\x20\x02\x08\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x03\
    \x20\t\x11\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03\x20\x14\x15\n\x0b\n\
    \x04\x04\x04\x02\x05\x12\x03!\x02\x13\n\x0c\n\x05\x04\x04\x02\x05\x05\
    \x12\x03!\x02\x08\n\x0c\n\x05\x04\x04\x02\x05\x01\x12\x03!\t\x0e\n\x0c\n\
    \x05\x04\x04\x02\x05\x03\x12\x03!\x11\x12\n\x0b\n\x04\x04\x04\x02\x06\
    \x12\x03\"\x02\x1a\n\x0c\n\x05\x04\x04\x02\x06\x05\x12\x03\"\x02\x08\n\
    \x0c\n\x05\x04\x04\x02\x06\x01\x12\x03\"\t\x15\n\x0c\n\x05\x04\x04\x02\
    \x06\x03\x12\x03\"\x18\x19\n\x0b\n\x04\x04\x04\x02\x07\x12\x03#\x02\x1d\
    \n\x0c\n\x05\x04\x04\x02\x07\x05\x12\x03#\x02\x08\n\x0c\n\x05\x04\x04\
    \x02\x07\x01\x12\x03#\t\x18\n\x0c\n\x05\x04\x04\x02\x07\x03\x12\x03#\x1b\
    \x1c\n\x0b\n\x04\x04\x04\x02\x08\x12\x03$\x02\x15\n\x0c\n\x05\x04\x04\
    \x02\x08\x05\x12\x03$\x02\x08\n\x0c\n\x05\x04\x04\x02\x08\x01\x12\x03$\t\
    \x10\n\x0c\n\x05\x04\x04\x02\x08\x03\x12\x03$\x13\x14\n\x0b\n\x04\x04\
    \x04\x02\t\x12\x03%\x02\x1f\n\x0c\n\x05\x04\x04\x02\t\x05\x12\x03%\x02\
    \x08\n\x0c\n\x05\x04\x04\x02\t\x01\x12\x03%\t\x19\n\x0c\n\x05\x04\x04\
    \x02\t\x03\x12\x03%\x1c\x1e\n\x0b\n\x04\x04\x04\x02\n\x12\x03&\x02\x1b\n\
    \x0c\n\x05\x04\x04\x02\n\x05\x12\x03&\x02\x08\n\x0c\n\x05\x04\x04\x02\n\
    \x01\x12\x03&\t\x15\n\x0c\n\x05\x04\x04\x02\n\x03\x12\x03&\x18\x1a\n\x0b\
    \n\x04\x04\x04\x02\x0b\x12\x03'\x02\x14\n\x0c\n\x05\x04\x04\x02\x0b\x05\
    \x12\x03'\x02\x08\n\x0c\n\x05\x04\x04\x02\x0b\x01\x12\x03'\t\x0e\n\x0c\n\
    \x05\x04\x04\x02\x0b\x03\x12\x03'\x11\x13\n\x0b\n\x04\x04\x04\x02\x0c\
    \x12\x03(\x02\x1f\n\x0c\n\x05\x04\x04\x02\x0c\x05\x12\x03(\x02\x08\n\x0c\
    \n\x05\x04\x04\x02\x0c\x01\x12\x03(\t\x19\n\x0c\n\x05\x04\x04\x02\x0c\
    \x03\x12\x03(\x1c\x1e\n\x0b\n\x04\x04\x04\x02\r\x12\x03)\x02\x18\n\x0c\n\
    \x05\x04\x04\x02\r\x05\x12\x03)\x02\x08\n\x0c\n\x05\x04\x04\x02\r\x01\
    \x12\x03)\t\x12\n\x0c\n\x05\x04\x04\x02\r\x03\x12\x03)\x15\x17\n\x0b\n\
    \x04\x04\x04\x02\x0e\x12\x03*\x02\x1c\n\x0c\n\x05\x04\x04\x02\x0e\x05\
    \x12\x03*\x02\x08\n\x0c\n\x05\x04\x04\x02\x0e\x01\x12\x03*\t\x16\n\x0c\n\
    \x05\x04\x04\x02\x0e\x03\x12\x03*\x19\x1b\n\x0b\n\x04\x04\x04\x02\x0f\
    \x12\x03+\x02\x13\n\x0c\n\x05\x04\x04\x02\x0f\x05\x12\x03+\x02\x08\n\x0c\
    \n\x05\x04\x04\x02\x0f\x01\x12\x03+\t\r\n\x0c\n\x05\x04\x04\x02\x0f\x03\
    \x12\x03+\x10\x12\n\x0b\n\x04\x04\x04\x02\x10\x12\x03,\x02\x13\n\x0c\n\
    \x05\x04\x04\x02\x10\x05\x12\x03,\x02\x08\n\x0c\n\x05\x04\x04\x02\x10\
    \x01\x12\x03,\t\r\n\x0c\n\x05\x04\x04\x02\x10\x03\x12\x03,\x10\x12\n\x0b\
    \n\x04\x04\x04\x02\x11\x12\x03-\x02\x12\n\x0c\n\x05\x04\x04\x02\x11\x05\
    \x12\x03-\x02\x08\n\x0c\n\x05\x04\x04\x02\x11\x01\x12\x03-\t\x0c\n\x0c\n\
    \x05\x04\x04\x02\x11\x03\x12\x03-\x0f\x11\n\n\n\x02\x04\x05\x12\x040\0>\
    \x01\n\n\n\x03\x04\x05\x01\x12\x030\x08\x0f\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x031\x02\x11\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x031\x02\x08\n\x0c\n\
    \x05\x04\x05\x02\0\x01\x12\x031\t\x0c\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x031\x0f\x10\n\x0b\n\x04\x04\x05\x02\x01\x12\x032\x02\x14\n\x0c\n\x05\
    \x04\x05\x02\x01\x05\x12\x032\x02\x08\n\x0c\n\x05\x04\x05\x02\x01\x01\
    \x12\x032\t\x0f\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x032\x12\x13\n\x0b\n\
    \x04\x04\x05\x02\x02\x12\x033\x02\x16\n\x0c\n\x05\x04\x05\x02\x02\x05\
    \x12\x033\x02\x08\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x033\t\x11\n\x0c\n\
    \x05\x04\x05\x02\x02\x03\x12\x033\x14\x15\n\x0b\n\x04\x04\x05\x02\x03\
    \x12\x034\x02\x13\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x034\x02\x08\n\x0c\
    \n\x05\x04\x05\x02\x03\x01\x12\x034\t\x0e\n\x0c\n\x05\x04\x05\x02\x03\
    \x03\x12\x034\x11\x12\n\x0b\n\x04\x04\x05\x02\x04\x12\x035\x02\x18\n\x0c\
    \n\x05\x04\x05\x02\x04\x05\x12\x035\x02\x08\n\x0c\n\x05\x04\x05\x02\x04\
    \x01\x12\x035\t\x13\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x035\x16\x17\n\
    \x0b\n\x04\x04\x05\x02\x05\x12\x036\x02\x15\n\x0c\n\x05\x04\x05\x02\x05\
    \x05\x12\x036\x02\x08\n\x0c\n\x05\x04\x05\x02\x05\x01\x12\x036\t\x10\n\
    \x0c\n\x05\x04\x05\x02\x05\x03\x12\x036\x13\x14\n\x0b\n\x04\x04\x05\x02\
    \x06\x12\x037\x02\x20\n\x0c\n\x05\x04\x05\x02\x06\x04\x12\x037\x02\n\n\
    \x0c\n\x05\x04\x05\x02\x06\x06\x12\x037\x0b\x12\n\x0c\n\x05\x04\x05\x02\
    \x06\x01\x12\x037\x13\x1b\n\x0c\n\x05\x04\x05\x02\x06\x03\x12\x037\x1e\
    \x1f\n\x0b\n\x04\x04\x05\x02\x07\x12\x038\x02\x1c\n\x0c\n\x05\x04\x05\
    \x02\x07\x05\x12\x038\x02\x08\n\x0c\n\x05\x04\x05\x02\x07\x01\x12\x038\t\
    \x17\n\x0c\n\x05\x04\x05\x02\x07\x03\x12\x038\x1a\x1b\n\x0b\n\x04\x04\
    \x05\x02\x08\x12\x039\x02\x10\n\x0c\n\x05\x04\x05\x02\x08\x05\x12\x039\
    \x02\x08\n\x0c\n\x05\x04\x05\x02\x08\x01\x12\x039\t\x0b\n\x0c\n\x05\x04\
    \x05\x02\x08\x03\x12\x039\x0e\x0f\n\x0b\n\x04\x04\x05\x02\t\x12\x03:\x02\
    \x11\n\x0c\n\x05\x04\x05\x02\t\x05\x12\x03:\x02\x08\n\x0c\n\x05\x04\x05\
    \x02\t\x01\x12\x03:\t\x0b\n\x0c\n\x05\x04\x05\x02\t\x03\x12\x03:\x0e\x10\
    \n\x0b\n\x04\x04\x05\x02\n\x12\x03;\x02\x17\n\x0c\n\x05\x04\x05\x02\n\
    \x05\x12\x03;\x02\x08\n\x0c\n\x05\x04\x05\x02\n\x01\x12\x03;\t\x11\n\x0c\
    \n\x05\x04\x05\x02\n\x03\x12\x03;\x14\x16\n\x0b\n\x04\x04\x05\x02\x0b\
    \x12\x03<\x02\x11\n\x0c\n\x05\x04\x05\x02\x0b\x05\x12\x03<\x02\x08\n\x0c\
    \n\x05\x04\x05\x02\x0b\x01\x12\x03<\t\x0b\n\x0c\n\x05\x04\x05\x02\x0b\
    \x03\x12\x03<\x0e\x10\n\n\n\x02\x04\x06\x12\x04@\0E\x01\n\n\n\x03\x04\
    \x06\x01\x12\x03@\x08\x12\n\x0b\n\x04\x04\x06\x02\0\x12\x03A\x02\x18\n\
    \x0c\n\x05\x04\x06\x02\0\x05\x12\x03A\x02\x08\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x03A\t\x13\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03A\x16\x17\n\x0b\
    \n\x04\x04\x06\x02\x01\x12\x03B\x02\x17\n\x0c\n\x05\x04\x06\x02\x01\x05\
    \x12\x03B\x02\x08\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03B\t\x12\n\x0c\n\
    \x05\x04\x06\x02\x01\x03\x12\x03B\x15\x16\n\x0b\n\x04\x04\x06\x02\x02\
    \x12\x03C\x02\x19\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x03C\x02\x08\n\x0c\
    \n\x05\x04\x06\x02\x02\x01\x12\x03C\t\x14\n\x0c\n\x05\x04\x06\x02\x02\
    \x03\x12\x03C\x17\x18\n\x0b\n\x04\x04\x06\x02\x03\x12\x03D\x02\x16\n\x0c\
    \n\x05\x04\x06\x02\x03\x05\x12\x03D\x02\x08\n\x0c\n\x05\x04\x06\x02\x03\
    \x01\x12\x03D\t\x11\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\x03D\x14\x15\n\n\
    \n\x02\x04\x07\x12\x04G\0N\x01\n\n\n\x03\x04\x07\x01\x12\x03G\x08\x17\n\
    \x0b\n\x04\x04\x07\x02\0\x12\x03H\x02\x12\n\x0c\n\x05\x04\x07\x02\0\x05\
    \x12\x03H\x02\x08\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03H\t\r\n\x0c\n\x05\
    \x04\x07\x02\0\x03\x12\x03H\x10\x11\n\x0b\n\x04\x04\x07\x02\x01\x12\x03I\
    \x02\x11\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03I\x02\x08\n\x0c\n\x05\
    \x04\x07\x02\x01\x01\x12\x03I\t\x0c\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\
    \x03I\x0f\x10\n\x0b\n\x04\x04\x07\x02\x02\x12\x03J\x02\x1c\n\x0c\n\x05\
    \x04\x07\x02\x02\x04\x12\x03J\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x06\x12\
    \x03J\x0b\x12\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03J\x13\x17\n\x0c\n\
    \x05\x04\x07\x02\x02\x03\x12\x03J\x1a\x1b\n\x0b\n\x04\x04\x07\x02\x03\
    \x12\x03K\x02\x1c\n\x0c\n\x05\x04\x07\x02\x03\x06\x12\x03K\x02\x0c\n\x0c\
    \n\x05\x04\x07\x02\x03\x01\x12\x03K\r\x17\n\x0c\n\x05\x04\x07\x02\x03\
    \x03\x12\x03K\x1a\x1b\n\n\n\x02\x04\x08\x12\x04P\0T\x01\n\n\n\x03\x04\
    \x08\x01\x12\x03P\x08\x19\n\x0b\n\x04\x04\x08\x02\0\x12\x03Q\x02\x15\n\
    \x0c\n\x05\x04\x08\x02\0\x05\x12\x03Q\x02\x08\n\x0c\n\x05\x04\x08\x02\0\
    \x01\x12\x03Q\t\x10\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03Q\x13\x14\n\x0b\
    \n\x04\x04\x08\x02\x01\x12\x03R\x02\x17\n\x0c\n\x05\x04\x08\x02\x01\x05\
    \x12\x03R\x02\x08\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03R\t\x12\n\x0c\n\
    \x05\x04\x08\x02\x01\x03\x12\x03R\x15\x16\n\x0b\n\x04\x04\x08\x02\x02\
    \x12\x03S\x02\x16\n\x0c\n\x05\x04\x08\x02\x02\x05\x12\x03S\x02\x08\n\x0c\
    \n\x05\x04\x08\x02\x02\x01\x12\x03S\t\x11\n\x0c\n\x05\x04\x08\x02\x02\
    \x03\x12\x03S\x14\x15\n\n\n\x02\x04\t\x12\x04V\0Z\x01\n\n\n\x03\x04\t\
    \x01\x12\x03V\x08\x16\n\x0b\n\x04\x04\t\x02\0\x12\x03W\x02\x12\n\x0c\n\
    \x05\x04\t\x02\0\x05\x12\x03W\x02\x08\n\x0c\n\x05\x04\t\x02\0\x01\x12\
    \x03W\t\r\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03W\x10\x11\n\x0b\n\x04\x04\t\
    \x02\x01\x12\x03X\x02\x11\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03X\x02\x08\
    \n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03X\t\x0c\n\x0c\n\x05\x04\t\x02\x01\
    \x03\x12\x03X\x0f\x10\n\x0b\n\x04\x04\t\x02\x02\x12\x03Y\x02\x13\n\x0c\n\
    \x05\x04\t\x02\x02\x06\x12\x03Y\x02\t\n\x0c\n\x05\x04\t\x02\x02\x01\x12\
    \x03Y\n\x0e\n\x0c\n\x05\x04\t\x02\x02\x03\x12\x03Y\x11\x12\n\n\n\x02\x04\
    \n\x12\x04\\\0`\x01\n\n\n\x03\x04\n\x01\x12\x03\\\x08\x0f\n\x0b\n\x04\
    \x04\n\x02\0\x12\x03]\x02\x11\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03]\x02\
    \x08\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03]\t\x0c\n\x0c\n\x05\x04\n\x02\0\
    \x03\x12\x03]\x0f\x10\n\x0b\n\x04\x04\n\x02\x01\x12\x03^\x02\x11\n\x0c\n\
    \x05\x04\n\x02\x01\x05\x12\x03^\x02\x08\n\x0c\n\x05\x04\n\x02\x01\x01\
    \x12\x03^\t\x0c\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03^\x0f\x10\n\n\n\x02\
    \x04\x0b\x12\x04a\0i\x01\n\n\n\x03\x04\x0b\x01\x12\x03a\x08\x11\n\x0b\n\
    \x04\x04\x0b\x02\0\x12\x03b\x02\x12\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\
    \x03b\x02\x08\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03b\t\r\n\x0c\n\x05\x04\
    \x0b\x02\0\x03\x12\x03b\x10\x11\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03c\x02\
    \x11\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03c\x02\x08\n\x0c\n\x05\x04\
    \x0b\x02\x01\x01\x12\x03c\t\x0c\n\x0c\n\x05\x04\x0b\x02\x01\x03\x12\x03c\
    \x0f\x10\n\x0b\n\x04\x04\x0b\x02\x02\x12\x03d\x02\x15\n\x0c\n\x05\x04\
    \x0b\x02\x02\x05\x12\x03d\x02\x08\n\x0c\n\x05\x04\x0b\x02\x02\x01\x12\
    \x03d\t\x10\n\x0c\n\x05\x04\x0b\x02\x02\x03\x12\x03d\x13\x14\n\x0b\n\x04\
    \x04\x0b\x02\x03\x12\x03e\x02\x16\n\x0c\n\x05\x04\x0b\x02\x03\x06\x12\
    \x03e\x02\t\n\x0c\n\x05\x04\x0b\x02\x03\x01\x12\x03e\n\x11\n\x0c\n\x05\
    \x04\x0b\x02\x03\x03\x12\x03e\x14\x15\n\x0b\n\x04\x04\x0b\x02\x04\x12\
    \x03f\x02\x1a\n\x0c\n\x05\x04\x0b\x02\x04\x05\x12\x03f\x02\x08\n\x0c\n\
    \x05\x04\x0b\x02\x04\x01\x12\x03f\t\x15\n\x0c\n\x05\x04\x0b\x02\x04\x03\
    \x12\x03f\x18\x19\n\x0b\n\x04\x04\x0b\x02\x05\x12\x03g\x02\x10\n\x0c\n\
    \x05\x04\x0b\x02\x05\x05\x12\x03g\x02\x08\n\x0c\n\x05\x04\x0b\x02\x05\
    \x01\x12\x03g\t\x0b\n\x0c\n\x05\x04\x0b\x02\x05\x03\x12\x03g\x0e\x0f\n\n\
    \n\x02\x04\x0c\x12\x04k\0q\x01\n\n\n\x03\x04\x0c\x01\x12\x03k\x08\x13\n\
    \x0b\n\x04\x04\x0c\x02\0\x12\x03l\x04\x19\n\x0c\n\x05\x04\x0c\x02\0\x05\
    \x12\x03l\x04\n\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03l\x0b\x14\n\x0c\n\
    \x05\x04\x0c\x02\0\x03\x12\x03l\x17\x18\n\x0b\n\x04\x04\x0c\x02\x01\x12\
    \x03m\x04\x1c\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\x03m\x04\n\n\x0c\n\x05\
    \x04\x0c\x02\x01\x01\x12\x03m\x0b\x17\n\x0c\n\x05\x04\x0c\x02\x01\x03\
    \x12\x03m\x1a\x1b\n\x0b\n\x04\x04\x0c\x02\x02\x12\x03n\x04\x15\n\x0c\n\
    \x05\x04\x0c\x02\x02\x05\x12\x03n\x04\n\n\x0c\n\x05\x04\x0c\x02\x02\x01\
    \x12\x03n\x0b\x10\n\x0c\n\x05\x04\x0c\x02\x02\x03\x12\x03n\x13\x14\n\x0b\
    \n\x04\x04\x0c\x02\x03\x12\x03o\x04\x15\n\x0c\n\x05\x04\x0c\x02\x03\x05\
    \x12\x03o\x04\n\n\x0c\n\x05\x04\x0c\x02\x03\x01\x12\x03o\x0b\x10\n\x0c\n\
    \x05\x04\x0c\x02\x03\x03\x12\x03o\x13\x14\n\x0b\n\x04\x04\x0c\x02\x04\
    \x12\x03p\x04\x12\n\x0c\n\x05\x04\x0c\x02\x04\x05\x12\x03p\x04\n\n\x0c\n\
    \x05\x04\x0c\x02\x04\x01\x12\x03p\x0b\r\n\x0c\n\x05\x04\x0c\x02\x04\x03\
    \x12\x03p\x10\x11\n\n\n\x02\x04\r\x12\x04s\0}\x01\n\n\n\x03\x04\r\x01\
    \x12\x03s\x08\x14\n\x0b\n\x04\x04\r\x02\0\x12\x03t\x02\x12\n\x0c\n\x05\
    \x04\r\x02\0\x05\x12\x03t\x02\x08\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03t\t\
    \r\n\x0c\n\x05\x04\r\x02\0\x03\x12\x03t\x10\x11\n\x0b\n\x04\x04\r\x02\
    \x01\x12\x03u\x02\x11\n\x0c\n\x05\x04\r\x02\x01\x05\x12\x03u\x02\x08\n\
    \x0c\n\x05\x04\r\x02\x01\x01\x12\x03u\t\x0c\n\x0c\n\x05\x04\r\x02\x01\
    \x03\x12\x03u\x0f\x10\n\x0b\n\x04\x04\r\x02\x02\x12\x03v\x02\x14\n\x0c\n\
    \x05\x04\r\x02\x02\x05\x12\x03v\x02\x08\n\x0c\n\x05\x04\r\x02\x02\x01\
    \x12\x03v\t\x0f\n\x0c\n\x05\x04\r\x02\x02\x03\x12\x03v\x12\x13\n\x0b\n\
    \x04\x04\r\x02\x03\x12\x03w\x02\x14\n\x0c\n\x05\x04\r\x02\x03\x05\x12\
    \x03w\x02\x08\n\x0c\n\x05\x04\r\x02\x03\x01\x12\x03w\t\x0f\n\x0c\n\x05\
    \x04\r\x02\x03\x03\x12\x03w\x12\x13\n\x0b\n\x04\x04\r\x02\x04\x12\x03x\
    \x02\x13\n\x0c\n\x05\x04\r\x02\x04\x05\x12\x03x\x02\x08\n\x0c\n\x05\x04\
    \r\x02\x04\x01\x12\x03x\t\x0e\n\x0c\n\x05\x04\r\x02\x04\x03\x12\x03x\x11\
    \x12\n\x0b\n\x04\x04\r\x02\x05\x12\x03y\x02\x1a\n\x0c\n\x05\x04\r\x02\
    \x05\x05\x12\x03y\x02\x08\n\x0c\n\x05\x04\r\x02\x05\x01\x12\x03y\t\x15\n\
    \x0c\n\x05\x04\r\x02\x05\x03\x12\x03y\x18\x19\n\x0b\n\x04\x04\r\x02\x06\
    \x12\x03z\x02\x1e\n\x0c\n\x05\x04\r\x02\x06\x05\x12\x03z\x02\x08\n\x0c\n\
    \x05\x04\r\x02\x06\x01\x12\x03z\t\x19\n\x0c\n\x05\x04\r\x02\x06\x03\x12\
    \x03z\x1c\x1d\n\x0b\n\x04\x04\r\x02\x07\x12\x03{\x02\x10\n\x0c\n\x05\x04\
    \r\x02\x07\x05\x12\x03{\x02\x08\n\x0c\n\x05\x04\r\x02\x07\x01\x12\x03{\t\
    \x0b\n\x0c\n\x05\x04\r\x02\x07\x03\x12\x03{\x0e\x0f\n\x0b\n\x04\x04\r\
    \x02\x08\x12\x03|\x02\x13\n\x0c\n\x05\x04\r\x02\x08\x05\x12\x03|\x02\x08\
    \n\x0c\n\x05\x04\r\x02\x08\x01\x12\x03|\t\x0e\n\x0c\n\x05\x04\r\x02\x08\
    \x03\x12\x03|\x11\x12\n\x0b\n\x02\x04\x0e\x12\x05\x7f\0\x83\x01\x01\n\n\
    \n\x03\x04\x0e\x01\x12\x03\x7f\x08\x12\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\
    \x80\x01\x02\x17\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\x80\x01\x02\x08\n\r\
    \n\x05\x04\x0e\x02\0\x01\x12\x04\x80\x01\t\x12\n\r\n\x05\x04\x0e\x02\0\
    \x03\x12\x04\x80\x01\x15\x16\n\x0c\n\x04\x04\x0e\x02\x01\x12\x04\x81\x01\
    \x02\x13\n\r\n\x05\x04\x0e\x02\x01\x05\x12\x04\x81\x01\x02\x08\n\r\n\x05\
    \x04\x0e\x02\x01\x01\x12\x04\x81\x01\t\x0e\n\r\n\x05\x04\x0e\x02\x01\x03\
    \x12\x04\x81\x01\x11\x12\n\x0c\n\x04\x04\x0e\x02\x02\x12\x04\x82\x01\x02\
    \x13\n\r\n\x05\x04\x0e\x02\x02\x05\x12\x04\x82\x01\x02\x08\n\r\n\x05\x04\
    \x0e\x02\x02\x01\x12\x04\x82\x01\t\x0e\n\r\n\x05\x04\x0e\x02\x02\x03\x12\
    \x04\x82\x01\x11\x12\n\x0c\n\x02\x04\x0f\x12\x06\x85\x01\0\x8b\x01\x01\n\
    \x0b\n\x03\x04\x0f\x01\x12\x04\x85\x01\x08\x16\n\x0c\n\x04\x04\x0f\x02\0\
    \x12\x04\x86\x01\x02\x17\n\r\n\x05\x04\x0f\x02\0\x05\x12\x04\x86\x01\x02\
    \x08\n\r\n\x05\x04\x0f\x02\0\x01\x12\x04\x86\x01\t\x12\n\r\n\x05\x04\x0f\
    \x02\0\x03\x12\x04\x86\x01\x15\x16\n\x0c\n\x04\x04\x0f\x02\x01\x12\x04\
    \x87\x01\x02\x14\n\r\n\x05\x04\x0f\x02\x01\x05\x12\x04\x87\x01\x02\x08\n\
    \r\n\x05\x04\x0f\x02\x01\x01\x12\x04\x87\x01\t\x0f\n\r\n\x05\x04\x0f\x02\
    \x01\x03\x12\x04\x87\x01\x12\x13\n\x0c\n\x04\x04\x0f\x02\x02\x12\x04\x88\
    \x01\x02\x13\n\r\n\x05\x04\x0f\x02\x02\x05\x12\x04\x88\x01\x02\x08\n\r\n\
    \x05\x04\x0f\x02\x02\x01\x12\x04\x88\x01\t\x0e\n\r\n\x05\x04\x0f\x02\x02\
    \x03\x12\x04\x88\x01\x11\x12\n\x0c\n\x04\x04\x0f\x02\x03\x12\x04\x89\x01\
    \x02\x16\n\r\n\x05\x04\x0f\x02\x03\x05\x12\x04\x89\x01\x02\x08\n\r\n\x05\
    \x04\x0f\x02\x03\x01\x12\x04\x89\x01\t\x11\n\r\n\x05\x04\x0f\x02\x03\x03\
    \x12\x04\x89\x01\x14\x15\n\x0c\n\x04\x04\x0f\x02\x04\x12\x04\x8a\x01\x02\
    \x15\n\r\n\x05\x04\x0f\x02\x04\x05\x12\x04\x8a\x01\x02\x08\n\r\n\x05\x04\
    \x0f\x02\x04\x01\x12\x04\x8a\x01\t\x10\n\r\n\x05\x04\x0f\x02\x04\x03\x12\
    \x04\x8a\x01\x13\x14\n\x0c\n\x02\x04\x10\x12\x06\x8d\x01\0\x93\x01\x01\n\
    \x0b\n\x03\x04\x10\x01\x12\x04\x8d\x01\x08\x16\n\x0c\n\x04\x04\x10\x02\0\
    \x12\x04\x8e\x01\x02\x12\n\r\n\x05\x04\x10\x02\0\x05\x12\x04\x8e\x01\x02\
    \x08\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\x8e\x01\t\r\n\r\n\x05\x04\x10\
    \x02\0\x03\x12\x04\x8e\x01\x10\x11\n\x0c\n\x04\x04\x10\x02\x01\x12\x04\
    \x8f\x01\x02\x11\n\r\n\x05\x04\x10\x02\x01\x05\x12\x04\x8f\x01\x02\x08\n\
    \r\n\x05\x04\x10\x02\x01\x01\x12\x04\x8f\x01\t\x0c\n\r\n\x05\x04\x10\x02\
    \x01\x03\x12\x04\x8f\x01\x0f\x10\n\x0c\n\x04\x04\x10\x02\x02\x12\x04\x90\
    \x01\x02\x17\n\r\n\x05\x04\x10\x02\x02\x05\x12\x04\x90\x01\x02\x08\n\r\n\
    \x05\x04\x10\x02\x02\x01\x12\x04\x90\x01\t\x12\n\r\n\x05\x04\x10\x02\x02\
    \x03\x12\x04\x90\x01\x15\x16\n\x0c\n\x04\x04\x10\x02\x03\x12\x04\x91\x01\
    \x02%\n\r\n\x05\x04\x10\x02\x03\x04\x12\x04\x91\x01\x02\n\n\r\n\x05\x04\
    \x10\x02\x03\x06\x12\x04\x91\x01\x0b\x15\n\r\n\x05\x04\x10\x02\x03\x01\
    \x12\x04\x91\x01\x16\x20\n\r\n\x05\x04\x10\x02\x03\x03\x12\x04\x91\x01#$\
    \n\x0c\n\x04\x04\x10\x02\x04\x12\x04\x92\x01\x02-\n\r\n\x05\x04\x10\x02\
    \x04\x04\x12\x04\x92\x01\x02\n\n\r\n\x05\x04\x10\x02\x04\x06\x12\x04\x92\
    \x01\x0b\x19\n\r\n\x05\x04\x10\x02\x04\x01\x12\x04\x92\x01\x1a(\n\r\n\
    \x05\x04\x10\x02\x04\x03\x12\x04\x92\x01+,\n\x0c\n\x02\x04\x11\x12\x06\
    \x95\x01\0\x9c\x01\x01\n\x0b\n\x03\x04\x11\x01\x12\x04\x95\x01\x08\x11\n\
    \x0c\n\x04\x04\x11\x02\0\x12\x04\x96\x01\x02\x13\n\r\n\x05\x04\x11\x02\0\
    \x05\x12\x04\x96\x01\x02\x08\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\x96\x01\
    \t\x0e\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\x96\x01\x11\x12\n\x0c\n\x04\
    \x04\x11\x02\x01\x12\x04\x97\x01\x02\x18\n\r\n\x05\x04\x11\x02\x01\x05\
    \x12\x04\x97\x01\x02\x08\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\x97\x01\t\
    \x13\n\r\n\x05\x04\x11\x02\x01\x03\x12\x04\x97\x01\x16\x17\n\x0c\n\x04\
    \x04\x11\x02\x02\x12\x04\x98\x01\x02\x1e\n\r\n\x05\x04\x11\x02\x02\x05\
    \x12\x04\x98\x01\x02\x08\n\r\n\x05\x04\x11\x02\x02\x01\x12\x04\x98\x01\t\
    \x19\n\r\n\x05\x04\x11\x02\x02\x03\x12\x04\x98\x01\x1c\x1d\n\x0c\n\x04\
    \x04\x11\x02\x03\x12\x04\x99\x01\x02\x12\n\r\n\x05\x04\x11\x02\x03\x05\
    \x12\x04\x99\x01\x02\x08\n\r\n\x05\x04\x11\x02\x03\x01\x12\x04\x99\x01\t\
    \r\n\r\n\x05\x04\x11\x02\x03\x03\x12\x04\x99\x01\x10\x11\n\x0c\n\x04\x04\
    \x11\x02\x04\x12\x04\x9a\x01\x02\x13\n\r\n\x05\x04\x11\x02\x04\x05\x12\
    \x04\x9a\x01\x02\x08\n\r\n\x05\x04\x11\x02\x04\x01\x12\x04\x9a\x01\t\x0e\
    \n\r\n\x05\x04\x11\x02\x04\x03\x12\x04\x9a\x01\x11\x12\n\x0c\n\x04\x04\
    \x11\x02\x05\x12\x04\x9b\x01\x02\x11\n\r\n\x05\x04\x11\x02\x05\x05\x12\
    \x04\x9b\x01\x02\x08\n\r\n\x05\x04\x11\x02\x05\x01\x12\x04\x9b\x01\t\x0c\
    \n\r\n\x05\x04\x11\x02\x05\x03\x12\x04\x9b\x01\x0f\x10\n\x0c\n\x02\x04\
    \x12\x12\x06\x9e\x01\0\xa6\x01\x01\n\x0b\n\x03\x04\x12\x01\x12\x04\x9e\
    \x01\x08\x10\n\x0e\n\x04\x04\x12\x03\0\x12\x06\x9f\x01\x02\xa3\x01\x03\n\
    \r\n\x05\x04\x12\x03\0\x01\x12\x04\x9f\x01\n\x12\n\x0e\n\x06\x04\x12\x03\
    \0\x02\0\x12\x04\xa0\x01\x04\x1a\n\x0f\n\x07\x04\x12\x03\0\x02\0\x05\x12\
    \x04\xa0\x01\x04\n\n\x0f\n\x07\x04\x12\x03\0\x02\0\x01\x12\x04\xa0\x01\
    \x0b\x15\n\x0f\n\x07\x04\x12\x03\0\x02\0\x03\x12\x04\xa0\x01\x18\x19\n\
    \x0e\n\x06\x04\x12\x03\0\x02\x01\x12\x04\xa1\x01\x04\x15\n\x0f\n\x07\x04\
    \x12\x03\0\x02\x01\x05\x12\x04\xa1\x01\x04\n\n\x0f\n\x07\x04\x12\x03\0\
    \x02\x01\x01\x12\x04\xa1\x01\x0b\x10\n\x0f\n\x07\x04\x12\x03\0\x02\x01\
    \x03\x12\x04\xa1\x01\x13\x14\n\x0e\n\x06\x04\x12\x03\0\x02\x02\x12\x04\
    \xa2\x01\x04\x13\n\x0f\n\x07\x04\x12\x03\0\x02\x02\x05\x12\x04\xa2\x01\
    \x04\n\n\x0f\n\x07\x04\x12\x03\0\x02\x02\x01\x12\x04\xa2\x01\x0b\x0e\n\
    \x0f\n\x07\x04\x12\x03\0\x02\x02\x03\x12\x04\xa2\x01\x11\x12\n\x0c\n\x04\
    \x04\x12\x02\0\x12\x04\xa4\x01\x02\x1d\n\r\n\x05\x04\x12\x02\0\x04\x12\
    \x04\xa4\x01\x02\n\n\r\n\x05\x04\x12\x02\0\x06\x12\x04\xa4\x01\x0b\x13\n\
    \r\n\x05\x04\x12\x02\0\x01\x12\x04\xa4\x01\x14\x18\n\r\n\x05\x04\x12\x02\
    \0\x03\x12\x04\xa4\x01\x1b\x1c\n\x0c\n\x04\x04\x12\x02\x01\x12\x04\xa5\
    \x01\x02\x12\n\r\n\x05\x04\x12\x02\x01\x05\x12\x04\xa5\x01\x02\x08\n\r\n\
    \x05\x04\x12\x02\x01\x01\x12\x04\xa5\x01\t\r\n\r\n\x05\x04\x12\x02\x01\
    \x03\x12\x04\xa5\x01\x10\x11\n\x0c\n\x02\x04\x13\x12\x06\xa8\x01\0\xae\
    \x01\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\xa8\x01\x08\x13\n\x0c\n\x04\x04\
    \x13\x02\0\x12\x04\xa9\x01\x02\x12\n\r\n\x05\x04\x13\x02\0\x05\x12\x04\
    \xa9\x01\x02\x08\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\xa9\x01\t\r\n\r\n\
    \x05\x04\x13\x02\0\x03\x12\x04\xa9\x01\x10\x11\n\x0c\n\x04\x04\x13\x02\
    \x01\x12\x04\xaa\x01\x02\x11\n\r\n\x05\x04\x13\x02\x01\x05\x12\x04\xaa\
    \x01\x02\x08\n\r\n\x05\x04\x13\x02\x01\x01\x12\x04\xaa\x01\t\x0c\n\r\n\
    \x05\x04\x13\x02\x01\x03\x12\x04\xaa\x01\x0f\x10\n\x0c\n\x04\x04\x13\x02\
    \x02\x12\x04\xab\x01\x02$\n\r\n\x05\x04\x13\x02\x02\x04\x12\x04\xab\x01\
    \x02\n\n\r\n\x05\x04\x13\x02\x02\x06\x12\x04\xab\x01\x0b\x13\n\r\n\x05\
    \x04\x13\x02\x02\x01\x12\x04\xab\x01\x14\x1f\n\r\n\x05\x04\x13\x02\x02\
    \x03\x12\x04\xab\x01\"#\n\x0c\n\x04\x04\x13\x02\x03\x12\x04\xac\x01\x02#\
    \n\r\n\x05\x04\x13\x02\x03\x04\x12\x04\xac\x01\x02\n\n\r\n\x05\x04\x13\
    \x02\x03\x06\x12\x04\xac\x01\x0b\x14\n\r\n\x05\x04\x13\x02\x03\x01\x12\
    \x04\xac\x01\x15\x1e\n\r\n\x05\x04\x13\x02\x03\x03\x12\x04\xac\x01!\"\n\
    \x0c\n\x02\x04\x14\x12\x06\xb0\x01\0\xc0\x01\x01\n\x0b\n\x03\x04\x14\x01\
    \x12\x04\xb0\x01\x08\x0f\n\x0c\n\x04\x04\x14\x02\0\x12\x04\xb1\x01\x02\
    \x19\n\r\n\x05\x04\x14\x02\0\x05\x12\x04\xb1\x01\x02\x08\n\r\n\x05\x04\
    \x14\x02\0\x01\x12\x04\xb1\x01\t\x14\n\r\n\x05\x04\x14\x02\0\x03\x12\x04\
    \xb1\x01\x17\x18\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\xb2\x01\x02\x15\n\r\
    \n\x05\x04\x14\x02\x01\x05\x12\x04\xb2\x01\x02\x08\n\r\n\x05\x04\x14\x02\
    \x01\x01\x12\x04\xb2\x01\t\x10\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\xb2\
    \x01\x13\x14\n\x0c\n\x04\x04\x14\x02\x02\x12\x04\xb3\x01\x02\x1a\n\r\n\
    \x05\x04\x14\x02\x02\x05\x12\x04\xb3\x01\x02\x08\n\r\n\x05\x04\x14\x02\
    \x02\x01\x12\x04\xb3\x01\t\x15\n\r\n\x05\x04\x14\x02\x02\x03\x12\x04\xb3\
    \x01\x18\x19\n\x0c\n\x04\x04\x14\x02\x03\x12\x04\xb4\x01\x02\x1a\n\r\n\
    \x05\x04\x14\x02\x03\x05\x12\x04\xb4\x01\x02\x08\n\r\n\x05\x04\x14\x02\
    \x03\x01\x12\x04\xb4\x01\t\x15\n\r\n\x05\x04\x14\x02\x03\x03\x12\x04\xb4\
    \x01\x18\x19\n\x0c\n\x04\x04\x14\x02\x04\x12\x04\xb5\x01\x02\x15\n\r\n\
    \x05\x04\x14\x02\x04\x05\x12\x04\xb5\x01\x02\x08\n\r\n\x05\x04\x14\x02\
    \x04\x01\x12\x04\xb5\x01\t\x10\n\r\n\x05\x04\x14\x02\x04\x03\x12\x04\xb5\
    \x01\x13\x14\n\x0c\n\x04\x04\x14\x02\x05\x12\x04\xb6\x01\x02\x18\n\r\n\
    \x05\x04\x14\x02\x05\x05\x12\x04\xb6\x01\x02\x08\n\r\n\x05\x04\x14\x02\
    \x05\x01\x12\x04\xb6\x01\t\x13\n\r\n\x05\x04\x14\x02\x05\x03\x12\x04\xb6\
    \x01\x16\x17\n\x0c\n\x04\x04\x14\x02\x06\x12\x04\xb7\x01\x02\x12\n\r\n\
    \x05\x04\x14\x02\x06\x05\x12\x04\xb7\x01\x02\x06\n\r\n\x05\x04\x14\x02\
    \x06\x01\x12\x04\xb7\x01\x08\r\n\r\n\x05\x04\x14\x02\x06\x03\x12\x04\xb7\
    \x01\x10\x11\n\x0c\n\x04\x04\x14\x02\x07\x12\x04\xb8\x01\x02\x17\n\r\n\
    \x05\x04\x14\x02\x07\x05\x12\x04\xb8\x01\x02\x06\n\r\n\x05\x04\x14\x02\
    \x07\x01\x12\x04\xb8\x01\x07\x12\n\r\n\x05\x04\x14\x02\x07\x03\x12\x04\
    \xb8\x01\x15\x16\n\x0c\n\x04\x04\x14\x02\x08\x12\x04\xb9\x01\x02\x1e\n\r\
    \n\x05\x04\x14\x02\x08\x05\x12\x04\xb9\x01\x02\x08\n\r\n\x05\x04\x14\x02\
    \x08\x01\x12\x04\xb9\x01\t\x19\n\r\n\x05\x04\x14\x02\x08\x03\x12\x04\xb9\
    \x01\x1c\x1d\n\x0c\n\x04\x04\x14\x02\t\x12\x04\xba\x01\x02\x18\n\r\n\x05\
    \x04\x14\x02\t\x05\x12\x04\xba\x01\x02\x08\n\r\n\x05\x04\x14\x02\t\x01\
    \x12\x04\xba\x01\t\x12\n\r\n\x05\x04\x14\x02\t\x03\x12\x04\xba\x01\x15\
    \x17\n\x0c\n\x04\x04\x14\x02\n\x12\x04\xbb\x01\x02\x14\n\r\n\x05\x04\x14\
    \x02\n\x05\x12\x04\xbb\x01\x02\x08\n\r\n\x05\x04\x14\x02\n\x01\x12\x04\
    \xbb\x01\t\x0e\n\r\n\x05\x04\x14\x02\n\x03\x12\x04\xbb\x01\x11\x13\n\x0c\
    \n\x04\x04\x14\x02\x0b\x12\x04\xbc\x01\x02\x12\n\r\n\x05\x04\x14\x02\x0b\
    \x05\x12\x04\xbc\x01\x02\x08\n\r\n\x05\x04\x14\x02\x0b\x01\x12\x04\xbc\
    \x01\t\x0c\n\r\n\x05\x04\x14\x02\x0b\x03\x12\x04\xbc\x01\x0f\x11\n\x0c\n\
    \x04\x04\x14\x02\x0c\x12\x04\xbd\x01\x02\x12\n\r\n\x05\x04\x14\x02\x0c\
    \x05\x12\x04\xbd\x01\x02\x08\n\r\n\x05\x04\x14\x02\x0c\x01\x12\x04\xbd\
    \x01\t\x0c\n\r\n\x05\x04\x14\x02\x0c\x03\x12\x04\xbd\x01\x0f\x11\n\x0c\n\
    \x04\x04\x14\x02\r\x12\x04\xbe\x01\x02\x1c\n\r\n\x05\x04\x14\x02\r\x04\
    \x12\x04\xbe\x01\x02\n\n\r\n\x05\x04\x14\x02\r\x05\x12\x04\xbe\x01\x0b\
    \x11\n\r\n\x05\x04\x14\x02\r\x01\x12\x04\xbe\x01\x12\x16\n\r\n\x05\x04\
    \x14\x02\r\x03\x12\x04\xbe\x01\x19\x1b\n\x0c\n\x04\x04\x14\x02\x0e\x12\
    \x04\xbf\x01\x02\x14\n\r\n\x05\x04\x14\x02\x0e\x05\x12\x04\xbf\x01\x02\
    \x08\n\r\n\x05\x04\x14\x02\x0e\x01\x12\x04\xbf\x01\t\x0e\n\r\n\x05\x04\
    \x14\x02\x0e\x03\x12\x04\xbf\x01\x11\x13\n\x0c\n\x02\x04\x15\x12\x06\xc2\
    \x01\0\xc7\x01\x01\n\x0b\n\x03\x04\x15\x01\x12\x04\xc2\x01\x08\x17\n\x0c\
    \n\x04\x04\x15\x02\0\x12\x04\xc3\x01\x02\x12\n\r\n\x05\x04\x15\x02\0\x05\
    \x12\x04\xc3\x01\x02\x08\n\r\n\x05\x04\x15\x02\0\x01\x12\x04\xc3\x01\t\r\
    \n\r\n\x05\x04\x15\x02\0\x03\x12\x04\xc3\x01\x10\x11\n\x0c\n\x04\x04\x15\
    \x02\x01\x12\x04\xc4\x01\x02\x11\n\r\n\x05\x04\x15\x02\x01\x05\x12\x04\
    \xc4\x01\x02\x08\n\r\n\x05\x04\x15\x02\x01\x01\x12\x04\xc4\x01\t\x0c\n\r\
    \n\x05\x04\x15\x02\x01\x03\x12\x04\xc4\x01\x0f\x10\n\x0c\n\x04\x04\x15\
    \x02\x02\x12\x04\xc5\x01\x02\x1c\n\r\n\x05\x04\x15\x02\x02\x04\x12\x04\
    \xc5\x01\x02\n\n\r\n\x05\x04\x15\x02\x02\x06\x12\x04\xc5\x01\x0b\x12\n\r\
    \n\x05\x04\x15\x02\x02\x01\x12\x04\xc5\x01\x13\x17\n\r\n\x05\x04\x15\x02\
    \x02\x03\x12\x04\xc5\x01\x1a\x1b\n\x0c\n\x04\x04\x15\x02\x03\x12\x04\xc6\
    \x01\x02\x1c\n\r\n\x05\x04\x15\x02\x03\x06\x12\x04\xc6\x01\x02\x0c\n\r\n\
    \x05\x04\x15\x02\x03\x01\x12\x04\xc6\x01\r\x17\n\r\n\x05\x04\x15\x02\x03\
    \x03\x12\x04\xc6\x01\x1a\x1b\n\x0c\n\x02\x04\x16\x12\x06\xc9\x01\0\xcd\
    \x01\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\xc9\x01\x08\x16\n\x0c\n\x04\x04\
    \x16\x02\0\x12\x04\xca\x01\x02\x12\n\r\n\x05\x04\x16\x02\0\x05\x12\x04\
    \xca\x01\x02\x08\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xca\x01\t\r\n\r\n\
    \x05\x04\x16\x02\0\x03\x12\x04\xca\x01\x10\x11\n\x0c\n\x04\x04\x16\x02\
    \x01\x12\x04\xcb\x01\x02\x11\n\r\n\x05\x04\x16\x02\x01\x05\x12\x04\xcb\
    \x01\x02\x08\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\xcb\x01\t\x0c\n\r\n\
    \x05\x04\x16\x02\x01\x03\x12\x04\xcb\x01\x0f\x10\n\x0c\n\x04\x04\x16\x02\
    \x02\x12\x04\xcc\x01\x02\x12\n\r\n\x05\x04\x16\x02\x02\x06\x12\x04\xcc\
    \x01\x02\t\n\r\n\x05\x04\x16\x02\x02\x01\x12\x04\xcc\x01\n\r\n\r\n\x05\
    \x04\x16\x02\x02\x03\x12\x04\xcc\x01\x10\x11\n\x0c\n\x02\x04\x17\x12\x06\
    \xcf\x01\0\xd4\x01\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\xcf\x01\x08\x13\n\
    \x0c\n\x04\x04\x17\x02\0\x12\x04\xd0\x01\x02\x12\n\r\n\x05\x04\x17\x02\0\
    \x05\x12\x04\xd0\x01\x02\x08\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\xd0\x01\
    \t\r\n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xd0\x01\x10\x11\n\x0c\n\x04\x04\
    \x17\x02\x01\x12\x04\xd1\x01\x02\x11\n\r\n\x05\x04\x17\x02\x01\x05\x12\
    \x04\xd1\x01\x02\x08\n\r\n\x05\x04\x17\x02\x01\x01\x12\x04\xd1\x01\t\x0c\
    \n\r\n\x05\x04\x17\x02\x01\x03\x12\x04\xd1\x01\x0f\x10\n\x0c\n\x04\x04\
    \x17\x02\x02\x12\x04\xd2\x01\x02\x10\n\r\n\x05\x04\x17\x02\x02\x05\x12\
    \x04\xd2\x01\x02\x08\n\r\n\x05\x04\x17\x02\x02\x01\x12\x04\xd2\x01\t\x0b\
    \n\r\n\x05\x04\x17\x02\x02\x03\x12\x04\xd2\x01\x0e\x0f\n\x0c\n\x04\x04\
    \x17\x02\x03\x12\x04\xd3\x01\x02\x11\n\r\n\x05\x04\x17\x02\x03\x05\x12\
    \x04\xd3\x01\x02\x08\n\r\n\x05\x04\x17\x02\x03\x01\x12\x04\xd3\x01\t\x0c\
    \n\r\n\x05\x04\x17\x02\x03\x03\x12\x04\xd3\x01\x0f\x10\n\x0c\n\x02\x04\
    \x18\x12\x06\xd6\x01\0\xdd\x01\x01\n\x0b\n\x03\x04\x18\x01\x12\x04\xd6\
    \x01\x08\x15\n\x0c\n\x04\x04\x18\x02\0\x12\x04\xd7\x01\x02\x13\n\r\n\x05\
    \x04\x18\x02\0\x05\x12\x04\xd7\x01\x02\x08\n\r\n\x05\x04\x18\x02\0\x01\
    \x12\x04\xd7\x01\t\x0e\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xd7\x01\x11\
    \x12\n\x0c\n\x04\x04\x18\x02\x01\x12\x04\xd8\x01\x02\x12\n\r\n\x05\x04\
    \x18\x02\x01\x05\x12\x04\xd8\x01\x02\x08\n\r\n\x05\x04\x18\x02\x01\x01\
    \x12\x04\xd8\x01\t\r\n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\xd8\x01\x10\
    \x11\n\x0c\n\x04\x04\x18\x02\x02\x12\x04\xd9\x01\x02\x12\n\r\n\x05\x04\
    \x18\x02\x02\x05\x12\x04\xd9\x01\x02\x08\n\r\n\x05\x04\x18\x02\x02\x01\
    \x12\x04\xd9\x01\t\r\n\r\n\x05\x04\x18\x02\x02\x03\x12\x04\xd9\x01\x10\
    \x11\n\x0c\n\x04\x04\x18\x02\x03\x12\x04\xda\x01\x02\x13\n\r\n\x05\x04\
    \x18\x02\x03\x05\x12\x04\xda\x01\x02\x06\n\r\n\x05\x04\x18\x02\x03\x01\
    \x12\x04\xda\x01\x07\x0e\n\r\n\x05\x04\x18\x02\x03\x03\x12\x04\xda\x01\
    \x11\x12\n\x0c\n\x04\x04\x18\x02\x04\x12\x04\xdb\x01\x02\x12\n\r\n\x05\
    \x04\x18\x02\x04\x05\x12\x04\xdb\x01\x02\x06\n\r\n\x05\x04\x18\x02\x04\
    \x01\x12\x04\xdb\x01\x08\r\n\r\n\x05\x04\x18\x02\x04\x03\x12\x04\xdb\x01\
    \x10\x11\n\x0c\n\x04\x04\x18\x02\x05\x12\x04\xdc\x01\x02\x19\n\r\n\x05\
    \x04\x18\x02\x05\x05\x12\x04\xdc\x01\x02\x08\n\r\n\x05\x04\x18\x02\x05\
    \x01\x12\x04\xdc\x01\n\x14\n\r\n\x05\x04\x18\x02\x05\x03\x12\x04\xdc\x01\
    \x17\x18\n\x0c\n\x02\x04\x19\x12\x06\xdf\x01\0\xe6\x01\x01\n\x0b\n\x03\
    \x04\x19\x01\x12\x04\xdf\x01\x08\x11\n\x0c\n\x04\x04\x19\x02\0\x12\x04\
    \xe0\x01\x02\x17\n\r\n\x05\x04\x19\x02\0\x05\x12\x04\xe0\x01\x02\x08\n\r\
    \n\x05\x04\x19\x02\0\x01\x12\x04\xe0\x01\t\x12\n\r\n\x05\x04\x19\x02\0\
    \x03\x12\x04\xe0\x01\x15\x16\n\x0c\n\x04\x04\x19\x02\x01\x12\x04\xe1\x01\
    \x02\x12\n\r\n\x05\x04\x19\x02\x01\x05\x12\x04\xe1\x01\x02\x08\n\r\n\x05\
    \x04\x19\x02\x01\x01\x12\x04\xe1\x01\t\r\n\r\n\x05\x04\x19\x02\x01\x03\
    \x12\x04\xe1\x01\x10\x11\n\x0c\n\x04\x04\x19\x02\x02\x12\x04\xe2\x01\x02\
    \x12\n\r\n\x05\x04\x19\x02\x02\x05\x12\x04\xe2\x01\x02\x08\n\r\n\x05\x04\
    \x19\x02\x02\x01\x12\x04\xe2\x01\t\r\n\r\n\x05\x04\x19\x02\x02\x03\x12\
    \x04\xe2\x01\x10\x11\n\x0c\n\x04\x04\x19\x02\x03\x12\x04\xe3\x01\x02\x19\
    \n\r\n\x05\x04\x19\x02\x03\x06\x12\x04\xe3\x01\x02\x0f\n\r\n\x05\x04\x19\
    \x02\x03\x01\x12\x04\xe3\x01\x10\x14\n\r\n\x05\x04\x19\x02\x03\x03\x12\
    \x04\xe3\x01\x17\x18\n\x0c\n\x04\x04\x19\x02\x04\x12\x04\xe4\x01\x02\x12\
    \n\r\n\x05\x04\x19\x02\x04\x05\x12\x04\xe4\x01\x02\x06\n\r\n\x05\x04\x19\
    \x02\x04\x01\x12\x04\xe4\x01\x07\r\n\r\n\x05\x04\x19\x02\x04\x03\x12\x04\
    \xe4\x01\x10\x11\n\x0c\n\x04\x04\x19\x02\x05\x12\x04\xe5\x01\x02\"\n\r\n\
    \x05\x04\x19\x02\x05\x04\x12\x04\xe5\x01\x02\n\n\r\n\x05\x04\x19\x02\x05\
    \x06\x12\x04\xe5\x01\x0b\x14\n\r\n\x05\x04\x19\x02\x05\x01\x12\x04\xe5\
    \x01\x15\x1d\n\r\n\x05\x04\x19\x02\x05\x03\x12\x04\xe5\x01\x20!\n\x0c\n\
    \x02\x04\x1a\x12\x06\xe9\x01\0\xed\x01\x01\n\x0b\n\x03\x04\x1a\x01\x12\
    \x04\xe9\x01\x08\x17\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\xea\x01\x02\x12\n\
    \r\n\x05\x04\x1a\x02\0\x05\x12\x04\xea\x01\x02\x08\n\r\n\x05\x04\x1a\x02\
    \0\x01\x12\x04\xea\x01\t\r\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xea\x01\
    \x10\x11\n\x0c\n\x04\x04\x1a\x02\x01\x12\x04\xeb\x01\x02\x11\n\r\n\x05\
    \x04\x1a\x02\x01\x05\x12\x04\xeb\x01\x02\x08\n\r\n\x05\x04\x1a\x02\x01\
    \x01\x12\x04\xeb\x01\t\x0c\n\r\n\x05\x04\x1a\x02\x01\x03\x12\x04\xeb\x01\
    \x0f\x10\n\x0c\n\x04\x04\x1a\x02\x02\x12\x04\xec\x01\x02\x1e\n\r\n\x05\
    \x04\x1a\x02\x02\x04\x12\x04\xec\x01\x02\n\n\r\n\x05\x04\x1a\x02\x02\x06\
    \x12\x04\xec\x01\x0b\x14\n\r\n\x05\x04\x1a\x02\x02\x01\x12\x04\xec\x01\
    \x15\x19\n\r\n\x05\x04\x1a\x02\x02\x03\x12\x04\xec\x01\x1c\x1d\n\x0c\n\
    \x02\x04\x1b\x12\x06\xef\x01\0\xf3\x01\x01\n\x0b\n\x03\x04\x1b\x01\x12\
    \x04\xef\x01\x08\x1f\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\xf0\x01\x02\x13\n\
    \r\n\x05\x04\x1b\x02\0\x05\x12\x04\xf0\x01\x02\x08\n\r\n\x05\x04\x1b\x02\
    \0\x01\x12\x04\xf0\x01\t\x0e\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xf0\x01\
    \x11\x12\n\x0c\n\x04\x04\x1b\x02\x01\x12\x04\xf1\x01\x02\x19\n\r\n\x05\
    \x04\x1b\x02\x01\x05\x12\x04\xf1\x01\x02\x08\n\r\n\x05\x04\x1b\x02\x01\
    \x01\x12\x04\xf1\x01\t\x14\n\r\n\x05\x04\x1b\x02\x01\x03\x12\x04\xf1\x01\
    \x17\x18\n\x0c\n\x04\x04\x1b\x02\x02\x12\x04\xf2\x01\x02\x13\n\r\n\x05\
    \x04\x1b\x02\x02\x05\x12\x04\xf2\x01\x02\x06\n\r\n\x05\x04\x1b\x02\x02\
    \x01\x12\x04\xf2\x01\x07\x0e\n\r\n\x05\x04\x1b\x02\x02\x03\x12\x04\xf2\
    \x01\x11\x12\n\x0c\n\x02\x04\x1c\x12\x06\xf5\x01\0\x8c\x02\x01\n\x0b\n\
    \x03\x04\x1c\x01\x12\x04\xf5\x01\x08\x1d\n\x0e\n\x04\x04\x1c\x03\0\x12\
    \x06\xf6\x01\x02\xff\x01\x03\n\r\n\x05\x04\x1c\x03\0\x01\x12\x04\xf6\x01\
    \n\x12\n\x0e\n\x06\x04\x1c\x03\0\x02\0\x12\x04\xf7\x01\x04\x15\n\x0f\n\
    \x07\x04\x1c\x03\0\x02\0\x05\x12\x04\xf7\x01\x04\n\n\x0f\n\x07\x04\x1c\
    \x03\0\x02\0\x01\x12\x04\xf7\x01\x0b\x10\n\x0f\n\x07\x04\x1c\x03\0\x02\0\
    \x03\x12\x04\xf7\x01\x13\x14\n\x0e\n\x06\x04\x1c\x03\0\x02\x01\x12\x04\
    \xf8\x01\x04\x17\n\x0f\n\x07\x04\x1c\x03\0\x02\x01\x05\x12\x04\xf8\x01\
    \x04\n\n\x0f\n\x07\x04\x1c\x03\0\x02\x01\x01\x12\x04\xf8\x01\x0b\x12\n\
    \x0f\n\x07\x04\x1c\x03\0\x02\x01\x03\x12\x04\xf8\x01\x15\x16\n\x0e\n\x06\
    \x04\x1c\x03\0\x02\x02\x12\x04\xf9\x01\x04\x19\n\x0f\n\x07\x04\x1c\x03\0\
    \x02\x02\x05\x12\x04\xf9\x01\x04\n\n\x0f\n\x07\x04\x1c\x03\0\x02\x02\x01\
    \x12\x04\xf9\x01\x0b\x14\n\x0f\n\x07\x04\x1c\x03\0\x02\x02\x03\x12\x04\
    \xf9\x01\x17\x18\n\x0e\n\x06\x04\x1c\x03\0\x02\x03\x12\x04\xfa\x01\x04\
    \x17\n\x0f\n\x07\x04\x1c\x03\0\x02\x03\x05\x12\x04\xfa\x01\x04\n\n\x0f\n\
    \x07\x04\x1c\x03\0\x02\x03\x01\x12\x04\xfa\x01\x0b\x12\n\x0f\n\x07\x04\
    \x1c\x03\0\x02\x03\x03\x12\x04\xfa\x01\x15\x16\n\x0e\n\x06\x04\x1c\x03\0\
    \x02\x04\x12\x04\xfb\x01\x04\x16\n\x0f\n\x07\x04\x1c\x03\0\x02\x04\x05\
    \x12\x04\xfb\x01\x04\n\n\x0f\n\x07\x04\x1c\x03\0\x02\x04\x01\x12\x04\xfb\
    \x01\x0b\x11\n\x0f\n\x07\x04\x1c\x03\0\x02\x04\x03\x12\x04\xfb\x01\x14\
    \x15\n\x0e\n\x06\x04\x1c\x03\0\x02\x05\x12\x04\xfc\x01\x04\x15\n\x0f\n\
    \x07\x04\x1c\x03\0\x02\x05\x05\x12\x04\xfc\x01\x04\x08\n\x0f\n\x07\x04\
    \x1c\x03\0\x02\x05\x01\x12\x04\xfc\x01\t\x10\n\x0f\n\x07\x04\x1c\x03\0\
    \x02\x05\x03\x12\x04\xfc\x01\x13\x14\n\x0e\n\x06\x04\x1c\x03\0\x02\x06\
    \x12\x04\xfd\x01\x04\x1a\n\x0f\n\x07\x04\x1c\x03\0\x02\x06\x05\x12\x04\
    \xfd\x01\x04\n\n\x0f\n\x07\x04\x1c\x03\0\x02\x06\x01\x12\x04\xfd\x01\x0b\
    \x15\n\x0f\n\x07\x04\x1c\x03\0\x02\x06\x03\x12\x04\xfd\x01\x18\x19\n\x0e\
    \n\x04\x04\x1c\x03\x01\x12\x06\x80\x02\x02\x87\x02\x03\n\r\n\x05\x04\x1c\
    \x03\x01\x01\x12\x04\x80\x02\n\x16\n\x0e\n\x06\x04\x1c\x03\x01\x02\0\x12\
    \x04\x81\x02\x06\x17\n\x0f\n\x07\x04\x1c\x03\x01\x02\0\x05\x12\x04\x81\
    \x02\x06\x0c\n\x0f\n\x07\x04\x1c\x03\x01\x02\0\x01\x12\x04\x81\x02\r\x12\
    \n\x0f\n\x07\x04\x1c\x03\x01\x02\0\x03\x12\x04\x81\x02\x15\x16\n\x0e\n\
    \x06\x04\x1c\x03\x01\x02\x01\x12\x04\x82\x02\x06\x1d\n\x0f\n\x07\x04\x1c\
    \x03\x01\x02\x01\x05\x12\x04\x82\x02\x06\x0c\n\x0f\n\x07\x04\x1c\x03\x01\
    \x02\x01\x01\x12\x04\x82\x02\r\x18\n\x0f\n\x07\x04\x1c\x03\x01\x02\x01\
    \x03\x12\x04\x82\x02\x1b\x1c\n\x0e\n\x06\x04\x1c\x03\x01\x02\x02\x12\x04\
    \x83\x02\x06\x1c\n\x0f\n\x07\x04\x1c\x03\x01\x02\x02\x05\x12\x04\x83\x02\
    \x06\x0c\n\x0f\n\x07\x04\x1c\x03\x01\x02\x02\x01\x12\x04\x83\x02\r\x17\n\
    \x0f\n\x07\x04\x1c\x03\x01\x02\x02\x03\x12\x04\x83\x02\x1a\x1b\n\x0e\n\
    \x06\x04\x1c\x03\x01\x02\x03\x12\x04\x84\x02\x06\x17\n\x0f\n\x07\x04\x1c\
    \x03\x01\x02\x03\x05\x12\x04\x84\x02\x06\n\n\x0f\n\x07\x04\x1c\x03\x01\
    \x02\x03\x01\x12\x04\x84\x02\x0b\x12\n\x0f\n\x07\x04\x1c\x03\x01\x02\x03\
    \x03\x12\x04\x84\x02\x15\x16\n\x0e\n\x06\x04\x1c\x03\x01\x02\x04\x12\x04\
    \x85\x02\x06%\n\x0f\n\x07\x04\x1c\x03\x01\x02\x04\x04\x12\x04\x85\x02\
    \x06\x0e\n\x0f\n\x07\x04\x1c\x03\x01\x02\x04\x06\x12\x04\x85\x02\x0f\x17\
    \n\x0f\n\x07\x04\x1c\x03\x01\x02\x04\x01\x12\x04\x85\x02\x18\x20\n\x0f\n\
    \x07\x04\x1c\x03\x01\x02\x04\x03\x12\x04\x85\x02#$\n\x0e\n\x06\x04\x1c\
    \x03\x01\x02\x05\x12\x04\x86\x02\x06\x14\n\x0f\n\x07\x04\x1c\x03\x01\x02\
    \x05\x05\x12\x04\x86\x02\x06\x0c\n\x0f\n\x07\x04\x1c\x03\x01\x02\x05\x01\
    \x12\x04\x86\x02\r\x0f\n\x0f\n\x07\x04\x1c\x03\x01\x02\x05\x03\x12\x04\
    \x86\x02\x12\x13\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\x88\x02\x02\x12\n\r\n\
    \x05\x04\x1c\x02\0\x05\x12\x04\x88\x02\x02\x08\n\r\n\x05\x04\x1c\x02\0\
    \x01\x12\x04\x88\x02\t\r\n\r\n\x05\x04\x1c\x02\0\x03\x12\x04\x88\x02\x10\
    \x11\n\x0c\n\x04\x04\x1c\x02\x01\x12\x04\x89\x02\x02\x11\n\r\n\x05\x04\
    \x1c\x02\x01\x05\x12\x04\x89\x02\x02\x08\n\r\n\x05\x04\x1c\x02\x01\x01\
    \x12\x04\x89\x02\t\x0c\n\r\n\x05\x04\x1c\x02\x01\x03\x12\x04\x89\x02\x0f\
    \x10\n\x0c\n\x04\x04\x1c\x02\x02\x12\x04\x8a\x02\x02\x13\n\r\n\x05\x04\
    \x1c\x02\x02\x05\x12\x04\x8a\x02\x02\x08\n\r\n\x05\x04\x1c\x02\x02\x01\
    \x12\x04\x8a\x02\t\x0e\n\r\n\x05\x04\x1c\x02\x02\x03\x12\x04\x8a\x02\x11\
    \x12\n\x0c\n\x04\x04\x1c\x02\x03\x12\x04\x8b\x02\x02!\n\r\n\x05\x04\x1c\
    \x02\x03\x04\x12\x04\x8b\x02\x02\n\n\r\n\x05\x04\x1c\x02\x03\x06\x12\x04\
    \x8b\x02\x0b\x17\n\r\n\x05\x04\x1c\x02\x03\x01\x12\x04\x8b\x02\x18\x1c\n\
    \r\n\x05\x04\x1c\x02\x03\x03\x12\x04\x8b\x02\x1f\x20\n\x0c\n\x02\x04\x1d\
    \x12\x06\x8e\x02\0\x99\x02\x01\n\x0b\n\x03\x04\x1d\x01\x12\x04\x8e\x02\
    \x08\x1e\n\x0c\n\x04\x04\x1d\x02\0\x12\x04\x8f\x02\x02\x13\n\r\n\x05\x04\
    \x1d\x02\0\x05\x12\x04\x8f\x02\x02\x08\n\r\n\x05\x04\x1d\x02\0\x01\x12\
    \x04\x8f\x02\t\x0e\n\r\n\x05\x04\x1d\x02\0\x03\x12\x04\x8f\x02\x11\x12\n\
    \x0c\n\x04\x04\x1d\x02\x01\x12\x04\x90\x02\x02\x15\n\r\n\x05\x04\x1d\x02\
    \x01\x05\x12\x04\x90\x02\x02\x08\n\r\n\x05\x04\x1d\x02\x01\x01\x12\x04\
    \x90\x02\t\x10\n\r\n\x05\x04\x1d\x02\x01\x03\x12\x04\x90\x02\x13\x14\n\
    \x0c\n\x04\x04\x1d\x02\x02\x12\x04\x91\x02\x02\x18\n\r\n\x05\x04\x1d\x02\
    \x02\x05\x12\x04\x91\x02\x02\x08\n\r\n\x05\x04\x1d\x02\x02\x01\x12\x04\
    \x91\x02\t\x13\n\r\n\x05\x04\x1d\x02\x02\x03\x12\x04\x91\x02\x16\x17\n\
    \x0c\n\x04\x04\x1d\x02\x03\x12\x04\x92\x02\x02\x13\n\r\n\x05\x04\x1d\x02\
    \x03\x05\x12\x04\x92\x02\x02\x06\n\r\n\x05\x04\x1d\x02\x03\x01\x12\x04\
    \x92\x02\x07\x0e\n\r\n\x05\x04\x1d\x02\x03\x03\x12\x04\x92\x02\x11\x12\n\
    \x0c\n\x04\x04\x1d\x02\x04\x12\x04\x93\x02\x02\x13\n\r\n\x05\x04\x1d\x02\
    \x04\x05\x12\x04\x93\x02\x02\x06\n\r\n\x05\x04\x1d\x02\x04\x01\x12\x04\
    \x93\x02\x07\x0e\n\r\n\x05\x04\x1d\x02\x04\x03\x12\x04\x93\x02\x11\x12\n\
    \x0c\n\x04\x04\x1d\x02\x05\x12\x04\x94\x02\x02\x1b\n\r\n\x05\x04\x1d\x02\
    \x05\x05\x12\x04\x94\x02\x02\x08\n\r\n\x05\x04\x1d\x02\x05\x01\x12\x04\
    \x94\x02\t\x16\n\r\n\x05\x04\x1d\x02\x05\x03\x12\x04\x94\x02\x19\x1a\n\
    \x0c\n\x04\x04\x1d\x02\x06\x12\x04\x95\x02\x02\x17\n\r\n\x05\x04\x1d\x02\
    \x06\x05\x12\x04\x95\x02\x02\x08\n\r\n\x05\x04\x1d\x02\x06\x01\x12\x04\
    \x95\x02\t\x12\n\r\n\x05\x04\x1d\x02\x06\x03\x12\x04\x95\x02\x15\x16\n\
    \x0c\n\x04\x04\x1d\x02\x07\x12\x04\x96\x02\x02\x14\n\r\n\x05\x04\x1d\x02\
    \x07\x05\x12\x04\x96\x02\x02\x08\n\r\n\x05\x04\x1d\x02\x07\x01\x12\x04\
    \x96\x02\t\x0f\n\r\n\x05\x04\x1d\x02\x07\x03\x12\x04\x96\x02\x12\x13\n\
    \x0c\n\x04\x04\x1d\x02\x08\x12\x04\x97\x02\x02#\n\r\n\x05\x04\x1d\x02\
    \x08\x04\x12\x04\x97\x02\x02\n\n\r\n\x05\x04\x1d\x02\x08\x05\x12\x04\x97\
    \x02\x0b\x11\n\r\n\x05\x04\x1d\x02\x08\x01\x12\x04\x97\x02\x12\x1e\n\r\n\
    \x05\x04\x1d\x02\x08\x03\x12\x04\x97\x02!\"\n\x0c\n\x04\x04\x1d\x02\t\
    \x12\x04\x98\x02\x02\x16\n\r\n\x05\x04\x1d\x02\t\x05\x12\x04\x98\x02\x02\
    \x08\n\r\n\x05\x04\x1d\x02\t\x01\x12\x04\x98\x02\t\x10\n\r\n\x05\x04\x1d\
    \x02\t\x03\x12\x04\x98\x02\x13\x15\n\x0c\n\x02\x04\x1e\x12\x06\x9b\x02\0\
    \xb2\x02\x01\n\x0b\n\x03\x04\x1e\x01\x12\x04\x9b\x02\x08\x1c\n\x0e\n\x04\
    \x04\x1e\x03\0\x12\x06\x9c\x02\x02\x9f\x02\x03\n\r\n\x05\x04\x1e\x03\0\
    \x01\x12\x04\x9c\x02\n\"\n\x0e\n\x06\x04\x1e\x03\0\x02\0\x12\x04\x9d\x02\
    \x06\x17\n\x0f\n\x07\x04\x1e\x03\0\x02\0\x05\x12\x04\x9d\x02\x06\x0c\n\
    \x0f\n\x07\x04\x1e\x03\0\x02\0\x01\x12\x04\x9d\x02\r\x12\n\x0f\n\x07\x04\
    \x1e\x03\0\x02\0\x03\x12\x04\x9d\x02\x15\x16\n\x0e\n\x06\x04\x1e\x03\0\
    \x02\x01\x12\x04\x9e\x02\x06\x1d\n\x0f\n\x07\x04\x1e\x03\0\x02\x01\x05\
    \x12\x04\x9e\x02\x06\x0c\n\x0f\n\x07\x04\x1e\x03\0\x02\x01\x01\x12\x04\
    \x9e\x02\r\x18\n\x0f\n\x07\x04\x1e\x03\0\x02\x01\x03\x12\x04\x9e\x02\x1b\
    \x1c\n\x0e\n\x04\x04\x1e\x03\x01\x12\x06\xa1\x02\x02\xac\x02\x03\n\r\n\
    \x05\x04\x1e\x03\x01\x01\x12\x04\xa1\x02\n\x1e\n\x0e\n\x06\x04\x1e\x03\
    \x01\x02\0\x12\x04\xa2\x02\x04\x15\n\x0f\n\x07\x04\x1e\x03\x01\x02\0\x05\
    \x12\x04\xa2\x02\x04\n\n\x0f\n\x07\x04\x1e\x03\x01\x02\0\x01\x12\x04\xa2\
    \x02\x0b\x10\n\x0f\n\x07\x04\x1e\x03\x01\x02\0\x03\x12\x04\xa2\x02\x13\
    \x14\n\x0e\n\x06\x04\x1e\x03\x01\x02\x01\x12\x04\xa3\x02\x04\x17\n\x0f\n\
    \x07\x04\x1e\x03\x01\x02\x01\x05\x12\x04\xa3\x02\x04\n\n\x0f\n\x07\x04\
    \x1e\x03\x01\x02\x01\x01\x12\x04\xa3\x02\x0b\x12\n\x0f\n\x07\x04\x1e\x03\
    \x01\x02\x01\x03\x12\x04\xa3\x02\x15\x16\n\x0e\n\x06\x04\x1e\x03\x01\x02\
    \x02\x12\x04\xa4\x02\x04\x19\n\x0f\n\x07\x04\x1e\x03\x01\x02\x02\x05\x12\
    \x04\xa4\x02\x04\n\n\x0f\n\x07\x04\x1e\x03\x01\x02\x02\x01\x12\x04\xa4\
    \x02\x0b\x14\n\x0f\n\x07\x04\x1e\x03\x01\x02\x02\x03\x12\x04\xa4\x02\x17\
    \x18\n\x0e\n\x06\x04\x1e\x03\x01\x02\x03\x12\x04\xa5\x02\x04\x15\n\x0f\n\
    \x07\x04\x1e\x03\x01\x02\x03\x05\x12\x04\xa5\x02\x04\x08\n\x0f\n\x07\x04\
    \x1e\x03\x01\x02\x03\x01\x12\x04\xa5\x02\t\x10\n\x0f\n\x07\x04\x1e\x03\
    \x01\x02\x03\x03\x12\x04\xa5\x02\x13\x14\n\x0e\n\x06\x04\x1e\x03\x01\x02\
    \x04\x12\x04\xa6\x02\x04\x16\n\x0f\n\x07\x04\x1e\x03\x01\x02\x04\x05\x12\
    \x04\xa6\x02\x04\n\n\x0f\n\x07\x04\x1e\x03\x01\x02\x04\x01\x12\x04\xa6\
    \x02\x0b\x11\n\x0f\n\x07\x04\x1e\x03\x01\x02\x04\x03\x12\x04\xa6\x02\x14\
    \x15\n\x0e\n\x06\x04\x1e\x03\x01\x02\x05\x12\x04\xa7\x02\x04\x15\n\x0f\n\
    \x07\x04\x1e\x03\x01\x02\x05\x05\x12\x04\xa7\x02\x04\x08\n\x0f\n\x07\x04\
    \x1e\x03\x01\x02\x05\x01\x12\x04\xa7\x02\t\x10\n\x0f\n\x07\x04\x1e\x03\
    \x01\x02\x05\x03\x12\x04\xa7\x02\x13\x14\n\x0e\n\x06\x04\x1e\x03\x01\x02\
    \x06\x12\x04\xa8\x02\x04\x1a\n\x0f\n\x07\x04\x1e\x03\x01\x02\x06\x05\x12\
    \x04\xa8\x02\x04\n\n\x0f\n\x07\x04\x1e\x03\x01\x02\x06\x01\x12\x04\xa8\
    \x02\x0b\x15\n\x0f\n\x07\x04\x1e\x03\x01\x02\x06\x03\x12\x04\xa8\x02\x18\
    \x19\n\x0e\n\x06\x04\x1e\x03\x01\x02\x07\x12\x04\xa9\x02\x04\x12\n\x0f\n\
    \x07\x04\x1e\x03\x01\x02\x07\x05\x12\x04\xa9\x02\x04\n\n\x0f\n\x07\x04\
    \x1e\x03\x01\x02\x07\x01\x12\x04\xa9\x02\x0b\r\n\x0f\n\x07\x04\x1e\x03\
    \x01\x02\x07\x03\x12\x04\xa9\x02\x10\x11\n\x0e\n\x06\x04\x1e\x03\x01\x02\
    \x08\x12\x04\xaa\x02\x04\x14\n\x0f\n\x07\x04\x1e\x03\x01\x02\x08\x05\x12\
    \x04\xaa\x02\x04\x08\n\x0f\n\x07\x04\x1e\x03\x01\x02\x08\x01\x12\x04\xaa\
    \x02\t\x0f\n\x0f\n\x07\x04\x1e\x03\x01\x02\x08\x03\x12\x04\xaa\x02\x12\
    \x13\n\x0e\n\x06\x04\x1e\x03\x01\x02\t\x12\x04\xab\x02\x04+\n\x0f\n\x07\
    \x04\x1e\x03\x01\x02\t\x06\x12\x04\xab\x02\x04\x1c\n\x0f\n\x07\x04\x1e\
    \x03\x01\x02\t\x01\x12\x04\xab\x02\x1d%\n\x0f\n\x07\x04\x1e\x03\x01\x02\
    \t\x03\x12\x04\xab\x02(*\n\x0c\n\x04\x04\x1e\x02\0\x12\x04\xad\x02\x02\
    \x12\n\r\n\x05\x04\x1e\x02\0\x05\x12\x04\xad\x02\x02\x08\n\r\n\x05\x04\
    \x1e\x02\0\x01\x12\x04\xad\x02\t\r\n\r\n\x05\x04\x1e\x02\0\x03\x12\x04\
    \xad\x02\x10\x11\n\x0c\n\x04\x04\x1e\x02\x01\x12\x04\xae\x02\x02\x11\n\r\
    \n\x05\x04\x1e\x02\x01\x05\x12\x04\xae\x02\x02\x08\n\r\n\x05\x04\x1e\x02\
    \x01\x01\x12\x04\xae\x02\t\x0c\n\r\n\x05\x04\x1e\x02\x01\x03\x12\x04\xae\
    \x02\x0f\x10\n\x0c\n\x04\x04\x1e\x02\x02\x12\x04\xaf\x02\x02\x13\n\r\n\
    \x05\x04\x1e\x02\x02\x05\x12\x04\xaf\x02\x02\x08\n\r\n\x05\x04\x1e\x02\
    \x02\x01\x12\x04\xaf\x02\t\x0e\n\r\n\x05\x04\x1e\x02\x02\x03\x12\x04\xaf\
    \x02\x11\x12\n\x0c\n\x04\x04\x1e\x02\x03\x12\x04\xb0\x02\x02)\n\r\n\x05\
    \x04\x1e\x02\x03\x04\x12\x04\xb0\x02\x02\n\n\r\n\x05\x04\x1e\x02\x03\x06\
    \x12\x04\xb0\x02\x0b\x1f\n\r\n\x05\x04\x1e\x02\x03\x01\x12\x04\xb0\x02\
    \x20$\n\r\n\x05\x04\x1e\x02\x03\x03\x12\x04\xb0\x02'(\n\x0c\n\x02\x04\
    \x1f\x12\x06\xb4\x02\0\xc2\x02\x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\xb4\
    \x02\x08\x1b\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xb5\x02\x02\x12\n\r\n\x05\
    \x04\x1f\x02\0\x05\x12\x04\xb5\x02\x02\x08\n\r\n\x05\x04\x1f\x02\0\x01\
    \x12\x04\xb5\x02\t\r\n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\xb5\x02\x10\x11\
    \n\x0c\n\x04\x04\x1f\x02\x01\x12\x04\xb6\x02\x02\x11\n\r\n\x05\x04\x1f\
    \x02\x01\x05\x12\x04\xb6\x02\x02\x08\n\r\n\x05\x04\x1f\x02\x01\x01\x12\
    \x04\xb6\x02\t\x0c\n\r\n\x05\x04\x1f\x02\x01\x03\x12\x04\xb6\x02\x0f\x10\
    \n\x0c\n\x04\x04\x1f\x02\x02\x12\x04\xb7\x02\x02\x13\n\r\n\x05\x04\x1f\
    \x02\x02\x05\x12\x04\xb7\x02\x02\x08\n\r\n\x05\x04\x1f\x02\x02\x01\x12\
    \x04\xb7\x02\t\x0e\n\r\n\x05\x04\x1f\x02\x02\x03\x12\x04\xb7\x02\x11\x12\
    \n\x0c\n\x04\x04\x1f\x02\x03\x12\x04\xb8\x02\x02\x15\n\r\n\x05\x04\x1f\
    \x02\x03\x05\x12\x04\xb8\x02\x02\x08\n\r\n\x05\x04\x1f\x02\x03\x01\x12\
    \x04\xb8\x02\t\x10\n\r\n\x05\x04\x1f\x02\x03\x03\x12\x04\xb8\x02\x13\x14\
    \n\x0c\n\x04\x04\x1f\x02\x04\x12\x04\xb9\x02\x02\x18\n\r\n\x05\x04\x1f\
    \x02\x04\x05\x12\x04\xb9\x02\x02\x08\n\r\n\x05\x04\x1f\x02\x04\x01\x12\
    \x04\xb9\x02\t\x13\n\r\n\x05\x04\x1f\x02\x04\x03\x12\x04\xb9\x02\x16\x17\
    \n\x0c\n\x04\x04\x1f\x02\x05\x12\x04\xba\x02\x02\x13\n\r\n\x05\x04\x1f\
    \x02\x05\x05\x12\x04\xba\x02\x02\x06\n\r\n\x05\x04\x1f\x02\x05\x01\x12\
    \x04\xba\x02\x07\x0e\n\r\n\x05\x04\x1f\x02\x05\x03\x12\x04\xba\x02\x11\
    \x12\n\x0c\n\x04\x04\x1f\x02\x06\x12\x04\xbb\x02\x02\x13\n\r\n\x05\x04\
    \x1f\x02\x06\x05\x12\x04\xbb\x02\x02\x06\n\r\n\x05\x04\x1f\x02\x06\x01\
    \x12\x04\xbb\x02\x07\x0e\n\r\n\x05\x04\x1f\x02\x06\x03\x12\x04\xbb\x02\
    \x11\x12\n\x0c\n\x04\x04\x1f\x02\x07\x12\x04\xbc\x02\x02\x1b\n\r\n\x05\
    \x04\x1f\x02\x07\x05\x12\x04\xbc\x02\x02\x08\n\r\n\x05\x04\x1f\x02\x07\
    \x01\x12\x04\xbc\x02\t\x16\n\r\n\x05\x04\x1f\x02\x07\x03\x12\x04\xbc\x02\
    \x19\x1a\n\x0c\n\x04\x04\x1f\x02\x08\x12\x04\xbd\x02\x02\x17\n\r\n\x05\
    \x04\x1f\x02\x08\x05\x12\x04\xbd\x02\x02\x08\n\r\n\x05\x04\x1f\x02\x08\
    \x01\x12\x04\xbd\x02\t\x12\n\r\n\x05\x04\x1f\x02\x08\x03\x12\x04\xbd\x02\
    \x15\x16\n\x0c\n\x04\x04\x1f\x02\t\x12\x04\xbe\x02\x02\x15\n\r\n\x05\x04\
    \x1f\x02\t\x05\x12\x04\xbe\x02\x02\x08\n\r\n\x05\x04\x1f\x02\t\x01\x12\
    \x04\xbe\x02\t\x0f\n\r\n\x05\x04\x1f\x02\t\x03\x12\x04\xbe\x02\x12\x14\n\
    \x0c\n\x04\x04\x1f\x02\n\x12\x04\xbf\x02\x02$\n\r\n\x05\x04\x1f\x02\n\
    \x04\x12\x04\xbf\x02\x02\n\n\r\n\x05\x04\x1f\x02\n\x05\x12\x04\xbf\x02\
    \x0b\x11\n\r\n\x05\x04\x1f\x02\n\x01\x12\x04\xbf\x02\x12\x1e\n\r\n\x05\
    \x04\x1f\x02\n\x03\x12\x04\xbf\x02!#\n\x0c\n\x04\x04\x1f\x02\x0b\x12\x04\
    \xc0\x02\x02\x16\n\r\n\x05\x04\x1f\x02\x0b\x05\x12\x04\xc0\x02\x02\x08\n\
    \r\n\x05\x04\x1f\x02\x0b\x01\x12\x04\xc0\x02\t\x10\n\r\n\x05\x04\x1f\x02\
    \x0b\x03\x12\x04\xc0\x02\x13\x15\n\x0c\n\x04\x04\x1f\x02\x0c\x12\x04\xc1\
    \x02\x02\x11\n\r\n\x05\x04\x1f\x02\x0c\x05\x12\x04\xc1\x02\x02\x08\n\r\n\
    \x05\x04\x1f\x02\x0c\x01\x12\x04\xc1\x02\t\x0b\n\r\n\x05\x04\x1f\x02\x0c\
    \x03\x12\x04\xc1\x02\x0e\x10\n\x0c\n\x02\x04\x20\x12\x06\xc5\x02\0\xc8\
    \x02\x01\n\x0b\n\x03\x04\x20\x01\x12\x04\xc5\x02\x08\x0c\n\x0c\n\x04\x04\
    \x20\x02\0\x12\x04\xc6\x02\x02\x11\n\r\n\x05\x04\x20\x02\0\x05\x12\x04\
    \xc6\x02\x02\x08\n\r\n\x05\x04\x20\x02\0\x01\x12\x04\xc6\x02\t\x0c\n\r\n\
    \x05\x04\x20\x02\0\x03\x12\x04\xc6\x02\x0f\x10\n\x0c\n\x04\x04\x20\x02\
    \x01\x12\x04\xc7\x02\x02\x12\n\r\n\x05\x04\x20\x02\x01\x05\x12\x04\xc7\
    \x02\x02\x08\n\r\n\x05\x04\x20\x02\x01\x01\x12\x04\xc7\x02\t\r\n\r\n\x05\
    \x04\x20\x02\x01\x03\x12\x04\xc7\x02\x10\x11\n\x0c\n\x02\x04!\x12\x06\
    \xca\x02\0\xe0\x02\x01\n\x0b\n\x03\x04!\x01\x12\x04\xca\x02\x08\x17\n\
    \x0e\n\x04\x04!\x03\0\x12\x06\xcc\x02\x02\xd0\x02\x03\n\r\n\x05\x04!\x03\
    \0\x01\x12\x04\xcc\x02\n\x0e\n\x0e\n\x06\x04!\x03\0\x02\0\x12\x04\xcd\
    \x02\x04\x13\n\x0f\n\x07\x04!\x03\0\x02\0\x05\x12\x04\xcd\x02\x04\n\n\
    \x0f\n\x07\x04!\x03\0\x02\0\x01\x12\x04\xcd\x02\x0b\x0e\n\x0f\n\x07\x04!\
    \x03\0\x02\0\x03\x12\x04\xcd\x02\x11\x12\n\x0e\n\x06\x04!\x03\0\x02\x01\
    \x12\x04\xce\x02\x04\x1a\n\x0f\n\x07\x04!\x03\0\x02\x01\x05\x12\x04\xce\
    \x02\x04\n\n\x0f\n\x07\x04!\x03\0\x02\x01\x01\x12\x04\xce\x02\x0b\x15\n\
    \x0f\n\x07\x04!\x03\0\x02\x01\x03\x12\x04\xce\x02\x18\x19\n\x0e\n\x06\
    \x04!\x03\0\x02\x02\x12\x04\xcf\x02\x04\x15\n\x0f\n\x07\x04!\x03\0\x02\
    \x02\x05\x12\x04\xcf\x02\x04\n\n\x0f\n\x07\x04!\x03\0\x02\x02\x01\x12\
    \x04\xcf\x02\x0b\x10\n\x0f\n\x07\x04!\x03\0\x02\x02\x03\x12\x04\xcf\x02\
    \x13\x14\n\x0e\n\x04\x04!\x03\x01\x12\x06\xd1\x02\x02\xd5\x02\x03\n\r\n\
    \x05\x04!\x03\x01\x01\x12\x04\xd1\x02\n\x13\n\x0e\n\x06\x04!\x03\x01\x02\
    \0\x12\x04\xd2\x02\x04\x13\n\x0f\n\x07\x04!\x03\x01\x02\0\x05\x12\x04\
    \xd2\x02\x04\n\n\x0f\n\x07\x04!\x03\x01\x02\0\x01\x12\x04\xd2\x02\x0b\
    \x0e\n\x0f\n\x07\x04!\x03\x01\x02\0\x03\x12\x04\xd2\x02\x11\x12\n\x0e\n\
    \x06\x04!\x03\x01\x02\x01\x12\x04\xd3\x02\x04\x14\n\x0f\n\x07\x04!\x03\
    \x01\x02\x01\x05\x12\x04\xd3\x02\x04\n\n\x0f\n\x07\x04!\x03\x01\x02\x01\
    \x01\x12\x04\xd3\x02\x0b\x0f\n\x0f\n\x07\x04!\x03\x01\x02\x01\x03\x12\
    \x04\xd3\x02\x12\x13\n\x0e\n\x06\x04!\x03\x01\x02\x02\x12\x04\xd4\x02\
    \x04\x15\n\x0f\n\x07\x04!\x03\x01\x02\x02\x05\x12\x04\xd4\x02\x04\n\n\
    \x0f\n\x07\x04!\x03\x01\x02\x02\x01\x12\x04\xd4\x02\x0b\x10\n\x0f\n\x07\
    \x04!\x03\x01\x02\x02\x03\x12\x04\xd4\x02\x13\x14\n\x0e\n\x04\x04!\x03\
    \x02\x12\x06\xd6\x02\x02\xd9\x02\x03\n\r\n\x05\x04!\x03\x02\x01\x12\x04\
    \xd6\x02\n\x15\n\x0e\n\x06\x04!\x03\x02\x02\0\x12\x04\xd7\x02\x04\x14\n\
    \x0f\n\x07\x04!\x03\x02\x02\0\x05\x12\x04\xd7\x02\x04\n\n\x0f\n\x07\x04!\
    \x03\x02\x02\0\x01\x12\x04\xd7\x02\x0b\x0f\n\x0f\n\x07\x04!\x03\x02\x02\
    \0\x03\x12\x04\xd7\x02\x12\x13\n\x0e\n\x06\x04!\x03\x02\x02\x01\x12\x04\
    \xd8\x02\x04\x1b\n\x0f\n\x07\x04!\x03\x02\x02\x01\x04\x12\x04\xd8\x02\
    \x04\x0c\n\x0f\n\x07\x04!\x03\x02\x02\x01\x06\x12\x04\xd8\x02\r\x11\n\
    \x0f\n\x07\x04!\x03\x02\x02\x01\x01\x12\x04\xd8\x02\x12\x16\n\x0f\n\x07\
    \x04!\x03\x02\x02\x01\x03\x12\x04\xd8\x02\x19\x1a\n\x0c\n\x04\x04!\x02\0\
    \x12\x04\xda\x02\x02\x12\n\r\n\x05\x04!\x02\0\x05\x12\x04\xda\x02\x02\
    \x08\n\r\n\x05\x04!\x02\0\x01\x12\x04\xda\x02\t\r\n\r\n\x05\x04!\x02\0\
    \x03\x12\x04\xda\x02\x10\x11\n\x0c\n\x04\x04!\x02\x01\x12\x04\xdb\x02\
    \x02\x11\n\r\n\x05\x04!\x02\x01\x05\x12\x04\xdb\x02\x02\x08\n\r\n\x05\
    \x04!\x02\x01\x01\x12\x04\xdb\x02\t\x0c\n\r\n\x05\x04!\x02\x01\x03\x12\
    \x04\xdb\x02\x0f\x10\n\x0c\n\x04\x04!\x02\x02\x12\x04\xdc\x02\x02#\n\r\n\
    \x05\x04!\x02\x02\x04\x12\x04\xdc\x02\x02\n\n\r\n\x05\x04!\x02\x02\x06\
    \x12\x04\xdc\x02\x0b\x14\n\r\n\x05\x04!\x02\x02\x01\x12\x04\xdc\x02\x15\
    \x1e\n\r\n\x05\x04!\x02\x02\x03\x12\x04\xdc\x02!\"\n\x0c\n\x04\x04!\x02\
    \x03\x12\x04\xdd\x02\x02'\n\r\n\x05\x04!\x02\x03\x04\x12\x04\xdd\x02\x02\
    \n\n\r\n\x05\x04!\x02\x03\x06\x12\x04\xdd\x02\x0b\x16\n\r\n\x05\x04!\x02\
    \x03\x01\x12\x04\xdd\x02\x17\"\n\r\n\x05\x04!\x02\x03\x03\x12\x04\xdd\
    \x02%&\n\x0c\n\x04\x04!\x02\x04\x12\x04\xde\x02\x02\x19\n\r\n\x05\x04!\
    \x02\x04\x04\x12\x04\xde\x02\x02\n\n\r\n\x05\x04!\x02\x04\x06\x12\x04\
    \xde\x02\x0b\x0f\n\r\n\x05\x04!\x02\x04\x01\x12\x04\xde\x02\x10\x14\n\r\
    \n\x05\x04!\x02\x04\x03\x12\x04\xde\x02\x17\x18\n\x0c\n\x02\x04\"\x12\
    \x06\xe2\x02\0\xe6\x02\x01\n\x0b\n\x03\x04\"\x01\x12\x04\xe2\x02\x08\x20\
    \n\x0c\n\x04\x04\"\x02\0\x12\x04\xe3\x02\x02\x10\n\r\n\x05\x04\"\x02\0\
    \x05\x12\x04\xe3\x02\x02\x08\n\r\n\x05\x04\"\x02\0\x01\x12\x04\xe3\x02\t\
    \x0b\n\r\n\x05\x04\"\x02\0\x03\x12\x04\xe3\x02\x0e\x0f\n\x0c\n\x04\x04\"\
    \x02\x01\x12\x04\xe4\x02\x02\x13\n\r\n\x05\x04\"\x02\x01\x05\x12\x04\xe4\
    \x02\x02\x08\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\xe4\x02\t\x0e\n\r\n\x05\
    \x04\"\x02\x01\x03\x12\x04\xe4\x02\x11\x12\n\x0c\n\x04\x04\"\x02\x02\x12\
    \x04\xe5\x02\x02\x19\n\r\n\x05\x04\"\x02\x02\x05\x12\x04\xe5\x02\x02\x08\
    \n\r\n\x05\x04\"\x02\x02\x01\x12\x04\xe5\x02\t\x14\n\r\n\x05\x04\"\x02\
    \x02\x03\x12\x04\xe5\x02\x17\x18\n\x0c\n\x02\x04#\x12\x06\xe8\x02\0\xf1\
    \x02\x01\n\x0b\n\x03\x04#\x01\x12\x04\xe8\x02\x08\x10\n\x0c\n\x04\x04#\
    \x02\0\x12\x04\xe9\x02\x02\x13\n\r\n\x05\x04#\x02\0\x05\x12\x04\xe9\x02\
    \x02\x08\n\r\n\x05\x04#\x02\0\x01\x12\x04\xe9\x02\t\x0e\n\r\n\x05\x04#\
    \x02\0\x03\x12\x04\xe9\x02\x11\x12\n\x0c\n\x04\x04#\x02\x01\x12\x04\xea\
    \x02\x02\x19\n\r\n\x05\x04#\x02\x01\x05\x12\x04\xea\x02\x02\x08\n\r\n\
    \x05\x04#\x02\x01\x01\x12\x04\xea\x02\t\x14\n\r\n\x05\x04#\x02\x01\x03\
    \x12\x04\xea\x02\x17\x18\n\x0c\n\x04\x04#\x02\x02\x12\x04\xeb\x02\x02\
    \x13\n\r\n\x05\x04#\x02\x02\x05\x12\x04\xeb\x02\x02\x08\n\r\n\x05\x04#\
    \x02\x02\x01\x12\x04\xeb\x02\t\x0e\n\r\n\x05\x04#\x02\x02\x03\x12\x04\
    \xeb\x02\x11\x12\n\x0c\n\x04\x04#\x02\x03\x12\x04\xec\x02\x02\x11\n\r\n\
    \x05\x04#\x02\x03\x05\x12\x04\xec\x02\x02\x08\n\r\n\x05\x04#\x02\x03\x01\
    \x12\x04\xec\x02\t\x0c\n\r\n\x05\x04#\x02\x03\x03\x12\x04\xec\x02\x0f\
    \x10\n\x0c\n\x04\x04#\x02\x04\x12\x04\xed\x02\x02\x17\n\r\n\x05\x04#\x02\
    \x04\x05\x12\x04\xed\x02\x02\x08\n\r\n\x05\x04#\x02\x04\x01\x12\x04\xed\
    \x02\t\x12\n\r\n\x05\x04#\x02\x04\x03\x12\x04\xed\x02\x15\x16\n\x0c\n\
    \x04\x04#\x02\x05\x12\x04\xee\x02\x02\x13\n\r\n\x05\x04#\x02\x05\x05\x12\
    \x04\xee\x02\x02\x06\n\r\n\x05\x04#\x02\x05\x01\x12\x04\xee\x02\x07\x0e\
    \n\r\n\x05\x04#\x02\x05\x03\x12\x04\xee\x02\x11\x12\n\x0c\n\x04\x04#\x02\
    \x06\x12\x04\xef\x02\x02\x10\n\r\n\x05\x04#\x02\x06\x05\x12\x04\xef\x02\
    \x02\x08\n\r\n\x05\x04#\x02\x06\x01\x12\x04\xef\x02\t\x0b\n\r\n\x05\x04#\
    \x02\x06\x03\x12\x04\xef\x02\x0e\x0f\n\x0c\n\x04\x04#\x02\x07\x12\x04\
    \xf0\x02\x02\x18\n\r\n\x05\x04#\x02\x07\x05\x12\x04\xf0\x02\x02\x08\n\r\
    \n\x05\x04#\x02\x07\x01\x12\x04\xf0\x02\t\x13\n\r\n\x05\x04#\x02\x07\x03\
    \x12\x04\xf0\x02\x16\x17\n\x0c\n\x02\x04$\x12\x06\xf3\x02\0\xfa\x02\x01\
    \n\x0b\n\x03\x04$\x01\x12\x04\xf3\x02\x08\x13\n\x0c\n\x04\x04$\x02\0\x12\
    \x04\xf4\x02\x02\x13\n\r\n\x05\x04$\x02\0\x05\x12\x04\xf4\x02\x02\x08\n\
    \r\n\x05\x04$\x02\0\x01\x12\x04\xf4\x02\t\x0e\n\r\n\x05\x04$\x02\0\x03\
    \x12\x04\xf4\x02\x11\x12\n\x0c\n\x04\x04$\x02\x01\x12\x04\xf5\x02\x02\
    \x19\n\r\n\x05\x04$\x02\x01\x05\x12\x04\xf5\x02\x02\x08\n\r\n\x05\x04$\
    \x02\x01\x01\x12\x04\xf5\x02\t\x14\n\r\n\x05\x04$\x02\x01\x03\x12\x04\
    \xf5\x02\x17\x18\n\x0c\n\x04\x04$\x02\x02\x12\x04\xf6\x02\x02\x13\n\r\n\
    \x05\x04$\x02\x02\x05\x12\x04\xf6\x02\x02\x08\n\r\n\x05\x04$\x02\x02\x01\
    \x12\x04\xf6\x02\t\x0e\n\r\n\x05\x04$\x02\x02\x03\x12\x04\xf6\x02\x11\
    \x12\n\x0c\n\x04\x04$\x02\x03\x12\x04\xf7\x02\x02\x11\n\r\n\x05\x04$\x02\
    \x03\x05\x12\x04\xf7\x02\x02\x08\n\r\n\x05\x04$\x02\x03\x01\x12\x04\xf7\
    \x02\t\x0c\n\r\n\x05\x04$\x02\x03\x03\x12\x04\xf7\x02\x0f\x10\n\x0c\n\
    \x04\x04$\x02\x04\x12\x04\xf8\x02\x02\x17\n\r\n\x05\x04$\x02\x04\x05\x12\
    \x04\xf8\x02\x02\x08\n\r\n\x05\x04$\x02\x04\x01\x12\x04\xf8\x02\t\x12\n\
    \r\n\x05\x04$\x02\x04\x03\x12\x04\xf8\x02\x15\x16\n\x0c\n\x04\x04$\x02\
    \x05\x12\x04\xf9\x02\x02\x15\n\r\n\x05\x04$\x02\x05\x05\x12\x04\xf9\x02\
    \x02\x08\n\r\n\x05\x04$\x02\x05\x01\x12\x04\xf9\x02\t\x10\n\r\n\x05\x04$\
    \x02\x05\x03\x12\x04\xf9\x02\x13\x14\n\x0c\n\x02\x04%\x12\x06\xfc\x02\0\
    \x81\x03\x01\n\x0b\n\x03\x04%\x01\x12\x04\xfc\x02\x08\x14\n\x0c\n\x04\
    \x04%\x02\0\x12\x04\xfd\x02\x02\x12\n\r\n\x05\x04%\x02\0\x05\x12\x04\xfd\
    \x02\x02\x08\n\r\n\x05\x04%\x02\0\x01\x12\x04\xfd\x02\t\r\n\r\n\x05\x04%\
    \x02\0\x03\x12\x04\xfd\x02\x10\x11\n\x0c\n\x04\x04%\x02\x01\x12\x04\xfe\
    \x02\x02\x11\n\r\n\x05\x04%\x02\x01\x05\x12\x04\xfe\x02\x02\x08\n\r\n\
    \x05\x04%\x02\x01\x01\x12\x04\xfe\x02\t\x0c\n\r\n\x05\x04%\x02\x01\x03\
    \x12\x04\xfe\x02\x0f\x10\n\x0c\n\x04\x04%\x02\x02\x12\x04\xff\x02\x02\
    \x13\n\r\n\x05\x04%\x02\x02\x05\x12\x04\xff\x02\x02\x08\n\r\n\x05\x04%\
    \x02\x02\x01\x12\x04\xff\x02\t\x0e\n\r\n\x05\x04%\x02\x02\x03\x12\x04\
    \xff\x02\x11\x12\n\x0c\n\x04\x04%\x02\x03\x12\x04\x80\x03\x02\x1d\n\r\n\
    \x05\x04%\x02\x03\x04\x12\x04\x80\x03\x02\n\n\r\n\x05\x04%\x02\x03\x06\
    \x12\x04\x80\x03\x0b\x13\n\r\n\x05\x04%\x02\x03\x01\x12\x04\x80\x03\x14\
    \x18\n\r\n\x05\x04%\x02\x03\x03\x12\x04\x80\x03\x1b\x1c\n\x0c\n\x02\x04&\
    \x12\x06\x83\x03\0\x93\x03\x01\n\x0b\n\x03\x04&\x01\x12\x04\x83\x03\x08\
    \x14\n\x0c\n\x04\x04&\x02\0\x12\x04\x84\x03\x02\x12\n\r\n\x05\x04&\x02\0\
    \x05\x12\x04\x84\x03\x02\x08\n\r\n\x05\x04&\x02\0\x01\x12\x04\x84\x03\t\
    \r\n\r\n\x05\x04&\x02\0\x03\x12\x04\x84\x03\x10\x11\n\x0c\n\x04\x04&\x02\
    \x01\x12\x04\x85\x03\x02\x11\n\r\n\x05\x04&\x02\x01\x05\x12\x04\x85\x03\
    \x02\x08\n\r\n\x05\x04&\x02\x01\x01\x12\x04\x85\x03\t\x0c\n\r\n\x05\x04&\
    \x02\x01\x03\x12\x04\x85\x03\x0f\x10\n\x0c\n\x04\x04&\x02\x02\x12\x04\
    \x86\x03\x02\x13\n\r\n\x05\x04&\x02\x02\x05\x12\x04\x86\x03\x02\x08\n\r\
    \n\x05\x04&\x02\x02\x01\x12\x04\x86\x03\t\x0f\n\r\n\x05\x04&\x02\x02\x03\
    \x12\x04\x86\x03\x11\x12\n\x0c\n\x04\x04&\x02\x03\x12\x04\x87\x03\x02\
    \x16\n\r\n\x05\x04&\x02\x03\x05\x12\x04\x87\x03\x02\x08\n\r\n\x05\x04&\
    \x02\x03\x01\x12\x04\x87\x03\t\x11\n\r\n\x05\x04&\x02\x03\x03\x12\x04\
    \x87\x03\x14\x15\n\x0c\n\x04\x04&\x02\x04\x12\x04\x88\x03\x02\x14\n\r\n\
    \x05\x04&\x02\x04\x05\x12\x04\x88\x03\x02\x08\n\r\n\x05\x04&\x02\x04\x01\
    \x12\x04\x88\x03\t\x0f\n\r\n\x05\x04&\x02\x04\x03\x12\x04\x88\x03\x12\
    \x13\n\x0c\n\x04\x04&\x02\x05\x12\x04\x89\x03\x02\x14\n\r\n\x05\x04&\x02\
    \x05\x05\x12\x04\x89\x03\x02\x08\n\r\n\x05\x04&\x02\x05\x01\x12\x04\x89\
    \x03\t\x0f\n\r\n\x05\x04&\x02\x05\x03\x12\x04\x89\x03\x12\x13\n\x0c\n\
    \x04\x04&\x02\x06\x12\x04\x8a\x03\x02\x16\n\r\n\x05\x04&\x02\x06\x05\x12\
    \x04\x8a\x03\x02\x08\n\r\n\x05\x04&\x02\x06\x01\x12\x04\x8a\x03\t\x11\n\
    \r\n\x05\x04&\x02\x06\x03\x12\x04\x8a\x03\x14\x15\n\x0c\n\x04\x04&\x02\
    \x07\x12\x04\x8b\x03\x02\x15\n\r\n\x05\x04&\x02\x07\x05\x12\x04\x8b\x03\
    \x02\x08\n\r\n\x05\x04&\x02\x07\x01\x12\x04\x8b\x03\t\x10\n\r\n\x05\x04&\
    \x02\x07\x03\x12\x04\x8b\x03\x13\x14\n\x0c\n\x04\x04&\x02\x08\x12\x04\
    \x8c\x03\x02\x16\n\r\n\x05\x04&\x02\x08\x05\x12\x04\x8c\x03\x02\x08\n\r\
    \n\x05\x04&\x02\x08\x01\x12\x04\x8c\x03\t\x11\n\r\n\x05\x04&\x02\x08\x03\
    \x12\x04\x8c\x03\x14\x15\n\x0c\n\x04\x04&\x02\t\x12\x04\x8d\x03\x02\x15\
    \n\r\n\x05\x04&\x02\t\x05\x12\x04\x8d\x03\x02\x08\n\r\n\x05\x04&\x02\t\
    \x01\x12\x04\x8d\x03\t\x10\n\r\n\x05\x04&\x02\t\x03\x12\x04\x8d\x03\x12\
    \x14\n\x0c\n\x04\x04&\x02\n\x12\x04\x8e\x03\x02\x12\n\r\n\x05\x04&\x02\n\
    \x05\x12\x04\x8e\x03\x02\x06\n\r\n\x05\x04&\x02\n\x01\x12\x04\x8e\x03\
    \x07\x0c\n\r\n\x05\x04&\x02\n\x03\x12\x04\x8e\x03\x0f\x11\n\x0c\n\x04\
    \x04&\x02\x0b\x12\x04\x8f\x03\x02\x14\n\r\n\x05\x04&\x02\x0b\x05\x12\x04\
    \x8f\x03\x02\x08\n\r\n\x05\x04&\x02\x0b\x01\x12\x04\x8f\x03\t\x0e\n\r\n\
    \x05\x04&\x02\x0b\x03\x12\x04\x8f\x03\x11\x13\n\x0c\n\x04\x04&\x02\x0c\
    \x12\x04\x90\x03\x02\x1b\n\r\n\x05\x04&\x02\x0c\x05\x12\x04\x90\x03\x02\
    \x06\n\r\n\x05\x04&\x02\x0c\x01\x12\x04\x90\x03\x07\x15\n\r\n\x05\x04&\
    \x02\x0c\x03\x12\x04\x90\x03\x18\x1a\n\x0c\n\x04\x04&\x02\r\x12\x04\x91\
    \x03\x02\x14\n\r\n\x05\x04&\x02\r\x05\x12\x04\x91\x03\x02\x08\n\r\n\x05\
    \x04&\x02\r\x01\x12\x04\x91\x03\t\x0e\n\r\n\x05\x04&\x02\r\x03\x12\x04\
    \x91\x03\x11\x13\n\x0c\n\x04\x04&\x02\x0e\x12\x04\x92\x03\x02\x1c\n\r\n\
    \x05\x04&\x02\x0e\x05\x12\x04\x92\x03\x02\x08\n\r\n\x05\x04&\x02\x0e\x01\
    \x12\x04\x92\x03\t\x16\n\r\n\x05\x04&\x02\x0e\x03\x12\x04\x92\x03\x19\
    \x1b\n\x0c\n\x02\x04'\x12\x06\x95\x03\0\x99\x03\x01\n\x0b\n\x03\x04'\x01\
    \x12\x04\x95\x03\x08\x16\n\x0c\n\x04\x04'\x02\0\x12\x04\x96\x03\x02\x12\
    \n\r\n\x05\x04'\x02\0\x05\x12\x04\x96\x03\x02\x08\n\r\n\x05\x04'\x02\0\
    \x01\x12\x04\x96\x03\t\r\n\r\n\x05\x04'\x02\0\x03\x12\x04\x96\x03\x10\
    \x11\n\x0c\n\x04\x04'\x02\x01\x12\x04\x97\x03\x02\x11\n\r\n\x05\x04'\x02\
    \x01\x05\x12\x04\x97\x03\x02\x08\n\r\n\x05\x04'\x02\x01\x01\x12\x04\x97\
    \x03\t\x0c\n\r\n\x05\x04'\x02\x01\x03\x12\x04\x97\x03\x0f\x10\n\x0c\n\
    \x04\x04'\x02\x02\x12\x04\x98\x03\x02\x11\n\r\n\x05\x04'\x02\x02\x05\x12\
    \x04\x98\x03\x02\x08\n\r\n\x05\x04'\x02\x02\x01\x12\x04\x98\x03\t\x0c\n\
    \r\n\x05\x04'\x02\x02\x03\x12\x04\x98\x03\x0f\x10\n\x0c\n\x02\x04(\x12\
    \x06\x9b\x03\0\xa4\x03\x01\n\x0b\n\x03\x04(\x01\x12\x04\x9b\x03\x08\x1b\
    \n\x0c\n\x04\x04(\x02\0\x12\x04\x9c\x03\x04\x16\n\r\n\x05\x04(\x02\0\x05\
    \x12\x04\x9c\x03\x04\n\n\r\n\x05\x04(\x02\0\x01\x12\x04\x9c\x03\x0b\x11\
    \n\r\n\x05\x04(\x02\0\x03\x12\x04\x9c\x03\x14\x15\n\x0c\n\x04\x04(\x02\
    \x01\x12\x04\x9d\x03\x04\x15\n\r\n\x05\x04(\x02\x01\x05\x12\x04\x9d\x03\
    \x04\n\n\r\n\x05\x04(\x02\x01\x01\x12\x04\x9d\x03\x0b\x10\n\r\n\x05\x04(\
    \x02\x01\x03\x12\x04\x9d\x03\x13\x14\n\x0c\n\x04\x04(\x02\x02\x12\x04\
    \x9e\x03\x04\x12\n\r\n\x05\x04(\x02\x02\x05\x12\x04\x9e\x03\x04\x08\n\r\
    \n\x05\x04(\x02\x02\x01\x12\x04\x9e\x03\t\x0e\n\r\n\x05\x04(\x02\x02\x03\
    \x12\x04\x9e\x03\x10\x11\n\x0c\n\x04\x04(\x02\x03\x12\x04\x9f\x03\x04\
    \x17\n\r\n\x05\x04(\x02\x03\x05\x12\x04\x9f\x03\x04\n\n\r\n\x05\x04(\x02\
    \x03\x01\x12\x04\x9f\x03\x0b\x12\n\r\n\x05\x04(\x02\x03\x03\x12\x04\x9f\
    \x03\x15\x16\n\x0c\n\x04\x04(\x02\x04\x12\x04\xa0\x03\x04\x18\n\r\n\x05\
    \x04(\x02\x04\x05\x12\x04\xa0\x03\x04\n\n\r\n\x05\x04(\x02\x04\x01\x12\
    \x04\xa0\x03\x0b\x13\n\r\n\x05\x04(\x02\x04\x03\x12\x04\xa0\x03\x16\x17\
    \n\x0c\n\x04\x04(\x02\x05\x12\x04\xa1\x03\x04\x18\n\r\n\x05\x04(\x02\x05\
    \x05\x12\x04\xa1\x03\x04\n\n\r\n\x05\x04(\x02\x05\x01\x12\x04\xa1\x03\
    \x0b\x13\n\r\n\x05\x04(\x02\x05\x03\x12\x04\xa1\x03\x16\x17\n\x0c\n\x04\
    \x04(\x02\x06\x12\x04\xa2\x03\x04\x1c\n\r\n\x05\x04(\x02\x06\x05\x12\x04\
    \xa2\x03\x04\x08\n\r\n\x05\x04(\x02\x06\x01\x12\x04\xa2\x03\t\x17\n\r\n\
    \x05\x04(\x02\x06\x03\x12\x04\xa2\x03\x1a\x1b\n\x0c\n\x04\x04(\x02\x07\
    \x12\x04\xa3\x03\x04\x17\n\r\n\x05\x04(\x02\x07\x05\x12\x04\xa3\x03\x04\
    \n\n\r\n\x05\x04(\x02\x07\x01\x12\x04\xa3\x03\x0b\x12\n\r\n\x05\x04(\x02\
    \x07\x03\x12\x04\xa3\x03\x15\x16\n\x0c\n\x02\x04)\x12\x06\xa7\x03\0\xab\
    \x03\x01\n\x0b\n\x03\x04)\x01\x12\x04\xa7\x03\x08\x1a\n\x0c\n\x04\x04)\
    \x02\0\x12\x04\xa8\x03\x02\x15\n\r\n\x05\x04)\x02\0\x05\x12\x04\xa8\x03\
    \x02\x08\n\r\n\x05\x04)\x02\0\x01\x12\x04\xa8\x03\t\x10\n\r\n\x05\x04)\
    \x02\0\x03\x12\x04\xa8\x03\x13\x14\n\x0c\n\x04\x04)\x02\x01\x12\x04\xa9\
    \x03\x02\x19\n\r\n\x05\x04)\x02\x01\x05\x12\x04\xa9\x03\x02\x08\n\r\n\
    \x05\x04)\x02\x01\x01\x12\x04\xa9\x03\t\x14\n\r\n\x05\x04)\x02\x01\x03\
    \x12\x04\xa9\x03\x17\x18\n\x0c\n\x04\x04)\x02\x02\x12\x04\xaa\x03\x02\
    \x10\n\r\n\x05\x04)\x02\x02\x05\x12\x04\xaa\x03\x02\x08\n\r\n\x05\x04)\
    \x02\x02\x01\x12\x04\xaa\x03\t\x0b\n\r\n\x05\x04)\x02\x02\x03\x12\x04\
    \xaa\x03\x0e\x0f\n\x0c\n\x02\x04*\x12\x06\xad\x03\0\xb3\x03\x01\n\x0b\n\
    \x03\x04*\x01\x12\x04\xad\x03\x08\x16\n\x0c\n\x04\x04*\x02\0\x12\x04\xae\
    \x03\x02\x12\n\r\n\x05\x04*\x02\0\x05\x12\x04\xae\x03\x02\x08\n\r\n\x05\
    \x04*\x02\0\x01\x12\x04\xae\x03\t\r\n\r\n\x05\x04*\x02\0\x03\x12\x04\xae\
    \x03\x10\x11\n\x0c\n\x04\x04*\x02\x01\x12\x04\xaf\x03\x02\x11\n\r\n\x05\
    \x04*\x02\x01\x05\x12\x04\xaf\x03\x02\x08\n\r\n\x05\x04*\x02\x01\x01\x12\
    \x04\xaf\x03\t\x0c\n\r\n\x05\x04*\x02\x01\x03\x12\x04\xaf\x03\x0f\x10\n\
    \x0c\n\x04\x04*\x02\x02\x12\x04\xb0\x03\x02\x18\n\r\n\x05\x04*\x02\x02\
    \x05\x12\x04\xb0\x03\x02\x08\n\r\n\x05\x04*\x02\x02\x01\x12\x04\xb0\x03\
    \t\x12\n\r\n\x05\x04*\x02\x02\x03\x12\x04\xb0\x03\x16\x17\n\x0c\n\x04\
    \x04*\x02\x03\x12\x04\xb1\x03\x02\x1a\n\r\n\x05\x04*\x02\x03\x05\x12\x04\
    \xb1\x03\x02\x08\n\r\n\x05\x04*\x02\x03\x01\x12\x04\xb1\x03\t\x15\n\r\n\
    \x05\x04*\x02\x03\x03\x12\x04\xb1\x03\x18\x19\n\x0c\n\x04\x04*\x02\x04\
    \x12\x04\xb2\x03\x02\x17\n\r\n\x05\x04*\x02\x04\x05\x12\x04\xb2\x03\x02\
    \x08\n\r\n\x05\x04*\x02\x04\x01\x12\x04\xb2\x03\t\x12\n\r\n\x05\x04*\x02\
    \x04\x03\x12\x04\xb2\x03\x15\x16\n\x0c\n\x02\x04+\x12\x06\xb5\x03\0\xbb\
    \x03\x01\n\x0b\n\x03\x04+\x01\x12\x04\xb5\x03\x08\x19\n\x0c\n\x04\x04+\
    \x02\0\x12\x04\xb6\x03\x02\x12\n\r\n\x05\x04+\x02\0\x05\x12\x04\xb6\x03\
    \x02\x08\n\r\n\x05\x04+\x02\0\x01\x12\x04\xb6\x03\t\r\n\r\n\x05\x04+\x02\
    \0\x03\x12\x04\xb6\x03\x10\x11\n\x0c\n\x04\x04+\x02\x01\x12\x04\xb7\x03\
    \x02\x11\n\r\n\x05\x04+\x02\x01\x05\x12\x04\xb7\x03\x02\x08\n\r\n\x05\
    \x04+\x02\x01\x01\x12\x04\xb7\x03\t\x0c\n\r\n\x05\x04+\x02\x01\x03\x12\
    \x04\xb7\x03\x0f\x10\n\x0c\n\x04\x04+\x02\x02\x12\x04\xb8\x03\x02\x1f\n\
    \r\n\x05\x04+\x02\x02\x04\x12\x04\xb8\x03\x02\n\n\r\n\x05\x04+\x02\x02\
    \x05\x12\x04\xb8\x03\x0b\x11\n\r\n\x05\x04+\x02\x02\x01\x12\x04\xb8\x03\
    \x12\x1a\n\r\n\x05\x04+\x02\x02\x03\x12\x04\xb8\x03\x1d\x1e\n\x0c\n\x04\
    \x04+\x02\x03\x12\x04\xb9\x03\x02#\n\r\n\x05\x04+\x02\x03\x04\x12\x04\
    \xb9\x03\x02\n\n\r\n\x05\x04+\x02\x03\x05\x12\x04\xb9\x03\x0b\x11\n\r\n\
    \x05\x04+\x02\x03\x01\x12\x04\xb9\x03\x12\x1e\n\r\n\x05\x04+\x02\x03\x03\
    \x12\x04\xb9\x03!\"\n\x0c\n\x04\x04+\x02\x04\x12\x04\xba\x03\x02!\n\r\n\
    \x05\x04+\x02\x04\x04\x12\x04\xba\x03\x02\n\n\r\n\x05\x04+\x02\x04\x05\
    \x12\x04\xba\x03\x0b\x11\n\r\n\x05\x04+\x02\x04\x01\x12\x04\xba\x03\x12\
    \x1c\n\r\n\x05\x04+\x02\x04\x03\x12\x04\xba\x03\x1f\x20\n\x0c\n\x02\x04,\
    \x12\x06\xbd\x03\0\xc1\x03\x01\n\x0b\n\x03\x04,\x01\x12\x04\xbd\x03\x08\
    \x1c\n\x0c\n\x04\x04,\x02\0\x12\x04\xbe\x03\x02\x10\n\r\n\x05\x04,\x02\0\
    \x05\x12\x04\xbe\x03\x02\x08\n\r\n\x05\x04,\x02\0\x01\x12\x04\xbe\x03\t\
    \x0b\n\r\n\x05\x04,\x02\0\x03\x12\x04\xbe\x03\x0e\x0f\n\x0c\n\x04\x04,\
    \x02\x01\x12\x04\xbf\x03\x02\x10\n\r\n\x05\x04,\x02\x01\x05\x12\x04\xbf\
    \x03\x02\x06\n\r\n\x05\x04,\x02\x01\x01\x12\x04\xbf\x03\x07\x0b\n\r\n\
    \x05\x04,\x02\x01\x03\x12\x04\xbf\x03\x0e\x0f\n\x0c\n\x04\x04,\x02\x02\
    \x12\x04\xc0\x03\x02\x12\n\r\n\x05\x04,\x02\x02\x05\x12\x04\xc0\x03\x02\
    \x06\n\r\n\x05\x04,\x02\x02\x01\x12\x04\xc0\x03\x07\r\n\r\n\x05\x04,\x02\
    \x02\x03\x12\x04\xc0\x03\x10\x11\n\x0c\n\x02\x04-\x12\x06\xc3\x03\0\xc8\
    \x03\x01\n\x0b\n\x03\x04-\x01\x12\x04\xc3\x03\x08\x1b\n\x0c\n\x04\x04-\
    \x02\0\x12\x04\xc4\x03\x02\x12\n\r\n\x05\x04-\x02\0\x05\x12\x04\xc4\x03\
    \x02\x08\n\r\n\x05\x04-\x02\0\x01\x12\x04\xc4\x03\t\r\n\r\n\x05\x04-\x02\
    \0\x03\x12\x04\xc4\x03\x10\x11\n\x0c\n\x04\x04-\x02\x01\x12\x04\xc5\x03\
    \x02\x11\n\r\n\x05\x04-\x02\x01\x05\x12\x04\xc5\x03\x02\x08\n\r\n\x05\
    \x04-\x02\x01\x01\x12\x04\xc5\x03\t\x0c\n\r\n\x05\x04-\x02\x01\x03\x12\
    \x04\xc5\x03\x0f\x10\n\x0c\n\x04\x04-\x02\x02\x12\x04\xc6\x03\x02\x10\n\
    \r\n\x05\x04-\x02\x02\x05\x12\x04\xc6\x03\x02\x06\n\r\n\x05\x04-\x02\x02\
    \x01\x12\x04\xc6\x03\x07\x0b\n\r\n\x05\x04-\x02\x02\x03\x12\x04\xc6\x03\
    \x0e\x0f\n\x0c\n\x04\x04-\x02\x03\x12\x04\xc7\x03\x02\x13\n\r\n\x05\x04-\
    \x02\x03\x05\x12\x04\xc7\x03\x02\x06\n\r\n\x05\x04-\x02\x03\x01\x12\x04\
    \xc7\x03\x07\x0e\n\r\n\x05\x04-\x02\x03\x03\x12\x04\xc7\x03\x11\x12\n\
    \x0c\n\x02\x04.\x12\x06\xcb\x03\0\xd7\x03\x01\n\x0b\n\x03\x04.\x01\x12\
    \x04\xcb\x03\x08\x12\n\x0e\n\x04\x04.\x03\0\x12\x06\xcc\x03\x02\xd1\x03\
    \x03\n\r\n\x05\x04.\x03\0\x01\x12\x04\xcc\x03\n\x11\n\x0e\n\x06\x04.\x03\
    \0\x02\0\x12\x04\xcd\x03\x06\x19\n\x0f\n\x07\x04.\x03\0\x02\0\x05\x12\
    \x04\xcd\x03\x06\x0c\n\x0f\n\x07\x04.\x03\0\x02\0\x01\x12\x04\xcd\x03\r\
    \x14\n\x0f\n\x07\x04.\x03\0\x02\0\x03\x12\x04\xcd\x03\x17\x18\n\x0e\n\
    \x06\x04.\x03\0\x02\x01\x12\x04\xce\x03\x06\x16\n\x0f\n\x07\x04.\x03\0\
    \x02\x01\x05\x12\x04\xce\x03\x06\x0c\n\x0f\n\x07\x04.\x03\0\x02\x01\x01\
    \x12\x04\xce\x03\r\x11\n\x0f\n\x07\x04.\x03\0\x02\x01\x03\x12\x04\xce\
    \x03\x14\x15\n\x0e\n\x06\x04.\x03\0\x02\x02\x12\x04\xcf\x03\x06\x17\n\
    \x0f\n\x07\x04.\x03\0\x02\x02\x05\x12\x04\xcf\x03\x06\x0c\n\x0f\n\x07\
    \x04.\x03\0\x02\x02\x01\x12\x04\xcf\x03\r\x12\n\x0f\n\x07\x04.\x03\0\x02\
    \x02\x03\x12\x04\xcf\x03\x15\x16\n\x0e\n\x06\x04.\x03\0\x02\x03\x12\x04\
    \xd0\x03\x06\x18\n\x0f\n\x07\x04.\x03\0\x02\x03\x05\x12\x04\xd0\x03\x06\
    \x0c\n\x0f\n\x07\x04.\x03\0\x02\x03\x01\x12\x04\xd0\x03\r\x13\n\x0f\n\
    \x07\x04.\x03\0\x02\x03\x03\x12\x04\xd0\x03\x16\x17\n\x0c\n\x04\x04.\x02\
    \0\x12\x04\xd2\x03\x02\x12\n\r\n\x05\x04.\x02\0\x05\x12\x04\xd2\x03\x02\
    \x08\n\r\n\x05\x04.\x02\0\x01\x12\x04\xd2\x03\t\r\n\r\n\x05\x04.\x02\0\
    \x03\x12\x04\xd2\x03\x10\x11\n\x0c\n\x04\x04.\x02\x01\x12\x04\xd3\x03\
    \x02\x11\n\r\n\x05\x04.\x02\x01\x05\x12\x04\xd3\x03\x02\x08\n\r\n\x05\
    \x04.\x02\x01\x01\x12\x04\xd3\x03\t\x0c\n\r\n\x05\x04.\x02\x01\x03\x12\
    \x04\xd3\x03\x0f\x10\n\x0c\n\x04\x04.\x02\x02\x12\x04\xd4\x03\x02\x19\n\
    \r\n\x05\x04.\x02\x02\x05\x12\x04\xd4\x03\x02\x08\n\r\n\x05\x04.\x02\x02\
    \x01\x12\x04\xd4\x03\t\x14\n\r\n\x05\x04.\x02\x02\x03\x12\x04\xd4\x03\
    \x17\x18\n\x0c\n\x04\x04.\x02\x03\x12\x04\xd5\x03\x02\x19\n\r\n\x05\x04.\
    \x02\x03\x05\x12\x04\xd5\x03\x02\x08\n\r\n\x05\x04.\x02\x03\x01\x12\x04\
    \xd5\x03\t\x14\n\r\n\x05\x04.\x02\x03\x03\x12\x04\xd5\x03\x17\x18\n\x0c\
    \n\x04\x04.\x02\x04\x12\x04\xd6\x03\x02\x1f\n\r\n\x05\x04.\x02\x04\x04\
    \x12\x04\xd6\x03\x02\n\n\r\n\x05\x04.\x02\x04\x06\x12\x04\xd6\x03\x0b\
    \x12\n\r\n\x05\x04.\x02\x04\x01\x12\x04\xd6\x03\x13\x1a\n\r\n\x05\x04.\
    \x02\x04\x03\x12\x04\xd6\x03\x1d\x1e\n\x0c\n\x02\x04/\x12\x06\xda\x03\0\
    \xe7\x03\x01\n\x0b\n\x03\x04/\x01\x12\x04\xda\x03\x08\x15\n\x0e\n\x04\
    \x04/\x03\0\x12\x06\xdb\x03\x02\xe2\x03\x03\n\r\n\x05\x04/\x03\0\x01\x12\
    \x04\xdb\x03\n\x13\n\x0e\n\x06\x04/\x03\0\x02\0\x12\x04\xdc\x03\x04\x14\
    \n\x0f\n\x07\x04/\x03\0\x02\0\x05\x12\x04\xdc\x03\x04\n\n\x0f\n\x07\x04/\
    \x03\0\x02\0\x01\x12\x04\xdc\x03\x0b\x0f\n\x0f\n\x07\x04/\x03\0\x02\0\
    \x03\x12\x04\xdc\x03\x12\x13\n\x0e\n\x06\x04/\x03\0\x02\x01\x12\x04\xdd\
    \x03\x04\x13\n\x0f\n\x07\x04/\x03\0\x02\x01\x05\x12\x04\xdd\x03\x04\n\n\
    \x0f\n\x07\x04/\x03\0\x02\x01\x01\x12\x04\xdd\x03\x0b\x0e\n\x0f\n\x07\
    \x04/\x03\0\x02\x01\x03\x12\x04\xdd\x03\x11\x12\n\x0e\n\x06\x04/\x03\0\
    \x02\x02\x12\x04\xde\x03\x04\x1a\n\x0f\n\x07\x04/\x03\0\x02\x02\x05\x12\
    \x04\xde\x03\x04\n\n\x0f\n\x07\x04/\x03\0\x02\x02\x01\x12\x04\xde\x03\
    \x0b\x15\n\x0f\n\x07\x04/\x03\0\x02\x02\x03\x12\x04\xde\x03\x18\x19\n\
    \x0e\n\x06\x04/\x03\0\x02\x03\x12\x04\xdf\x03\x04\x12\n\x0f\n\x07\x04/\
    \x03\0\x02\x03\x05\x12\x04\xdf\x03\x04\n\n\x0f\n\x07\x04/\x03\0\x02\x03\
    \x01\x12\x04\xdf\x03\x0b\r\n\x0f\n\x07\x04/\x03\0\x02\x03\x03\x12\x04\
    \xdf\x03\x10\x11\n\x0c\n\x04\x04/\x02\0\x12\x04\xe3\x03\x02\x12\n\r\n\
    \x05\x04/\x02\0\x05\x12\x04\xe3\x03\x02\x08\n\r\n\x05\x04/\x02\0\x01\x12\
    \x04\xe3\x03\t\r\n\r\n\x05\x04/\x02\0\x03\x12\x04\xe3\x03\x10\x11\n\x0c\
    \n\x04\x04/\x02\x01\x12\x04\xe4\x03\x02\x11\n\r\n\x05\x04/\x02\x01\x05\
    \x12\x04\xe4\x03\x02\x08\n\r\n\x05\x04/\x02\x01\x01\x12\x04\xe4\x03\t\
    \x0c\n\r\n\x05\x04/\x02\x01\x03\x12\x04\xe4\x03\x0f\x10\n0\n\x04\x04/\
    \x02\x02\x12\x04\xe5\x03\x02\x13\"\"\x20\x20repeated\x20categoryBase\x20\
    rows\x20=\x204;\n\n\r\n\x05\x04/\x02\x02\x05\x12\x04\xe5\x03\x02\x08\n\r\
    \n\x05\x04/\x02\x02\x01\x12\x04\xe5\x03\t\x0e\n\r\n\x05\x04/\x02\x02\x03\
    \x12\x04\xe5\x03\x11\x12\n\x0c\n\x02\x040\x12\x06\xe9\x03\0\xed\x03\x01\
    \n\x0b\n\x03\x040\x01\x12\x04\xe9\x03\x08\x0f\n\x0c\n\x04\x040\x02\0\x12\
    \x04\xea\x03\x02\x12\n\r\n\x05\x040\x02\0\x05\x12\x04\xea\x03\x02\x08\n\
    \r\n\x05\x040\x02\0\x01\x12\x04\xea\x03\t\r\n\r\n\x05\x040\x02\0\x03\x12\
    \x04\xea\x03\x10\x11\n\x0c\n\x04\x040\x02\x01\x12\x04\xeb\x03\x02\x11\n\
    \r\n\x05\x040\x02\x01\x05\x12\x04\xeb\x03\x02\x08\n\r\n\x05\x040\x02\x01\
    \x01\x12\x04\xeb\x03\t\x0c\n\r\n\x05\x040\x02\x01\x03\x12\x04\xeb\x03\
    \x0f\x10\n\x0c\n\x04\x040\x02\x02\x12\x04\xec\x03\x02\x1b\n\r\n\x05\x040\
    \x02\x02\x04\x12\x04\xec\x03\x02\n\n\r\n\x05\x040\x02\x02\x05\x12\x04\
    \xec\x03\x0b\x11\n\r\n\x05\x040\x02\x02\x01\x12\x04\xec\x03\x12\x16\n\r\
    \n\x05\x040\x02\x02\x03\x12\x04\xec\x03\x19\x1ab\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
