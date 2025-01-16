use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
  provider: ArgParser,
  context: App,
}]
pub trait CanParseArg<Args, Tag>: HasAsyncErrorType {
    type Parsed: Async;

    fn parse_arg(&self, args: &Args, tag: PhantomData<Tag>) -> Result<Self::Parsed, Self::Error>;
}

impl<App, Args, Tag, Components, Delegate> ArgParser<App, Args, Tag> for UseDelegate<Components>
where
    App: HasAsyncErrorType,
    Components: DelegateComponent<(Args, Tag), Delegate = Delegate>,
    Delegate: ArgParser<App, Args, Tag>,
{
    type Parsed = Delegate::Parsed;

    fn parse_arg(
        app: &App,
        args: &Args,
        tag: PhantomData<Tag>,
    ) -> Result<Self::Parsed, App::Error> {
        Delegate::parse_arg(app, args, tag)
    }
}
