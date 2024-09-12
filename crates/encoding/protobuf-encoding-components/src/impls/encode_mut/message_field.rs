use crate::impls::encode_mut::message::EncodeProstMessage;
use crate::impls::encode_mut::proto_field::EncodeProtoField;

pub type EncodeProstMessageField<const TAG: u32> = EncodeProtoField<EncodeProstMessage, TAG>;
