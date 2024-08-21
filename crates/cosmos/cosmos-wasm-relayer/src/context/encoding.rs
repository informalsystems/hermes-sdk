use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::{CanConvert, CanConvertBothWays};
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::{ViaAny, ViaProtobuf};
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

use crate::encoding::components::*;
use crate::types::client_state::WrappedTendermintClientState;

pub struct WasmCosmosEncoding;

pub struct WasmCosmosEncodingComponents2;

impl HasComponents for WasmCosmosEncoding {
    type Components = WasmCosmosEncodingComponents2;
}

with_wasm_cosmos_encoding_components! {
    delegate_components! {
        WasmCosmosEncodingComponents2 {
            @WasmCosmosEncodingComponents: WasmCosmosEncodingComponents,
        }
    }
}

delegate_components! {
    WasmCosmosEncodingComponents2 {
        [
            ErrorTypeComponent,
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

impl<Context> ProvideEncodingType<Context, AsBytes> for ProvideWasmCosmosEncoding
where
    Context: Async,
{
    type Encoding = WasmCosmosEncoding;
}

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
    + CanEncodeAndDecode<ViaProtobuf, ProtoWasmClientState>
    + CanEncode<ViaProtobuf, WasmClientState>
    + CanConvert<WasmClientState, ProtoWasmClientState>
    + CanConvert<ProtoWasmClientState, WasmClientState>
    + CanEncodeAndDecode<ViaAny, WasmClientState>
    + CanEncodeAndDecode<ViaAny, WasmConsensusState>
    + CanConvertBothWays<Any, WrappedTendermintClientState>
    + CanConvert<WasmClientState, Any>
    + CanConvert<WasmConsensusState, Any>
    + CanEncode<ViaAny, TendermintClientState>
    + CanEncode<ViaAny, TendermintConsensusState>
{
}

impl CheckWasmCosmosEncoding for WasmCosmosEncoding {}
