/*!
   Constructs for the chain context.

   A chain context corresponds to the context that the relayer uses to
   interact with a single chain. For the purpose of the relayer, the
   chain context acts as a _client_ to the chain. This is in contrast
   with other provider-side chain constructs, which are used for implementing
   a blockchain, and are not covered by this chain context.

   At its core, a chain context should implement
   [`HasChainTypes`](traits::types::chain::HasChainTypes), which declares the abstract
   types that are commonly used inside a chain context.

   The chain context provides functionalities for querying the chain,
   such as through `CanQueryChainStatus`. It also supports sending of messages
   to a chain using `CanSendMessages`.
*/

pub mod impls;
pub mod traits;
pub mod types;
