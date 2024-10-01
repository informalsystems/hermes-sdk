use core::marker::PhantomData;

use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::commitment::path::send_packet::CanBuildSendPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::send_packet::CanBuildSendPacketCommitmentValue;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::transaction::IbcTransactionHandler;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction::HasIbcTransactionType;

pub struct CommitSendPacket<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IbcTransactionHandler<Chain, Counterparty>
    for CommitSendPacket<InHandler>
where
    Chain: HasErrorType
        + HasIbcTransactionType<Counterparty>
        + HasPacketType<Counterparty>
        + HasPacketHeader<Counterparty>
        + CanBuildSendPacketCommitmentPath<Counterparty>
        + CanBuildSendPacketCommitmentValue<Counterparty>
        + CanStoreCommitment,
    Counterparty: HasClientIdType<Chain>,
    InHandler: IbcTransactionHandler<Chain, Counterparty>,
{
    async fn handle_ibc_transaction(
        chain: &Chain,
        transaction: &Chain::IbcTransaction,
    ) -> Result<Chain::Packet, Chain::Error> {
        let packet = InHandler::handle_ibc_transaction(chain, transaction).await?;

        let packet_header = Chain::packet_header(&packet);

        let commitment_path = Chain::build_send_packet_commitment_path(packet_header)?;

        let commitment_value = Chain::build_send_packet_commitment_value(&packet)?;

        chain
            .store_commitment(&commitment_path, &commitment_value)
            .await?;

        Ok(packet)
    }
}
