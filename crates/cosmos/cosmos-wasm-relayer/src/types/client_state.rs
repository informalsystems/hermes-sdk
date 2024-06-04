use core::time::Duration;

use cgp_core::Async;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use ibc::core::client::types::Height as IbcHeight;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::Height;
use prost_types::Any;

pub struct WrappedTendermintClientState {
    pub tendermint_client_state: TendermintClientState,
    pub wasm_code_hash: Vec<u8>,
}

impl From<WrappedTendermintClientState> for TendermintClientState {
    fn from(value: WrappedTendermintClientState) -> Self {
        value.tendermint_client_state
    }
}

pub struct ProvideWasmTendermintClientStateType;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideWasmTendermintClientStateType
where
    Chain: Async,
{
    type ClientState = WrappedTendermintClientState;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideWasmTendermintClientStateType
where
    Chain: HasClientStateType<Counterparty, ClientState = WrappedTendermintClientState>
        + HasHeightType<Height = Height>,
{
    fn client_state_latest_height(client_state: &WrappedTendermintClientState) -> Height {
        client_state.tendermint_client_state.latest_height
    }

    fn client_state_is_frozen(client_state: &WrappedTendermintClientState) -> bool {
        client_state.tendermint_client_state.is_frozen()
    }

    fn client_state_has_expired(
        client_state: &WrappedTendermintClientState,
        elapsed: Duration,
    ) -> bool {
        elapsed > client_state.tendermint_client_state.trusting_period
    }
}

pub struct EncodeWrappedTendermintClientState;

impl<Encoding> Converter<Encoding, WrappedTendermintClientState, Any>
    for EncodeWrappedTendermintClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<Any, TendermintClientState>
        + CanConvert<WasmClientState, Any>,
{
    fn convert(
        encoding: &Encoding,
        client_state: &WrappedTendermintClientState,
    ) -> Result<Any, Encoding::Error> {
        let tendermint_client_state_bytes =
            encoding.encode(&client_state.tendermint_client_state)?;

        let latest_height = client_state.tendermint_client_state.latest_height;

        let wasm_client_state = WasmClientState {
            data: tendermint_client_state_bytes,
            checksum: client_state.wasm_code_hash.clone(),
            latest_height: IbcHeight::new(
                latest_height.revision_number(),
                latest_height.revision_height(),
            )
            .unwrap(),
        };

        encoding.convert(&wasm_client_state)
    }
}

impl<Encoding> Converter<Encoding, Any, WrappedTendermintClientState>
    for EncodeWrappedTendermintClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<Any, TendermintClientState>
        + CanConvert<Any, WasmClientState>,
{
    fn convert(
        encoding: &Encoding,
        client_state_any: &Any,
    ) -> Result<WrappedTendermintClientState, Encoding::Error> {
        let wasm_client_state = encoding.convert(client_state_any)?;

        let tendermint_client_state = encoding.decode(&wasm_client_state.data)?;

        let wrapped_tendermint_client_state = WrappedTendermintClientState {
            tendermint_client_state,
            wasm_code_hash: wasm_client_state.checksum,
        };

        Ok(wrapped_tendermint_client_state)
    }
}
