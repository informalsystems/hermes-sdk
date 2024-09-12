use cgp::prelude::*;
use prost::encoding::WireType;

pub trait CanMergeProtoValue<Value>: HasErrorType {
    fn merge(tag: u32, wire_type: WireType, value: &mut Value);
}
