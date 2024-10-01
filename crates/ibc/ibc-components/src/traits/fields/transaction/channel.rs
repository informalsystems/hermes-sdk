use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub trait HasIbcTransactionChannels<Counterparty>:
    HasIbcTransactionHeaderType<Counterparty> + HasChannelIdType<Counterparty>
where
    Counterparty: HasChannelIdType<Self>,
{
    fn transaction_src_channel_id(header: &Self::IbcTransactionHeader) -> &Self::ChannelId;

    fn transaction_dst_channel_id(header: &Self::IbcTransactionHeader) -> &Counterparty::ChannelId;
}
