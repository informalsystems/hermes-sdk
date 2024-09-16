use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::*;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
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

pub struct CosmosEncodingContextComponents;

impl HasComponents for CosmosEncoding {
    type Components = CosmosEncodingContextComponents;
}

with_cosmos_client_encoding_components! {
    delegate_components! {
        CosmosEncodingContextComponents {
            @CosmosClientEncodingComponents: CosmosClientEncodingComponents,
        }
    }
}

delegate_components! {
    CosmosEncodingContextComponents {
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
    + CanEncodeAndDecode<ViaProtobuf, Height>
{
}

impl CheckCosmosEncoding for CosmosEncoding {}

#[cfg(test)]
mod test {
    use hermes_encoding_components::traits::decode::CanDecode;
    use hermes_encoding_components::traits::encode::CanEncode;
    use hermes_error::types::HermesError;
    use ibc::core::client::types::Height;
    use ibc_proto::Protobuf;

    use crate::contexts::encoding::CosmosEncoding;

    #[test]
    fn test_height_encoding() -> Result<(), HermesError> {
        let height = Height::new(1, 12)?;

        let bytes1 = height.encode_vec();

        println!("bytes1: {:?}", bytes1);

        let bytes2 = CosmosEncoding.encode(&height)?;

        println!("bytes2: {:?}", bytes2);

        assert_eq!(bytes1, bytes2);

        let height2: Height = CosmosEncoding.decode(&bytes2)?;

        assert_eq!(height, height2);

        Ok(())
    }
}
