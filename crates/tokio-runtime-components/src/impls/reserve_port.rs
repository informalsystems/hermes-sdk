use cgp_core::prelude::*;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserver;
use rand::Rng;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub struct TokioReserveTcpPort;

#[async_trait]
impl<Runtime> TcpPortReserver<Runtime> for TokioReserveTcpPort
where
    Runtime: HasErrorType,
{
    async fn reserve_tcp_port(_runtime: &Runtime) -> Result<u16, Runtime::Error> {
        loop {
            let port = {
                let mut rng = rand::thread_rng();
                rng.gen_range(1024..=u16::MAX)
            };

            let loopback = Ipv4Addr::new(127, 0, 0, 1);
            let address = SocketAddrV4::new(loopback, port);

            if let Ok(_) = TcpListener::bind(address) {
                // TODO: also check if the port has been previously reserved

                return Ok(port);
            }
        }
    }
}
