use cgp_core::prelude::*;

#[derive_component(ArgParserComponent, ArgParser<App>)]
pub trait CanParseArg<Args, Tag>: HasErrorType {
    type Parsed: Async;

    fn parse_arg(&self, args: &Args) -> Result<Self::Parsed, Self::Error>;
}
