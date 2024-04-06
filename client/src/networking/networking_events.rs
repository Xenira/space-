use bevy::prelude::Event;
use protocol::protocol::Protocol;

#[derive(Debug, Event)]
pub(crate) struct NetworkingEvent(pub(crate) Protocol);
