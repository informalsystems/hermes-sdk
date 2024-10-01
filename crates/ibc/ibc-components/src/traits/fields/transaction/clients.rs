use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub trait HasIbcTransactionClients<Counterparty>:
    HasIbcTransactionHeaderType<Counterparty> + HasClientIdType<Counterparty>
where
    Counterparty: HasClientIdType<Self>,
{
    fn transaction_src_client_id(header: &Self::IbcTransactionHeader) -> &Self::ClientId;

    fn transaction_dst_client_id(header: &Self::IbcTransactionHeader) -> &Counterparty::ClientId;
}
