use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

pub trait HasChannel<Counterparty, const I: usize>: HasIbcChainTypes<Counterparty> {
    fn channel_id(&self) -> &Self::ChannelId;

    fn port_id(&self) -> &Self::PortId;
}

/// Helper auto trait method for accessing N-th channel/port ID
pub trait NthChannel<Counterparty>: HasIbcChainTypes<Counterparty> {
    fn nth_channel_id<const I: usize>(&self) -> &Self::ChannelId
    where
        Self: HasChannel<Counterparty, I>;

    fn nth_port_id<const I: usize>(&self) -> &Self::PortId
    where
        Self: HasChannel<Counterparty, I>;
}

impl<Chain, Counterparty> NthChannel<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty>,
{
    fn nth_channel_id<const I: usize>(&self) -> &Self::ChannelId
    where
        Self: HasChannel<Counterparty, I>,
    {
        self.channel_id()
    }

    fn nth_port_id<const I: usize>(&self) -> &Self::PortId
    where
        Self: HasChannel<Counterparty, I>,
    {
        self.port_id()
    }
}
