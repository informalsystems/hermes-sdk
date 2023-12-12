use cgp_core::prelude::*;
use ibc_relayer_components::components::default::chain::DefaultChainComponents;

use crate::chain::traits::build::ChainIdFromStringBuilderComponent;
use crate::chain::traits::types::address::AddressTypeComponent;
use crate::chain::traits::types::amount::AmountTypeComponent;
use crate::chain::traits::types::denom::DenomTypeComponent;
use crate::chain::traits::types::wallet::{WalletSignerComponent, WalletTypeComponent};

delegate_components!(
    DefaultChainComponents<BaseComponents>;
    [
        ChainIdFromStringBuilderComponent,
        AmountTypeComponent,
        DenomTypeComponent,
        AddressTypeComponent,
        WalletTypeComponent,
        WalletSignerComponent,
    ]:
        BaseComponents
);
