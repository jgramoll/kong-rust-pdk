/*
   .google.protobuf.Value doesnt' support serde serialize
*/

use prost_types::NullValue;

#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Struct {
    /// Unordered map of dynamically typed values.
    #[prost(btree_map = "string, message", tag = "1")]
    pub fields: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, Value>,
}

#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof, serde::Serialize, serde::Deserialize)]
    pub enum Kind {
        /// Represents a null value.
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        /// Represents a double value.
        #[prost(double, tag = "2")]
        NumberValue(f64),
        /// Represents a string value.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// Represents a structured value.
        #[prost(message, tag = "5")]
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        #[prost(message, tag = "6")]
        ListValue(super::ListValue),
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
