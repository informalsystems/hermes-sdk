use std::iter;
use std::str::FromStr;

use cgp_core::HasErrorType;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::clients::tendermint::types::Header;
use ibc_core::client::types::Height as DataChainHeight;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::core::ics02_client::header::AnyHeader;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::ClientId as RelayerClientId;

use crate::sovereign::traits::chain::data_chain::{HasDataChain, HasDataChainType};
use crate::sovereign::types::client_state::SovereignClientState;
use crate::sovereign::types::height::RollupHeight;
use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

/**
   Build an update client payload from a Sovereign rollup, to be used later
   for sending an update client message to a Cosmos counterparty chain.
*/
pub struct BuildSovereignUpdateClientPayload;

impl<Chain, Counterparty, DataChain> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignUpdateClientPayload
where
    Chain: HasHeightType<Height = RollupHeight>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = SovereignUpdateClientPayload>
        + HasClientStateType<Counterparty, ClientState = SovereignClientState>
        + HasDataChain
        + HasDataChainType<DataChain = DataChain>
        + HasErrorType,
    Chain::DataChain: CanQueryClientState<Counterparty>
        + HasIbcChainTypes<Counterparty, ClientId = RelayerClientId, Height = Height>
        + HasBlockingChainHandle,
    // TODO: Add dependencies for update client payload here
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &RollupHeight,
        target_height: &RollupHeight,
        _client_state: Chain::ClientState,
    ) -> Result<SovereignUpdateClientPayload, Chain::Error> {
        let tm_trusted_height = Height::new(1, trusted_height.slot_number as u64).unwrap();
        let tm_target_height = Height::new(1, target_height.slot_number as u64).unwrap();
        let da_trusted_height = DataChainHeight::new(1, trusted_height.slot_number as u64).unwrap();
        let da_target_height = DataChainHeight::new(1, target_height.slot_number as u64).unwrap();

        let dummy_da_client_id = RelayerClientId::from_str("07-tendermint-1").unwrap();

        let data_chain = chain.data_chain();

        let da_client_state = data_chain
            .query_client_state(&dummy_da_client_id, &tm_target_height)
            .await
            .unwrap();

        let headers = data_chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (header, support) = chain_handle
                    .build_header(
                        tm_trusted_height,
                        tm_target_height,
                        AnyClientState::Tendermint(da_client_state),
                    )
                    .unwrap();

                let headers = iter::once(header)
                    .chain(support.into_iter())
                    .map(|header| match header {
                        AnyHeader::Tendermint(header) => {
                            let da_height = DataChainHeight::new(
                                header.trusted_height.revision_number(),
                                header.trusted_height.revision_height(),
                            )
                            .unwrap();
                            let da_header = Header {
                                signed_header: header.signed_header.clone(),
                                validator_set: header.validator_set.clone(),
                                trusted_height: da_height,
                                trusted_next_validator_set: header.trusted_validator_set.clone(),
                            };
                            Ok(da_header)
                        }
                    })
                    .collect::<Result<Vec<Header>, Chain::Error>>()
                    .unwrap();

                Ok(headers)
            })
            .await
            .unwrap();

        Ok(SovereignUpdateClientPayload {
            datachain_header: headers,
            initial_state_height: da_trusted_height,
            final_state_height: da_target_height,
        })
    }
}
