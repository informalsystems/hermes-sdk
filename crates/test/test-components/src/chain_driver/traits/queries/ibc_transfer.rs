use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelId, OutgoingPacket, PortId};

use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::types::wallet::HasWalletType;
use crate::driver::traits::types::chain::HasChainType;

#[derive_component(TokenIbcTransferrerComponent, TokenIbcTransferrer<Chain>)]
#[async_trait]
pub trait CanIbcTransferToken<CounterpartyDriver>:
    HasErrorType + HasChainType + HasWalletType + HasAmountType
where
    Self::Chain:
        HasIbcChainTypes<CounterpartyDriver::Chain> + HasIbcPacketTypes<CounterpartyDriver::Chain>,
    CounterpartyDriver: HasAddressType + HasChainType,
{
    async fn ibc_transfer_token(
        &self,
        channel_id: &ChannelId<Self::Chain, CounterpartyDriver::Chain>,
        port_id: &PortId<Self::Chain, CounterpartyDriver::Chain>,
        sender_wallet: &Self::Wallet,
        recipient_address: &CounterpartyDriver::Address,
        amount: &Self::Amount,
    ) -> Result<OutgoingPacket<Self::Chain, CounterpartyDriver::Chain>, Self::Error>;
}
