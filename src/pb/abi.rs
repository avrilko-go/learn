#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(
        oneof = "command_request::RequestData",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag = "1")]
        HGet(super::HGet),
        #[prost(message, tag = "2")]
        HMGet(super::HmGet),
        #[prost(message, tag = "3")]
        HGetAll(super::HGetAll),
        #[prost(message, tag = "4")]
        HSet(super::HSet),
        #[prost(message, tag = "5")]
        HMSet(super::HmSet),
        #[prost(message, tag = "6")]
        HExist(super::HExist),
        #[prost(message, tag = "7")]
        HMExist(super::HmExist),
        #[prost(message, tag = "8")]
        HDel(super::HDel),
        #[prost(message, tag = "9")]
        HMDel(super::HmDel),
        #[prost(message, tag = "10")]
        Subscribe(super::Subscribe),
        #[prost(message, tag = "11")]
        UnSubscribe(super::UnSubscribe),
        #[prost(message, tag = "12")]
        Publish(super::Publish),
    }
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(uint32, tag = "1")]
    pub status: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    #[prost(message, repeated, tag = "4")]
    pub kv_pairs: ::prost::alloc::vec::Vec<KvPair>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HGet {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HmGet {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HGetAll {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HSet {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub kv_pair: ::core::option::Option<KvPair>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HmSet {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub kv_pairs: ::prost::alloc::vec::Vec<KvPair>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HExist {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HmExist {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HDel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct HmDel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct UnSubscribe {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Publish {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(double, tag = "2")]
        Float(f64),
        #[prost(bool, tag = "3")]
        Bool(bool),
        #[prost(bytes, tag = "4")]
        Binary(::prost::alloc::vec::Vec<u8>),
        #[prost(int64, tag = "5")]
        Int(i64),
    }
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
