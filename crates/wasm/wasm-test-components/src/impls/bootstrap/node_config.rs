use core::marker::PhantomData;

use cgp_core::error::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use toml::Value;

pub struct ModifyWasmNodeConfig<InModifier>(pub PhantomData<InModifier>);

impl<Bootstrap, InModifier> CometConfigModifier<Bootstrap> for ModifyWasmNodeConfig<InModifier>
where
    Bootstrap: CanRaiseError<&'static str>,
    InModifier: CometConfigModifier<Bootstrap>,
{
    fn modify_comet_config(
        bootstrap: &Bootstrap,
        comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        comet_config
            .get_mut("rpc")
            .and_then(|rpc| rpc.as_table_mut())
            .ok_or_else(|| Bootstrap::raise_error("Failed to retrieve `rpc` in app configuration"))?
            .insert("max_body_bytes".to_string(), Value::Integer(10001048576));

        InModifier::modify_comet_config(bootstrap, comet_config)?;

        Ok(())
    }
}
