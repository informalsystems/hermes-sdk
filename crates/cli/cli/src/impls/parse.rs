use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cli_components::traits::parse::{ArgParser, ArgParserComponent};
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::error::ChannelError;
use ibc::core::channel::types::Version;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ConnectionId;

#[cgp_auto_getter]
pub trait HasCosmosCreateChannelFields {
    fn target_connection_id(&self) -> &String;

    fn version(&self) -> &String;

    fn ordering(&self) -> &String;
}

#[cgp_new_provider(ArgParserComponent)]
impl<App, Args, Tag> ArgParser<App, Args, Tag> for ParseInitCosmosChannelOptions
where
    App: CanRaiseAsyncError<IdentifierError> + CanRaiseAsyncError<ChannelError>,
    Args: HasCosmosCreateChannelFields,
{
    type Parsed = CosmosInitChannelOptions;

    fn parse_arg(
        _app: &App,
        args: &Args,
        _tag: PhantomData<Tag>,
    ) -> Result<Self::Parsed, App::Error> {
        let connection_id: ConnectionId = args
            .target_connection_id()
            .parse()
            .map_err(App::raise_error)?;

        let ordering: Order = args.ordering().parse().map_err(App::raise_error)?;

        let channel_version: Version = args.version().clone().into();

        Ok(CosmosInitChannelOptions {
            connection_hops: vec![connection_id],
            ordering,
            channel_version,
        })
    }
}
