use crate::encoding::components::{
    IsWasmCosmosEncodingComponent, WasmCosmosEncodingComponents as BaseWasmCosmosEncodingComponents,
};
use cgp_core::prelude::*;
use cgp_core::{delegate_all, CanRaiseError, ErrorRaiserComponent, ErrorTypeComponent};
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
use ibc::core::client::types::error::ClientError;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

use crate::types::client_state::WrappedTendermintClientState;

pub struct WasmCosmosEncoding;

pub struct WasmCosmosEncodingComponents;

impl HasComponents for WasmCosmosEncoding {
    type Components = WasmCosmosEncodingComponents;
}

delegate_all!(
    IsWasmCosmosEncodingComponent,
    BaseWasmCosmosEncodingComponents,
    WasmCosmosEncodingComponents,
);

delegate_components! {
    WasmCosmosEncodingComponents {
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
// + CanConvertBothWays<Any, WrappedTendermintClientState>
{
}

impl CheckWasmCosmosEncoding for WasmCosmosEncoding {}
