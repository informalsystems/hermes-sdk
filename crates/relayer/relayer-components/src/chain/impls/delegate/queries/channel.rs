use core::marker::PhantomData;

use cgp_core::{DelegateComponent, HasErrorType};

use crate::chain::traits::{queries::channel::ChannelQuerier, types::{channel::HasChannelEndsType, ibc::HasIbcChainTypes}};

pub struct DelegateQueryChannel<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ChannelQuerier<Chain, Counterparty>
    for DelegateQueryChannel<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasChannelEndsType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ChannelQuerier<Chain, Counterparty>,
{
    async fn query_channel(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId, 
        height: &Chain::Height,
    ) -> Result<Counterparty::ChannelEnd, Chain::Error> {
        Delegate::query_channel(chain, port_id, channel_id, height).await
    }
}