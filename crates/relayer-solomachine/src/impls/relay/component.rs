use cgp_core::{Async, HasComponents};
use ibc_relayer_components::components::default::relay::DefaultRelayComponents;

use crate::context::relay::SolomachineRelay;

pub struct SolomachineRelayComponents;

impl<Chain> HasComponents for SolomachineRelay<Chain>
where
    Chain: Async,
{
    type Components = DefaultRelayComponents<SolomachineRelayComponents>;
}
