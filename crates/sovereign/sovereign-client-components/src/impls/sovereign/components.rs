use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::impls::sovereign::client::create_client_message::BuildCreateCosmosClientMessageOnSovereign;
use crate::impls::sovereign::client::create_client_payload::BuildSovereignCreateClientPayload;
use crate::impls::sovereign::client::update_client_message::BuildUpdateCosmosClientMessageOnSovereign;
use crate::impls::sovereign::client::update_client_payload::BuildSovereignUpdateClientPayload;
use crate::impls::sovereign::types::chain::ProvideSovereignChainTypes;

pub struct SovereignClientComponents;

delegate_components! {
    #[mark_component(IsSovereignClientComponent)]
    SovereignClientComponents {
        [
            HeightTypeComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
        ]:
            ProvideSovereignChainTypes,
        CreateClientPayloadBuilderComponent: BuildSovereignCreateClientPayload,
        CreateClientMessageBuilderComponent: BuildCreateCosmosClientMessageOnSovereign,
        UpdateClientPayloadBuilderComponent: BuildSovereignUpdateClientPayload,
        UpdateClientMessageBuilderComponent: BuildUpdateCosmosClientMessageOnSovereign,
    }
}
