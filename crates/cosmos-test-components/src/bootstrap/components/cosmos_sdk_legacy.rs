use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::bootstrap::components::cosmos_sdk::CosmosSdkBootstrapComponents;
use crate::bootstrap::components::cosmos_sdk::IsCommonCosmosSdkBootstrapComponent;
use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;

pub struct LegacyCosmosSdkBootstrapComponents<BaseComponents>(pub PhantomData<BaseComponents>);

pub trait IsLegacyCosmosSdkBootstrapComponent<Component> {}

impl<T, Component> IsLegacyCosmosSdkBootstrapComponent<Component> for T where
    (): IsCommonCosmosSdkBootstrapComponent<Component>
{
}

impl<Component, BaseComponents> DelegateComponent<Component>
    for LegacyCosmosSdkBootstrapComponents<BaseComponents>
where
    LegacyCosmosSdkBootstrapComponents<BaseComponents>:
        IsCommonCosmosSdkBootstrapComponent<Component> + Async,
{
    type Delegate = CosmosSdkBootstrapComponents<BaseComponents>;
}

// Components that differ from `CosmosSdkBootstrapComponents`
delegate_components!(
    LegacyCosmosSdkBootstrapComponents<BaseComponents>
        @markers[ IsLegacyCosmosSdkBootstrapComponent ]
    ;

    GenesisAccountAdderComponent: LegacyAddCosmosGenesisAccount,
    GenesisValidatorAdderComponent: LegacyAddCosmosGenesisValidator,
    GenesisTransactionsCollectorComponent: LegacyCollectCosmosGentxs,
);
