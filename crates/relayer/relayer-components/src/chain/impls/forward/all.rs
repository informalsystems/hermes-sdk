use cgp_core::prelude::*;

use crate::chain::impls::forward::chain_id::ForwardChainId;
use crate::chain::impls::forward::create_client::ForwardCreateClientOptionsType;
use crate::chain::impls::forward::init_channel::ForwardInitChannelOptionsType;
use crate::chain::impls::forward::init_connection::ForwardInitConnectionOptionsType;
use crate::chain::impls::forward::types::ForwardChainTypes;
use crate::chain::traits::types::chain_id::{ChainIdGetterComponent, ChainIdTypeComponent};
use crate::chain::traits::types::channel::InitChannelOptionsTypeComponent;
use crate::chain::traits::types::connection::InitConnectionOptionsTypeComponent;
use crate::chain::traits::types::create_client::CreateClientOptionsTypeComponent;
use crate::chain::traits::types::event::EventTypeComponent;
use crate::chain::traits::types::height::HeightTypeComponent;
use crate::chain::traits::types::ibc::IbcChainTypesComponent;
use crate::chain::traits::types::message::MessageTypeComponent;
use crate::chain::traits::types::timestamp::TimestampTypeComponent;

pub struct ForwardToInnerChain;

delegate_components! {
    #[mark_component(IsForwardToInnerChainComponent)]
    #[mark_delegate(DelegatesToInnerChain)]
    ForwardToInnerChain {
        [
            HeightTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            ChainIdTypeComponent,
            TimestampTypeComponent,
            IbcChainTypesComponent,
        ]:
            ForwardChainTypes,
        ChainIdGetterComponent: ForwardChainId,
        CreateClientOptionsTypeComponent:
            ForwardCreateClientOptionsType,
        InitConnectionOptionsTypeComponent:
            ForwardInitConnectionOptionsType,
        InitChannelOptionsTypeComponent:
            ForwardInitChannelOptionsType,
    }
}
