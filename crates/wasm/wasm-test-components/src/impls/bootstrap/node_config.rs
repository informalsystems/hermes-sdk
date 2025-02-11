use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::{
    CometConfigModifier, CometConfigModifierComponent,
};
use toml::Value;

pub struct ModifyWasmNodeConfig<InModifier>(pub PhantomData<InModifier>);

#[cgp_provider(CometConfigModifierComponent)]
impl<Bootstrap, InModifier> CometConfigModifier<Bootstrap> for ModifyWasmNodeConfig<InModifier>
where
    Bootstrap: CanRaiseAsyncError<&'static str>,
    InModifier: CometConfigModifier<Bootstrap>,
{
    fn modify_comet_config(
        bootstrap: &Bootstrap,
        comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        comet_config
            .get_mut("rpc")
            .and_then(|rpc| rpc.as_table_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error("Failed to retrieve `rpc` in node configuration")
            })?
            .insert("max_body_bytes".to_string(), Value::Integer(10001048576));

        comet_config
            .get_mut("mempool")
            .and_then(|mempool| mempool.as_table_mut())
            .ok_or_else(|| {
                Bootstrap::raise_error("Failed to retrieve `mempool` in node configuration")
            })?
            .insert("max_tx_bytes".to_string(), Value::Integer(104857600));

        InModifier::modify_comet_config(bootstrap, comet_config)?;

        Ok(())
    }
}
