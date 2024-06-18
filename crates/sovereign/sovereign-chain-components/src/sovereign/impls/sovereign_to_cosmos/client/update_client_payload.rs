use std::time::Duration;

use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::payload_builders::update_client::{
    CanBuildUpdateClientPayload, UpdateClientPayloadBuilder,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_sovereign_rollup_components::impls::queries::slot_hash::CanQuerySlotHash;
use hermes_sovereign_rollup_components::types::client_state::WrappedSovereignClientState;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use ibc::core::client::types::error::ClientError as IbcClientError;
use ibc::core::client::types::Height as IbcHeight;
use ibc_relayer_types::clients::ics07_tendermint::client_state::AllowUpdate;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold as RelayerTrustThreshold;
use ibc_relayer_types::core::ics23_commitment::specs::ProofSpecs;
use ibc_relayer_types::core::ics24_host::identifier::ChainId as RelayerChainId;
use sov_celestia_client::types::client_state::TendermintClientParams;

use crate::sovereign::traits::chain::data_chain::HasDataChain;
use crate::sovereign::traits::chain::rollup::HasRollup;
use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

/**
   Build an update client payload from a Sovereign rollup, to be used later
   for sending an update client message to a Cosmos counterparty chain.
*/
pub struct BuildSovereignUpdateClientPayload;

impl<Chain, Counterparty, DataChain, Rollup> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignUpdateClientPayload
where
    Chain: HasHeightType<Height = RollupHeight>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = SovereignUpdateClientPayload>
        + HasClientStateType<Counterparty, ClientState = WrappedSovereignClientState>
        + HasRollup<Rollup = Rollup>
        + HasDataChain<DataChain = DataChain>
        + CanRaiseError<Rollup::Error>
        + CanRaiseError<DataChain::Error>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<IbcClientError>,
    DataChain: HasHeightType<Height = Height>
        + HasClientStateType<Counterparty, ClientState = TendermintClientState>
        + CanBuildUpdateClientPayload<Counterparty, UpdateClientPayload = CosmosUpdateClientPayload>,
    Rollup: HasHeightType<Height = RollupHeight> + CanQuerySlotHash,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &RollupHeight,
        target_height: &RollupHeight,
        client_state: Chain::ClientState,
    ) -> Result<SovereignUpdateClientPayload, Chain::Error> {
        let rollup = chain.rollup();
        let data_chain = chain.data_chain();

        let sovereign_params = &client_state.sovereign_client_state.sovereign_params;

        // DA height is higher than rollup height. This requires adding
        // the genesis Height to the trusted and target Heights
        let da_trusted_height = Height::new(
            sovereign_params.genesis_da_height.revision_number(),
            trusted_height.slot_number + sovereign_params.genesis_da_height.revision_height(),
        )
        .map_err(Chain::raise_error)?;

        let da_target_height = Height::new(
            sovereign_params.genesis_da_height.revision_number(),
            target_height.slot_number + sovereign_params.genesis_da_height.revision_height(),
        )
        .map_err(Chain::raise_error)?;

        let rollup_trusted_height = IbcHeight::new(
            sovereign_params.latest_height.revision_number(),
            trusted_height.slot_number,
        )
        .map_err(Chain::raise_error)?;

        let rollup_target_height = IbcHeight::new(
            sovereign_params.latest_height.revision_number(),
            target_height.slot_number,
        )
        .map_err(Chain::raise_error)?;

        let da_client_state = convert_tm_params_to_client_state(
            &client_state.sovereign_client_state.da_params,
            &da_target_height,
        )
        .map_err(Chain::raise_error)?;

        let da_payload = data_chain
            .build_update_client_payload(&da_trusted_height, &da_target_height, da_client_state)
            .await
            .map_err(Chain::raise_error)?;

        let slot_hash = rollup
            .query_slot_hash(target_height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(SovereignUpdateClientPayload {
            datachain_header: da_payload.headers,
            initial_state_height: rollup_trusted_height,
            final_state_height: rollup_target_height,
            final_user_hash: slot_hash.user_hash,
            final_kernel_hash: slot_hash.kernel_hash,
            final_root_hash: slot_hash.root_hash,
        })
    }
}

/// This is a temporary solution which converts the TendermintParams to Tendermint ClientState.
/// The Sovereign client state only has a TendermintParams field, but in order to build the
/// client update payload, the DA chain's client state is required.
/// Until the Light client is decoupled from the Cosmos SDK in order to build the DA header
/// half the Tendermint ClientState value are mocked.
/// See issue: https://github.com/informalsystems/hermes-sdk/issues/204
fn convert_tm_params_to_client_state(
    tm_params: &TendermintClientParams,
    da_target_height: &Height,
) -> Result<TendermintClientState, Ics02Error> {
    let relayer_chain_id = RelayerChainId::from_string(&tm_params.chain_id.to_string());

    let relayer_trust_threshold = RelayerTrustThreshold::new(
        tm_params.trust_level.numerator(),
        tm_params.trust_level.denominator(),
    )?;

    Ok(TendermintClientState {
        chain_id: relayer_chain_id,
        trust_threshold: relayer_trust_threshold,
        // trusting_period was removed from `TendermintClientParams`
        // https://github.com/informalsystems/sovereign-ibc/commit/a9aaa80c4fe7b21fa777ae2a186838aac1fed68c#diff-8735596286f5213c6003fc9dc4c719fe9c9d4f14b7a385f1418f766ef48faa54L17
        trusting_period: Duration::from_secs(300),
        unbonding_period: tm_params.unbonding_period,
        max_clock_drift: tm_params.max_clock_drift,
        latest_height: *da_target_height,
        proof_specs: ProofSpecs::default(),
        upgrade_path: vec![],
        allow_update: AllowUpdate {
            after_expiry: false,
            after_misbehaviour: false,
        },
        frozen_height: None,
    })
}
