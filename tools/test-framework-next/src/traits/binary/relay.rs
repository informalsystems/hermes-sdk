use ibc_relayer_components::relay::traits::two_way::HasTwoWayRelay;

use crate::traits::binary::chain::HasTwoChains;

pub trait HasRelay: HasTwoChains {
    type BiRelay: HasTwoWayRelay;
}
