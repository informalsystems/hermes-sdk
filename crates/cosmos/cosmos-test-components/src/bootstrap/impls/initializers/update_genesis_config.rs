use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_runtime_components::traits::{
    CanReadFileAsString, CanWriteStringToFile, HasFilePathType, HasRuntime,
};
use serde_json::{Error as JsonError, Value};

use crate::bootstrap::traits::{
    CanModifyCosmosGenesisConfig, ChainGenesisConfigInitializer,
    ChainGenesisConfigInitializerComponent, DenomForStaking, DenomForTransfer,
    HasChainGenesisConfigType, HasDenomPrefix, HasDynamicGas,
};
use crate::bootstrap::types::CosmosGenesisConfig;
use crate::chain::types::Denom;

/// Parse the generated genesis JSON file, and allow the bootstrap context to modify the genesis config
pub struct UpdateCosmosGenesisConfig;

#[cgp_provider(ChainGenesisConfigInitializerComponent)]
impl<Bootstrap, Runtime> ChainGenesisConfigInitializer<Bootstrap> for UpdateCosmosGenesisConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainGenesisConfigType
        + CanModifyCosmosGenesisConfig
        + HasDenomPrefix<DenomForStaking>
        + HasDenomPrefix<DenomForTransfer>
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<JsonError>
        + HasDynamicGas
        + CanRaiseAsyncError<&'static str>,
    Runtime: HasFilePathType + CanReadFileAsString + CanWriteStringToFile,
    Bootstrap::ChainGenesisConfig: From<CosmosGenesisConfig>,
{
    async fn init_genesis_config(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
    ) -> Result<Bootstrap::ChainGenesisConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let genesis_file_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("config/genesis.json"),
        );

        let config_string = runtime
            .read_file_as_string(&genesis_file_path)
            .await
            .map_err(Bootstrap::raise_error)?;

        let mut config_json: Value =
            serde_json::from_str(&config_string).map_err(Bootstrap::raise_error)?;

        bootstrap.modify_genesis_config(&mut config_json)?;

        // If dynamic gas pricing is not enabled in the relayer, disable it on the chain
        if bootstrap.dynamic_gas().is_none() {
            disable_fee_market(&mut config_json).map_err(Bootstrap::raise_error)?;
        }

        let modified_config_string =
            serde_json::to_string_pretty(&config_json).map_err(Bootstrap::raise_error)?;

        runtime
            .write_string_to_file(&genesis_file_path, &modified_config_string)
            .await
            .map_err(Bootstrap::raise_error)?;

        // TODO: generate random denom
        let staking_denom = Denom::Base(
            bootstrap
                .denom_prefix(PhantomData::<DenomForStaking>)
                .into(),
        );
        let transfer_denom = Denom::Base(
            bootstrap
                .denom_prefix(PhantomData::<DenomForTransfer>)
                .into(),
        );

        let genesis_config = CosmosGenesisConfig {
            config_json,
            staking_denom,
            transfer_denom,
        };

        Ok(genesis_config.into())
    }
}

// TODO: always disable fee market until we implemented dynamic fees in Hermes SDK
pub fn disable_fee_market(config: &mut Value) -> Result<(), &'static str> {
    let m_fee_market = config
        .get_mut("app_state")
        .ok_or("expect app_state to present")?
        .get_mut("feemarket");

    if let Some(feemarket) = m_fee_market {
        feemarket
            .get_mut("params")
            .ok_or("expect feemarket.params to present")?
            .as_object_mut()
            .ok_or("expect feemarket.params to present as dictionary")?
            .insert("enabled".into(), false.into());
    }

    Ok(())
}
