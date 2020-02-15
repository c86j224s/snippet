// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `AuthServer.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Packet {
    // message fields
    pub id: Packet_Id,
    pub ping: ::protobuf::SingularPtrField<PING>,
    pub login: ::protobuf::SingularPtrField<LOGIN>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Packet {
    fn default() -> &'a Packet {
        <Packet as ::protobuf::Message>::default_instance()
    }
}

impl Packet {
    pub fn new() -> Packet {
        ::std::default::Default::default()
    }

    // .Packet.Id id = 1;


    pub fn get_id(&self) -> Packet_Id {
        self.id
    }
    pub fn clear_id(&mut self) {
        self.id = Packet_Id::PING;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: Packet_Id) {
        self.id = v;
    }

    // .PING ping = 2;


    pub fn get_ping(&self) -> &PING {
        self.ping.as_ref().unwrap_or_else(|| PING::default_instance())
    }
    pub fn clear_ping(&mut self) {
        self.ping.clear();
    }

    pub fn has_ping(&self) -> bool {
        self.ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: PING) {
        self.ping = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping(&mut self) -> &mut PING {
        if self.ping.is_none() {
            self.ping.set_default();
        }
        self.ping.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping(&mut self) -> PING {
        self.ping.take().unwrap_or_else(|| PING::new())
    }

    // .LOGIN login = 3;


    pub fn get_login(&self) -> &LOGIN {
        self.login.as_ref().unwrap_or_else(|| LOGIN::default_instance())
    }
    pub fn clear_login(&mut self) {
        self.login.clear();
    }

    pub fn has_login(&self) -> bool {
        self.login.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login(&mut self, v: LOGIN) {
        self.login = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login(&mut self) -> &mut LOGIN {
        if self.login.is_none() {
            self.login.set_default();
        }
        self.login.as_mut().unwrap()
    }

    // Take field
    pub fn take_login(&mut self) -> LOGIN {
        self.login.take().unwrap_or_else(|| LOGIN::new())
    }
}

impl ::protobuf::Message for Packet {
    fn is_initialized(&self) -> bool {
        for v in &self.ping {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.login {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.id, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login)?;
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
        if self.id != Packet_Id::PING {
            my_size += ::protobuf::rt::enum_size(1, self.id);
        }
        if let Some(ref v) = self.ping.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.login.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.id != Packet_Id::PING {
            os.write_enum(1, self.id.value())?;
        }
        if let Some(ref v) = self.ping.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.login.as_ref() {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Packet {
        Packet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Packet_Id>>(
                    "id",
                    |m: &Packet| { &m.id },
                    |m: &mut Packet| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PING>>(
                    "ping",
                    |m: &Packet| { &m.ping },
                    |m: &mut Packet| { &mut m.ping },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LOGIN>>(
                    "login",
                    |m: &Packet| { &m.login },
                    |m: &mut Packet| { &mut m.login },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Packet>(
                    "Packet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Packet {
        static mut instance: ::protobuf::lazy::Lazy<Packet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Packet,
        };
        unsafe {
            instance.get(Packet::new)
        }
    }
}

impl ::protobuf::Clear for Packet {
    fn clear(&mut self) {
        self.id = Packet_Id::PING;
        self.ping.clear();
        self.login.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Packet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Packet_Id {
    PING = 0,
    LOGIN = 1,
}

impl ::protobuf::ProtobufEnum for Packet_Id {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Packet_Id> {
        match value {
            0 => ::std::option::Option::Some(Packet_Id::PING),
            1 => ::std::option::Option::Some(Packet_Id::LOGIN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Packet_Id] = &[
            Packet_Id::PING,
            Packet_Id::LOGIN,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Packet_Id", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Packet_Id {
}

impl ::std::default::Default for Packet_Id {
    fn default() -> Self {
        Packet_Id::PING
    }
}

impl ::protobuf::reflect::ProtobufValue for Packet_Id {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PING {
    // message fields
    pub now_utc: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PING {
    fn default() -> &'a PING {
        <PING as ::protobuf::Message>::default_instance()
    }
}

impl PING {
    pub fn new() -> PING {
        ::std::default::Default::default()
    }

    // uint64 now_utc = 1;


    pub fn get_now_utc(&self) -> u64 {
        self.now_utc
    }
    pub fn clear_now_utc(&mut self) {
        self.now_utc = 0;
    }

    // Param is passed by value, moved
    pub fn set_now_utc(&mut self, v: u64) {
        self.now_utc = v;
    }
}

impl ::protobuf::Message for PING {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.now_utc = tmp;
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
        if self.now_utc != 0 {
            my_size += ::protobuf::rt::value_size(1, self.now_utc, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.now_utc != 0 {
            os.write_uint64(1, self.now_utc)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PING {
        PING::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "now_utc",
                    |m: &PING| { &m.now_utc },
                    |m: &mut PING| { &mut m.now_utc },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PING>(
                    "PING",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PING {
        static mut instance: ::protobuf::lazy::Lazy<PING> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PING,
        };
        unsafe {
            instance.get(PING::new)
        }
    }
}

impl ::protobuf::Clear for PING {
    fn clear(&mut self) {
        self.now_utc = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PING {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PING {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LOGIN {
    // message fields
    pub account: ::protobuf::SingularPtrField<super::Common::Account>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LOGIN {
    fn default() -> &'a LOGIN {
        <LOGIN as ::protobuf::Message>::default_instance()
    }
}

impl LOGIN {
    pub fn new() -> LOGIN {
        ::std::default::Default::default()
    }

    // .Account account = 1;


    pub fn get_account(&self) -> &super::Common::Account {
        self.account.as_ref().unwrap_or_else(|| super::Common::Account::default_instance())
    }
    pub fn clear_account(&mut self) {
        self.account.clear();
    }

    pub fn has_account(&self) -> bool {
        self.account.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account(&mut self, v: super::Common::Account) {
        self.account = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account(&mut self) -> &mut super::Common::Account {
        if self.account.is_none() {
            self.account.set_default();
        }
        self.account.as_mut().unwrap()
    }

    // Take field
    pub fn take_account(&mut self) -> super::Common::Account {
        self.account.take().unwrap_or_else(|| super::Common::Account::new())
    }
}

impl ::protobuf::Message for LOGIN {
    fn is_initialized(&self) -> bool {
        for v in &self.account {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.account)?;
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
        if let Some(ref v) = self.account.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.account.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LOGIN {
        LOGIN::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Common::Account>>(
                    "account",
                    |m: &LOGIN| { &m.account },
                    |m: &mut LOGIN| { &mut m.account },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LOGIN>(
                    "LOGIN",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static LOGIN {
        static mut instance: ::protobuf::lazy::Lazy<LOGIN> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LOGIN,
        };
        unsafe {
            instance.get(LOGIN::new)
        }
    }
}

impl ::protobuf::Clear for LOGIN {
    fn clear(&mut self) {
        self.account.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LOGIN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LOGIN {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10AuthServer.proto\x1a\x0cCommon.proto\"x\n\x06Packet\x12\x1a\n\x02i\
    d\x18\x01\x20\x01(\x0e2\n.Packet.IdR\x02id\x12\x19\n\x04ping\x18\x02\x20\
    \x01(\x0b2\x05.PINGR\x04ping\x12\x1c\n\x05login\x18\x03\x20\x01(\x0b2\
    \x06.LOGINR\x05login\"\x19\n\x02Id\x12\x08\n\x04PING\x10\0\x12\t\n\x05LO\
    GIN\x10\x01\"\x1f\n\x04PING\x12\x17\n\x07now_utc\x18\x01\x20\x01(\x04R\
    \x06nowUtc\"+\n\x05LOGIN\x12\"\n\x07account\x18\x01\x20\x01(\x0b2\x08.Ac\
    countR\x07accountb\x06proto3\
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
