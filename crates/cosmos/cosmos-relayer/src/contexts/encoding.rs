use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::*;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::impls::encode_mut::chunk::ProtoChunks;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use ibc::core::client::types::Height;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost::bytes::BufMut;
use prost_types::Any;

use crate::impls::error::HandleCosmosError;

pub struct CosmosEncoding;

pub struct CosmosEncodingComponents2;

impl HasComponents for CosmosEncoding {
    type Components = CosmosEncodingComponents2;
}

with_cosmos_encoding_components! {
    delegate_components! {
        CosmosEncodingComponents2 {
            @CosmosEncodingComponents: CosmosEncodingComponents,
        }
    }
}

delegate_components! {
    CosmosEncodingComponents2 {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
    }
}

pub struct ProvideCosmosEncoding;

delegate_components! {
    ProvideCosmosEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

impl<Context> ProvideEncodingType<Context, AsBytes> for ProvideCosmosEncoding
where
    Context: Async,
{
    type Encoding = CosmosEncoding;
}

impl<Context> DefaultEncodingGetter<Context, AsBytes> for ProvideCosmosEncoding
where
    Context: HasEncodingType<AsBytes, Encoding = CosmosEncoding>,
{
    fn default_encoding() -> &'static CosmosEncoding {
        &CosmosEncoding
    }
}

pub trait CheckCosmosEncoding:
    HasEncodedType<Encoded = Vec<u8>>
    + HasEncodeBufferType<EncodeBuffer = Vec<u8>>
    + HasEncodeBufferType<EncodeBuffer: BufMut>
    + for<'a> HasDecodeBufferType<DecodeBuffer<'a> = ProtoChunks<'a>>
    + CanEncodeAndDecode<ViaProtobuf, Vec<u8>>
    + CanEncodeAndDecode<ViaProtobuf, TendermintClientState>
    + CanEncodeAndDecode<ViaProtobuf, TendermintConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, MerkleProof>
    + CanEncodeAndDecode<ViaAny, TendermintClientState>
    + CanEncodeAndDecode<ViaAny, TendermintConsensusState>
    + CanConvertBothWays<Any, TendermintClientState>
    + CanConvertBothWays<Any, TendermintConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, String>
    + CanEncode<ViaProtobuf, Height>
{
}

impl CheckCosmosEncoding for CosmosEncoding {}

#[cfg(test)]
#[test]
fn test_height_encoding() {
    use ibc_proto::Protobuf;

    let height = Height::new(8888, 9999).unwrap();

    let bytes1 = height.encode_vec();

    let bytes2 = CosmosEncoding.encode(&height).unwrap();

    assert_eq!(bytes1, bytes2);
}
