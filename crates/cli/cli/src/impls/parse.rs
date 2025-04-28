use core::marker::PhantomData;

use hermes_cli_components::traits::{ArgParser, ArgParserComponent};
use hermes_cosmos::chain_components::types::CosmosInitChannelOptions;
use hermes_cosmos::ibc::core::channel::types::channel::Order;
use hermes_cosmos::ibc::core::channel::types::error::ChannelError;
use hermes_cosmos::ibc::core::channel::types::Version;
use hermes_cosmos::ibc::core::host::types::error::IdentifierError;
use hermes_cosmos::ibc::core::host::types::identifiers::ConnectionId;
use hermes_prelude::*;

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
