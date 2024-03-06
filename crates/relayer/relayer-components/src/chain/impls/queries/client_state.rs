use alloc::vec::Vec;
use cgp_core::CanRaiseError;

use crate::chain::traits::queries::client_state::{
    AllClientStatesQuerier, CanQueryAllClientStatesBytes, CanQueryClientStateBytes,
    ClientStateQuerier,
};
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::encode::traits::decoder::CanDecode;
use crate::encode::traits::encoded::HasEncodedType;
use crate::encode::traits::has_encoding::HasEncoding;

pub struct QueryAndDecodeClientState;

impl<Chain, Counterparty, Encoding> ClientStateQuerier<Chain, Counterparty>
    for QueryAndDecodeClientState
where
    Chain: CanQueryClientStateBytes<Counterparty> + CanRaiseError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasEncoding<Encoding = Encoding>,
    Encoding: Default + HasEncodedType<Encoded = Vec<u8>> + CanDecode<Counterparty::ClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let client_state_bytes = chain.query_client_state_bytes(client_id, height).await?;

        let client_state = Counterparty::Encoding::default()
            .decode(&client_state_bytes)
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}

impl<Chain, Counterparty, Encoding> AllClientStatesQuerier<Chain, Counterparty>
    for QueryAndDecodeClientState
where
    Chain: CanQueryAllClientStatesBytes<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasEncoding<Encoding = Encoding>,
    Encoding: Default + HasEncodedType<Encoded = Vec<u8>> + CanDecode<Counterparty::ClientState>,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        let encoding = Counterparty::Encoding::default();
        let raw_entries = chain.query_all_client_states_bytes(height).await?;

        let entries = raw_entries
            .into_iter()
            .filter_map(|(client_id, client_state_bytes)| {
                let client_state = encoding.decode(&client_state_bytes).ok()?;

                Some((client_id, client_state))
            })
            .collect();

        Ok(entries)
    }
}
