//! Helper functions for encoding protobuf messages into bytes.

use ibc_proto::google::protobuf::Any;
use prost::Message as ProstMessage;

pub fn encode_protobuf<Message>(message: &Message) -> Vec<u8>
where
    Message: ProstMessage,
{
    Message::encode_to_vec(message)
}

pub fn encode_to_any<Message>(type_url: &str, message: &Message) -> Any
where
    Message: ProstMessage,
{
    let encoded_message = Message::encode_to_vec(message);

    Any {
        type_url: type_url.to_string(),
        value: encoded_message,
    }
}

pub fn encode_any_to_bytes<Message>(type_url: &str, message: &Message) -> Vec<u8>
where
    Message: ProstMessage,
{
    let any = encode_to_any(type_url, message);

    encode_protobuf(&any)
}
