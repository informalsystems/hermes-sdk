use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelId, PortId};

use crate::driver::traits::types::chain::HasChainType;

pub trait HasChannelAt<Counterparty, const I: usize>: HasChainType
where
    Self::Chain: HasIbcChainTypes<Counterparty>,
{
    fn channel_id(&self) -> &ChannelId<Self::Chain, Counterparty>;

    fn port_id(&self) -> &PortId<Self::Chain, Counterparty>;
}
