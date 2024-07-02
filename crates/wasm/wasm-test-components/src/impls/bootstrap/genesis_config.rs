use core::marker::PhantomData;

use cgp_core::error::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use serde_json::Value;

pub struct ModifyWasmGenesisConfig<InModifier>(pub PhantomData<InModifier>);

impl<Bootstrap, InModifier> CosmosGenesisConfigModifier<Bootstrap>
    for ModifyWasmGenesisConfig<InModifier>
where
    Bootstrap: CanRaiseError<&'static str>,
    InModifier: CosmosGenesisConfigModifier<Bootstrap>,
{
    fn modify_genesis_config(
        bootstrap: &Bootstrap,
        config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        let max_deposit_period = config
            .get_mut("app_state")
            .and_then(|app_state| app_state.get_mut("gov"))
            .and_then(|gov| gov.get_mut("params"))
            .and_then(|deposit_params| deposit_params.as_object_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "Failed to retrieve `deposit_params` in genesis configuration",
                )
            })?;

        max_deposit_period
            .insert(
                "max_deposit_period".to_owned(),
                Value::String("10s".to_owned()),
            )
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "Failed to update `max_deposit_period` in genesis configuration",
                )
            })?;

        let voting_period = config
            .get_mut("app_state")
            .and_then(|app_state| app_state.get_mut("gov"))
            .and_then(|gov| gov.get_mut("params"))
            .and_then(|voting_params| voting_params.as_object_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "Failed to retrieve `voting_params` in genesis configuration",
                )
            })?;

        voting_period
            .insert(
                "voting_period".to_owned(),
                serde_json::Value::String("10s".to_owned()),
            )
            .ok_or_else(|| {
                Bootstrap::raise_error("Failed to update `voting_period` in genesis configuration")
            })?;

        let allowed_clients = config
            .get_mut("app_state")
            .and_then(|app_state| app_state.get_mut("ibc"))
            .and_then(|ibc| ibc.get_mut("client_genesis"))
            .and_then(|client_genesis| client_genesis.get_mut("params"))
            .and_then(|params| params.get_mut("allowed_clients"))
            .and_then(|allowed_clients| allowed_clients.as_array_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "Failed to retrieve `allowed_clients` in genesis configuration",
                )
            })?;

        allowed_clients.push(Value::String("08-wasm".to_string()));

        InModifier::modify_genesis_config(bootstrap, config)?;

        Ok(())
    }
}
