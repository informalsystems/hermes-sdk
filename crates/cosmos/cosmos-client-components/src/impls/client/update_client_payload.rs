use core::iter;

use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::core::ics02_client::header::AnyHeader;
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::client::CosmosUpdateClientPayload;
use crate::types::tendermint::{TendermintClientState, TendermintHeader};

pub struct BuildUpdateClientPayloadWithChainHandle;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildUpdateClientPayloadWithChainHandle
where
    Chain: HasHeightType<Height = Height>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = CosmosUpdateClientPayload>
        + HasClientStateType<Counterparty, ClientState = TendermintClientState>
        + HasBlockingChainHandle,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &Height,
        target_height: &Height,
        client_state: TendermintClientState,
    ) -> Result<CosmosUpdateClientPayload, Chain::Error> {
        let trusted_height = *trusted_height;
        let target_height = *target_height;

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (header, support) = chain_handle
                    .build_header(
                        trusted_height,
                        target_height,
                        AnyClientState::Tendermint(client_state),
                    )
                    .map_err(Chain::raise_error)?;

                let headers = iter::once(header)
                    .chain(support.into_iter())
                    .map(|header| match header {
                        AnyHeader::Tendermint(header) => Ok(header),
                    })
                    .collect::<Result<Vec<TendermintHeader>, Chain::Error>>()?;

                Ok(CosmosUpdateClientPayload { headers })
            })
            .await
    }
}
