use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cli_components::traits::parse::{ArgParser, ArgParserComponent};
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::Version;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::{ConnectionId, PortId};

const DEFAULT_VERSION: &str = "ics20-1";

pub struct ParsePortId;

#[cgp_provider(ArgParserComponent)]
impl<App, Args, Tag> ArgParser<App, Args, Tag> for ParsePortId
where
    App: CanRaiseAsyncError<IdentifierError>,
    Args: HasField<Tag, Value = String>,
{
    type Parsed = PortId;

    fn parse_arg(
        _app: &App,
        args: &Args,
        _tag: PhantomData<Tag>,
    ) -> Result<Self::Parsed, App::Error> {
        if let Ok(port_id) = args.get_field(PhantomData).parse::<PortId>() {
            Ok(port_id)
        } else {
            Ok(PortId::transfer())
        }
    }
}

pub struct ParseInitCosmosChannelOptions;

#[cgp_provider(ArgParserComponent)]
impl<App, Args, Tag> ArgParser<App, Args, Tag> for ParseInitCosmosChannelOptions
where
    App: HasAsyncErrorType,
    Args: HasField<symbol!("target_connection_id"), Value = String>
        + HasField<symbol!("version"), Value = String>
        + HasField<symbol!("ordering"), Value = String>,
{
    type Parsed = CosmosInitChannelOptions;

    fn parse_arg(
        _app: &App,
        args: &Args,
        _tag: PhantomData<Tag>,
    ) -> Result<Self::Parsed, App::Error> {
        let connection_hops = if let Ok(conn_id) = args
            .get_field(PhantomData::<symbol!("target_connection_id")>)
            .parse::<ConnectionId>()
        {
            vec![conn_id]
        } else {
            Default::default()
        };

        let ordering =
            if let Ok(ordering) = args.get_field(PhantomData::<symbol!("ordering")>).parse() {
                ordering
            } else {
                Order::Unordered
            };

        let channel_version = match args.get_field(PhantomData::<symbol!("version")>).parse() {
            Ok(version) => version,
            Err(_) => Version::new(DEFAULT_VERSION.to_string()),
        };

        Ok(CosmosInitChannelOptions {
            connection_hops,
            ordering,
            channel_version,
        })
    }
}
