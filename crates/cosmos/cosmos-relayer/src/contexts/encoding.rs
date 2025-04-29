use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_core::encoding_components::impls::GetDefaultEncoding;
use hermes_core::encoding_components::traits::{
    CanConvertBothWays, CanEncodeAndDecode, CanEncodeAndDecodeMut, DefaultEncodingGetter,
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeProviderComponent,
    HasDecodeBufferType, HasEncodeBufferType, HasEncodedType, HasEncodingType,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_core::chain_components::types::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_core::protobuf_encoding_components::impls::ProtoChunks;
use hermes_cosmos_core::protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc::primitives::Timestamp;
use prost::bytes::BufMut;
use prost_types::Any;

use crate::impls::HandleCosmosError;

#[cgp_context(CosmosEncodingContextComponents: CosmosClientEncodingComponents)]
pub struct CosmosEncoding;

delegate_components! {
    CosmosEncodingContextComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
    }
}

pub struct UseCosmosEncoding;

delegate_components! {
    UseCosmosEncoding {
        EncodingTypeProviderComponent<AsBytes>:
            UseType<CosmosEncoding>,
        EncodingGetterComponent<AsBytes>:
            GetDefaultEncoding,
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsBytes>)]
impl<Context> DefaultEncodingGetter<Context, AsBytes> for UseCosmosEncoding
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
    + CanEncodeAndDecodeMut<ViaProtobuf, Height>
    + CanEncodeAndDecodeMut<ViaProtobuf, Timestamp>
    + CanEncodeAndDecodeMut<ViaProtobuf, CommitmentRoot>
{
}

impl CheckCosmosEncoding for CosmosEncoding {}

#[cfg(test)]
mod test {
    use hermes_core::encoding_components::traits::{CanDecode, CanEncode};
    use hermes_error::types::HermesError;
    use ibc::core::client::types::Height;
    use ibc_proto::Protobuf;

    use crate::contexts::CosmosEncoding;

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
