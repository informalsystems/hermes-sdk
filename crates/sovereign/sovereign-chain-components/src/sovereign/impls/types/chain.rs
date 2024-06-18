use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use hermes_relayer_components::chain::traits::types::channel::ChannelEndTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::ConnectionEndTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightIncrementerComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::PacketCommitmentTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::PacketReceiptTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_sovereign_rollup_components::impls::types::rollup::ProvideSovereignRollupTypes;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

pub struct ProvideSovereignChainTypes;

delegate_components! {
    ProvideSovereignChainTypes {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            TimestampTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            ChainStatusTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
            PacketCommitmentTypeComponent,
            AcknowledgementTypeComponent,
            PacketReceiptTypeComponent,
            ConnectionEndTypeComponent,
            ChannelEndTypeComponent,
        ]:
            ProvideSovereignRollupTypes,

    }
}

impl<Chain> ProvideChainIdType<Chain> for ProvideSovereignChainTypes
where
    Chain: Async,
{
    // TODO: A rollup chain ID should be a composite of the rollup ID
    // and the DA chain ID. But for now we will handle only the DA chain ID
    // for simplicity.
    type ChainId = ChainId;
}
