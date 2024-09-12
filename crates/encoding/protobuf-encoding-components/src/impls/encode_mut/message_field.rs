use crate::impls::encode_mut::field::EncodeField;
use crate::impls::encode_mut::message::EncodeProstMessage;

pub type EncodeProstMessageField<const TAG: u32> = EncodeField<EncodeProstMessage, TAG>;
