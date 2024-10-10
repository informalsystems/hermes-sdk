use cgp::prelude::*;
use hermes_ibc_components::traits::types::payload::data::{HasPayloadDataType, PayloadDataOf};

#[derive_component(IncomingTransferAppsComponent, ProvideIncomingTransferApps<Chain>)]
pub trait HasIncomingTransferApps: Async {
    type MintApp: Async;

    type UnescrowApp: Async;
}

#[derive_component(IncomingTransferDataParserComponent, IncomingTransferDataParser<Chain>)]
pub trait CanParseIncomingTransferData<Counterparty, App>:
    HasErrorType + HasIncomingTransferApps
where
    Counterparty: HasPayloadDataType<Self, App>
        + HasPayloadDataType<Self, Self::MintApp>
        + HasPayloadDataType<Self, Self::UnescrowApp>,
{
    fn parse_incoming_transfer_data(
        &self,
        data: &PayloadDataOf<Counterparty, Self, App>,
    ) -> Result<IncomingTransferData<Self, Counterparty>, Self::Error>;
}

pub enum IncomingTransferData<Chain, Counterparty>
where
    Chain: HasIncomingTransferApps,
    Counterparty:
        HasPayloadDataType<Chain, Chain::MintApp> + HasPayloadDataType<Chain, Chain::UnescrowApp>,
{
    Mint(PayloadDataOf<Counterparty, Chain, Chain::MintApp>),
    Unescrow(PayloadDataOf<Counterparty, Chain, Chain::UnescrowApp>),
}
