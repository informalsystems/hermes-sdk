use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

#[derive(HasField)]
pub struct IbcTransferUnescrowPacketData<Counterparty>
where
    Counterparty: HasAmountType + HasAddressType,
{
    pub receiver: Counterparty::Address,
    pub amount: Counterparty::Amount,
}
