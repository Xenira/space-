use async_channel::Receiver;
use bevy::prelude::*;
use surf::http::Method;

use crate::networking::util::get_task;

use super::{networking_events::NetworkingEvent, networking_ressource::NetworkingRessource};

#[derive(Component, Debug)]
pub struct PollingReceiver(Receiver<NetworkingEvent>);

pub(crate) enum PollingStatus {
    Start,
    Stop,
}

pub(crate) fn on_polling_status_change(
    mut commands: Commands,
    mut ev_polling_status: EventReader<PollingStatus>,
    mut res: ResMut<NetworkingRessource>,
    query_poller: Query<Entity, With<PollingReceiver>>,
) {
    for ev in ev_polling_status.iter() {
        match ev {
            PollingStatus::Start => {
                commands.spawn_empty().insert(PollingReceiver(get_task(
                    &res.polling_client,
                    res.get_request(Method::Get, "poll").build(),
                )));
            }
            PollingStatus::Stop => {
                query_poller
                    .iter()
                    .for_each(|p| commands.entity(p).despawn_recursive());
            }
        };
    }
}

pub(crate) fn polling_poller(
    mut commands: Commands,
    mut ev: EventWriter<NetworkingEvent>,
    transform_tasks: Query<(Entity, &PollingReceiver)>,
    res: Res<NetworkingRessource>,
) {
    for (entity, receiver) in transform_tasks.iter() {
        if let Ok(event) = receiver.0.try_recv() {
            debug!("Sending networking event {:?}", event);
            commands.entity(entity).insert(PollingReceiver(get_task(
                &res.polling_client,
                res.get_request(Method::Get, "poll").build(),
            )));
            ev.send(event);
        } else if receiver.0.is_closed() {
            warn!("Removing entity {:?} with closed receiver. This could indicate networking requests failing.", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}
