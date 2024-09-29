use core::marker::PhantomData;

use cgp::prelude::CanRaiseError;
use hermes_relayer_components::error::impls::error::MaxRetryExceededError;
use hermes_relayer_components::error::traits::retry::{HasMaxErrorRetry, HasRetryableError};
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, PacketOf};
use hermes_relayer_components::relay::traits::packet_relayer::PacketRelayer;

pub struct RetryRelayer<InRelay> {
    pub phantom: PhantomData<InRelay>,
}

impl<Relay, InRelayer> PacketRelayer<Relay> for RetryRelayer<InRelayer>
where
    Relay: HasRelayChains
        + HasRetryableError
        + HasMaxErrorRetry
        + for<'a> CanRaiseError<MaxRetryExceededError<'a, Relay>>,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        let max_retry = relay.max_retry();
        let mut retries_made: usize = 0;

        loop {
            let res = InRelayer::relay_packet(relay, packet).await;

            match res {
                Ok(()) => {
                    return Ok(());
                }
                Err(e) => {
                    if Relay::is_retryable_error(&e) {
                        retries_made += 1;

                        if retries_made > max_retry {
                            return Err(Relay::raise_error(MaxRetryExceededError {
                                context: relay,
                                error: e,
                                max_retry,
                            }));
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }
}
