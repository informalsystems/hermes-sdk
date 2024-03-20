use cgp_core::prelude::*;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetter, ProvideLoggerType,
};
use hermes_relayer_components::log::traits::logger::Logger;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use tracing::info;

use crate::contexts::chain::CosmosChain;

pub struct CosmosLogger;

pub struct CosmosLoggerComponents;

pub struct ProvideCosmosLogger;

impl HasComponents for CosmosLogger {
    type Components = CosmosLoggerComponents;
}

impl<Context> ProvideLoggerType<Context> for ProvideCosmosLogger
where
    Context: Async,
{
    type Logger = CosmosLogger;
}

impl<Context> LoggerGetter<Context> for ProvideCosmosLogger
where
    Context: HasLoggerType<Logger = CosmosLogger>,
{
    fn logger(_context: &Context) -> &CosmosLogger {
        &CosmosLogger
    }
}

impl<Context> GlobalLoggerGetter<Context> for ProvideCosmosLogger
where
    Context: HasLoggerType<Logger = CosmosLogger>,
{
    fn global_logger() -> &'static CosmosLogger {
        &CosmosLogger
    }
}

impl<'a, Logging> Logger<Logging, LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    for CosmosLoggerComponents
where
    Logging: Async,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogSendMessagesWithSignerAndNonce<'a, CosmosChain>,
    ) {
        info!(
            chain_id = %details.chain.chain_id,
            nonce = ?details.nonce,
            signer = ?details.signer,
            "{message}",
        );
    }
}
