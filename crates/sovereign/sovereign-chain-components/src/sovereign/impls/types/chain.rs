use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
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
            TimestampTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            ChainStatusTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
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
