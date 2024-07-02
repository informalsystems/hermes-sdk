use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::{CanConvert, CanConvertBothWays};
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_protobuf_encoding_components::types::Protobuf;
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

impl<Context> ProvideEncodingType<Context> for ProvideWasmCosmosEncoding
where
    Context: Async,
{
    type Encoding = WasmCosmosEncoding;
}

impl<Context> DefaultEncodingGetter<Context> for ProvideWasmCosmosEncoding
where
    Context: HasEncodingType<Encoding = WasmCosmosEncoding>,
{
    fn default_encoding() -> &'static WasmCosmosEncoding {
        &WasmCosmosEncoding
    }
}

pub trait CheckWasmCosmosEncoding:
    HasEncodedType<Encoded = Vec<u8>>
    + CanEncodeAndDecode<Protobuf, TendermintClientState>
    + CanEncodeAndDecode<Any, TendermintClientState>
    + CanEncodeAndDecode<Protobuf, TendermintConsensusState>
    + CanEncodeAndDecode<Any, TendermintConsensusState>
    + CanConvertBothWays<Any, TendermintClientState>
    + CanConvertBothWays<Any, TendermintConsensusState>
    + CanEncodeAndDecode<Protobuf, ProtoWasmClientState>
    + CanEncode<Protobuf, WasmClientState>
    + CanConvert<WasmClientState, ProtoWasmClientState>
    + CanConvert<ProtoWasmClientState, WasmClientState>
    + CanEncodeAndDecode<Any, WasmClientState>
    + CanEncodeAndDecode<Any, WasmConsensusState>
    + CanConvertBothWays<Any, WrappedTendermintClientState>
    + CanConvert<WasmClientState, Any>
    + CanConvert<WasmConsensusState, Any>
    + CanEncode<Any, TendermintClientState>
    + CanEncode<Any, TendermintConsensusState>
{
}

impl CheckWasmCosmosEncoding for WasmCosmosEncoding {}
