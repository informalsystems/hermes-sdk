#![allow(non_snake_case)]
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_cosmos::methods::encode::encode_to_any;
use prost::Message;
use secp256k1::PublicKey as SecpPublicKey;

use crate::types::error::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey {
    pub key: SecpPublicKey,
}

impl PublicKey {
    pub fn from_secp256k1_key(key: SecpPublicKey) -> Self {
        Self { key }
    }
}

/// PubKey defines a secp256k1 public key
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// PrivKey defines a secp256k1 private key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl From<&PublicKey> for PubKey {
    fn from(key: &PublicKey) -> Self {
        let key_bytes = key.key.serialize_uncompressed();
        Self {
            key: key_bytes.to_vec(),
        }
    }
}

const TYPE_URL: &str = "/cosmos.crypto.secp256k1.PubKey";

pub fn encode_public_key(public_key: &PublicKey) -> Any {
    let key = PubKey::from(public_key);

    encode_to_any(TYPE_URL, &key).unwrap()
}

impl Protobuf<PubKey> for PublicKey {}

impl TryFrom<PubKey> for PublicKey {
    type Error = Error;

    fn try_from(raw_msg: PubKey) -> Result<Self, Self::Error> {
        let key = SecpPublicKey::from_slice(&raw_msg.key).unwrap();
        Ok(PublicKey { key })
    }
}

impl From<PublicKey> for PubKey {
    fn from(domain_msg: PublicKey) -> Self {
        let key = domain_msg.key.serialize();
        PubKey { key: key.to_vec() }
    }
}

pub fn decode_public_key_from_any(buf: Any) -> PublicKey {
    let proto_state = PubKey::decode(buf.value.as_ref()).unwrap();

    let public_key = proto_state.try_into().unwrap();

    public_key
}

// decode raw -> Any -> Proto -> Domain
pub fn decode_public_key(buf: &[u8]) -> PublicKey {
    let any_value = Any::decode(buf).unwrap();
    let proto_state = PubKey::decode(any_value.value.as_ref()).unwrap();

    let public_key = proto_state.try_into().unwrap();

    public_key
}
