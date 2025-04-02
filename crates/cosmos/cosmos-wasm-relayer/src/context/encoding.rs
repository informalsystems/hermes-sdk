use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::{CanConvert, CanConvertBothWays};
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetterComponent,
    EncodingTypeProvider, EncodingTypeProviderComponent, HasEncodingType,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_wasm_encoding_components::types::client_message::WasmClientMessage;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::core::client::types::Height;
use prost_types::Any;

use crate::encoding::components::*;
use crate::types::client_state::WasmTendermintClientState;

#[cgp_context(WasmCosmosEncodingContextComponents: WasmCosmosEncodingComponents)]
pub struct WasmCosmosEncoding;

delegate_components! {
    WasmCosmosEncodingContextComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
    }
}

pub struct ProvideWasmCosmosEncoding;

delegate_components! {
    ProvideWasmCosmosEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

#[cgp_provider(EncodingTypeProviderComponent)]
impl<Context> EncodingTypeProvider<Context, AsBytes> for ProvideWasmCosmosEncoding
where
    Context: Async,
{
    type Encoding = WasmCosmosEncoding;
}

#[cgp_provider(DefaultEncodingGetterComponent)]
impl<Context> DefaultEncodingGetter<Context, AsBytes> for ProvideWasmCosmosEncoding
where
    Context: HasEncodingType<AsBytes, Encoding = WasmCosmosEncoding>,
{
    fn default_encoding() -> &'static WasmCosmosEncoding {
        &WasmCosmosEncoding
    }
}

pub trait CheckWasmCosmosEncoding:
    HasEncodedType<Encoded = Vec<u8>>
    + CanEncodeAndDecode<ViaProtobuf, TendermintClientState>
    + CanEncodeAndDecode<ViaAny, TendermintClientState>
    + CanEncodeAndDecode<ViaProtobuf, TendermintConsensusState>
    + CanEncodeAndDecode<ViaAny, TendermintConsensusState>
    + CanConvertBothWays<Any, TendermintClientState>
    + CanConvertBothWays<Any, TendermintConsensusState>
    + CanEncode<ViaProtobuf, WasmClientState>
    + CanEncodeAndDecode<ViaAny, WasmClientState>
    + CanEncodeAndDecode<ViaAny, WasmConsensusState>
    + CanEncodeAndDecode<ViaAny, WasmClientMessage>
    + CanConvertBothWays<Any, WasmTendermintClientState>
    + CanConvert<WasmClientState, Any>
    + CanConvert<WasmConsensusState, Any>
    + CanEncode<ViaAny, TendermintClientState>
    + CanEncode<ViaAny, TendermintConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, Height>
    + CanEncodeAndDecode<ViaProtobuf, WasmClientState>
    + CanEncodeAndDecode<ViaProtobuf, WasmConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, WasmClientMessage>
{
}

impl CheckWasmCosmosEncoding for WasmCosmosEncoding {}

#[cfg(test)]
mod test {
    use hermes_encoding_components::traits::decode::Decoder;
    use hermes_encoding_components::traits::encode::{CanEncode, Encoder};
    use hermes_error::types::HermesError;
    use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
    use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
    use hermes_wasm_encoding_components::types::client_state::WasmClientState;
    use ibc::core::client::types::Height;

    use crate::context::encoding::WasmCosmosEncoding;

    #[test]
    fn test_wasm_client_state_encoding() -> Result<(), HermesError> {
        let wasm_client_state = WasmClientState {
            data: vec![1, 2, 3],
            checksum: vec![4, 5, 6],
            latest_height: Height::new(0, 1)?,
        };

        let bytes1 = <WasmCosmosEncoding as CanEncode<ViaProtobuf, WasmClientState>>::encode(
            &WasmCosmosEncoding,
            &wasm_client_state,
        )?;

        println!("bytes1: {:?}", bytes1);

        let bytes2 = <EncodeProtoWithMutBuffer as Encoder<
            WasmCosmosEncoding,
            ViaProtobuf,
            WasmClientState,
        >>::encode(&WasmCosmosEncoding, &wasm_client_state)?;

        println!("bytes2: {:?}", bytes2);

        assert_eq!(bytes1, bytes2);

        let client_state_2 = <EncodeProtoWithMutBuffer as Decoder<
            WasmCosmosEncoding,
            ViaProtobuf,
            WasmClientState,
        >>::decode(&WasmCosmosEncoding, &bytes1)?;

        println!("decoded client state: {:?}", client_state_2);

        assert_eq!(client_state_2, wasm_client_state);

        Ok(())
    }
}
