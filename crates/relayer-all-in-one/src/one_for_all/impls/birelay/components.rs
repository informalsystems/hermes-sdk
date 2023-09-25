use cgp_core::traits::sync::Async;
use cgp_core::traits::{DelegateComponent, HasComponents};
use ibc_relayer_components_extra::components::extra::birelay::ExtraBiRelayComponents;

use crate::one_for_all::types::birelay::OfaBiRelayWrapper;
use crate::one_for_all::types::component::OfaComponents;

impl<BiRelay> HasComponents for OfaBiRelayWrapper<BiRelay>
where
    BiRelay: Async,
{
    type Components = ExtraBiRelayComponents<OfaComponents>;
}

impl<BiRelay, Name> DelegateComponent<Name> for OfaBiRelayWrapper<BiRelay>
where
    BiRelay: Async,
{
    type Delegate = ExtraBiRelayComponents<OfaComponents>;
}
