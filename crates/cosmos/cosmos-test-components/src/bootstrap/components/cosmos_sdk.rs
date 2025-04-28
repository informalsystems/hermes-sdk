#[cgp::re_export_imports]
mod preset {
    use hermes_core::test_components::bootstrap::traits::ChainBootstrapperComponent;
    use hermes_prelude::*;

    use crate::bootstrap::impls::{
        AddCosmosGenesisAccount, AddCosmosGenesisValidator, AddCosmosWalletToGenesis,
        BootstrapCosmosChain, CollectCosmosGentxs, CreateChainHomeDirFromTestDir,
        GenerateRandomChainId, GetCosmosGenesisDenoms, GetStdOut, InitCosmosChainData,
        InitCosmosTestWallet, ProvideCosmosChainNodeConfigType, ProvideCosmosGenesisConfigType,
        ProvideCosmosHdPath, ProvideCosmosWalletConfigType, StartCosmosChain,
        UpdateCosmosChainNodeConfig, UpdateCosmosGenesisConfig,
    };
    use crate::bootstrap::traits::{
        ChainDataInitializerComponent, ChainFullNodeStarterComponent,
        ChainGenesisConfigInitializerComponent, ChainGenesisConfigTypeComponent,
        ChainHomeDirInitializerComponent, ChainIdGeneratorComponent,
        ChainNodeConfigInitializerComponent, ChainNodeConfigTypeComponent,
        GenesisAccountAdderComponent, GenesisDenomGetterComponent,
        GenesisTransactionsCollectorComponent, GenesisValidatorAdderComponent,
        GenesisWalletAdderComponent, WalletConfigFieldsComponent, WalletConfigTypeComponent,
        WalletHdPathComponent, WalletInitializerComponent,
    };

    cgp_preset! {
        CosmosSdkBootstrapComponents {
            GenesisAccountAdderComponent: AddCosmosGenesisAccount,
            GenesisValidatorAdderComponent: AddCosmosGenesisValidator,
            GenesisTransactionsCollectorComponent: CollectCosmosGentxs,
            WalletInitializerComponent: InitCosmosTestWallet<GetStdOut>,

            ChainNodeConfigTypeComponent: ProvideCosmosChainNodeConfigType,
            ChainGenesisConfigTypeComponent: ProvideCosmosGenesisConfigType,
            [
                WalletConfigTypeComponent,
                WalletConfigFieldsComponent,
            ]: ProvideCosmosWalletConfigType,
            ChainIdGeneratorComponent: GenerateRandomChainId,
            ChainHomeDirInitializerComponent: CreateChainHomeDirFromTestDir,
            ChainDataInitializerComponent: InitCosmosChainData,
            WalletHdPathComponent: ProvideCosmosHdPath,
            GenesisDenomGetterComponent: GetCosmosGenesisDenoms,
            ChainNodeConfigInitializerComponent: UpdateCosmosChainNodeConfig,
            ChainGenesisConfigInitializerComponent: UpdateCosmosGenesisConfig,
            GenesisWalletAdderComponent: AddCosmosWalletToGenesis,
            ChainFullNodeStarterComponent: StartCosmosChain,
            ChainBootstrapperComponent: BootstrapCosmosChain,
        }
    }
}
