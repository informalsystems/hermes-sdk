use cgp_core::prelude::*;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::impls::types::receipt::ProvideBoolPacketReceipt;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use hermes_relayer_components::chain::traits::types::channel::ChannelEndTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::ConnectionEndTypeComponent;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{
    HasHeightType, HeightFieldGetter, HeightIncrementer, ProvideHeightType,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::PacketCommitmentTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::PacketReceiptTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
use hermes_relayer_components::chain::traits::types::timestamp::{
    HasTimestampType, TimestampTypeComponent,
};
use ibc_relayer_types::timestamp::Timestamp;

use crate::types::commitment_proof::ProvideSovereignCommitmentProof;
use crate::types::event::SovereignEvent;
use crate::types::height::RollupHeight;
use crate::types::message::SovereignMessage;
use crate::types::rollup_id::RollupId;
use crate::types::status::SovereignRollupStatus;

pub struct ProvideSovereignRollupTypes;

impl<Chain> ProvideHeightType<Chain> for ProvideSovereignRollupTypes
where
    Chain: Async,
{
    type Height = RollupHeight;
}

impl<Chain> HeightFieldGetter<Chain> for ProvideSovereignRollupTypes
where
    Chain: HasHeightType<Height = RollupHeight>,
{
    fn revision_number(_height: &RollupHeight) -> u64 {
        0
    }

    fn revision_height(height: &RollupHeight) -> u64 {
        height.slot_number
    }
}

impl<Chain> HeightIncrementer<Chain> for ProvideSovereignRollupTypes
where
    Chain: HasHeightType<Height = RollupHeight> + HasErrorType,
{
    fn increment_height(height: &RollupHeight) -> Result<RollupHeight, Chain::Error> {
        // FIXME: do not increment height for now, as proof height for Sovereign is not incremented
        Ok(height.clone())
    }
}

impl<Chain> ProvideChainIdType<Chain> for ProvideSovereignRollupTypes
where
    Chain: Async,
{
    type ChainId = RollupId;
}

impl<Chain> ProvideMessageType<Chain> for ProvideSovereignRollupTypes
where
    Chain: Async,
{
    type Message = SovereignMessage;
}

impl<Chain> ProvideEventType<Chain> for ProvideSovereignRollupTypes
where
    Chain: Async,
{
    type Event = SovereignEvent;
}

impl<Chain> ProvideChainStatusType<Chain> for ProvideSovereignRollupTypes
where
    Chain: HasHeightType<Height = RollupHeight> + HasTimestampType<Timestamp = Timestamp>,
{
    type ChainStatus = SovereignRollupStatus;

    fn chain_status_height(status: &SovereignRollupStatus) -> &RollupHeight {
        &status.height
    }

    fn chain_status_timestamp(status: &Self::ChainStatus) -> &Timestamp {
        &status.timestamp
    }
}

delegate_components! {
    ProvideSovereignRollupTypes {
        [
            TimestampTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            CommitmentPrefixTypeComponent,
            PacketCommitmentTypeComponent,
            AcknowledgementTypeComponent,
            ConnectionEndTypeComponent,
            ChannelEndTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            CommitmentProofTypeComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
        ]:
            ProvideSovereignCommitmentProof,
        PacketReceiptTypeComponent:
            ProvideBoolPacketReceipt,
    }
}
