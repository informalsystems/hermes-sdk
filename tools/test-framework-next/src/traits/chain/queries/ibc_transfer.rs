use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::token::HasTokenType;
use crate::traits::chain::types::wallet::HasWalletType;

#[async_trait]
pub trait CanIbcTransferToken<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasWalletType + HasTokenType + HasErrorType
where
    Counterparty: HasAddressType,
{
    async fn ibc_transfer_token(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sender_wallet: &Self::Wallet,
        recipient_address: &Counterparty::Address,
        token: &Self::Token,
    ) -> Result<(), Self::Error>;
}
