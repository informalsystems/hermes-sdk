use crate::components::default::closures::relay::auto_relayer::UseDefaultAutoRelayer;
use crate::components::default::closures::relay::event_relayer::UseDefaultEventRelayer;
use crate::components::default::closures::relay::packet_relayer::UseDefaultPacketRelayer;

pub trait CanUseDefaultRelayPreset:
    UseDefaultPacketRelayer + UseDefaultEventRelayer + UseDefaultAutoRelayer
{
}
