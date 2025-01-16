use core::marker::PhantomData;

use cgp::core::error::CanRaiseAsyncError;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use serde_json::Value;

pub struct ModifyWasmGenesisConfig<InModifier>(pub PhantomData<InModifier>);

impl<Bootstrap, InModifier> CosmosGenesisConfigModifier<Bootstrap>
    for ModifyWasmGenesisConfig<InModifier>
where
    Bootstrap: CanRaiseAsyncError<&'static str>,
    InModifier: CosmosGenesisConfigModifier<Bootstrap>,
{
    fn modify_genesis_config(
        bootstrap: &Bootstrap,
        config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        let gov_params = config
            .get_mut("app_state")
            .and_then(|app_state| app_state.get_mut("gov"))
            .and_then(|gov| gov.get_mut("params"))
            .and_then(|gov_params| gov_params.as_object_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error("Failed to retrieve `gov.params` in genesis configuration")
            })?;

        gov_params.insert(
            "max_deposit_period".to_owned(),
            Value::String("6s".to_owned()),
        );

        if gov_params.contains_key("expedited_voting_period") {
            gov_params.insert(
                "expedited_voting_period".to_owned(),
                Value::String("5s".to_owned()),
            );
        }

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

        voting_period.insert(
            "voting_period".to_owned(),
            serde_json::Value::String("10s".to_owned()),
        );

        let client_genesis_params = config
            .get_mut("app_state")
            .and_then(|app_state| app_state.get_mut("ibc"))
            .and_then(|ibc| ibc.get_mut("client_genesis"))
            .and_then(|client_genesis| client_genesis.get_mut("params"))
            .and_then(|client_genesis_params| client_genesis_params.as_object_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "Failed to retrieve `client_genesis.params` in genesis configuration",
                )
            })?;

        client_genesis_params.insert(
            "allowed_clients".to_owned(),
            Value::Array(vec![Value::String("08-wasm".to_owned())]),
        );

        InModifier::modify_genesis_config(bootstrap, config)?;

        Ok(())
    }
}
