use alloc::borrow::ToOwned;
use alloc::string::String;

use hermes_ibc_components::traits::queries::client_id::{
    ClientIdFromChannelIdQuerier, ClientIdFromChannelIdQuerierComponent,
};
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::channel_id::MockChannelId;
use crate::types::client_id::MockClientId;
use crate::types::tagged::Tagged;

#[cgp_provider(ClientIdFromChannelIdQuerierComponent)]
impl<Chain: Async, Counterparty: Async>
    ClientIdFromChannelIdQuerier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn query_client_id_from_channel_id(
        chain: &MockChain<Chain, Counterparty>,
        channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
    ) -> Result<Tagged<Chain, Counterparty, MockClientId>, String> {
        let state = chain.pending_state.mock_chain_state();

        let client_id = state
            .channel_clients
            .get(channel_id)
            .ok_or_else(|| "channel not found".to_owned())?;

        Ok(client_id.clone())
    }
}
