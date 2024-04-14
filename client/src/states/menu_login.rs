use crate::{
    cleanup_system,
    components::anchors::{AnchorType, Anchors},
    networking::{
        networking_events::NetworkingEvent, networking_ressource::NetworkingRessource,
        polling::PollingStatus,
    },
    prefabs::ui::timer::TimerTextBundle,
    AppState, Cleanup, StateChangeEvent,
};
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};
use protocol::protocol::{Credentials, Protocol, UserData};
use reqwest::{header::HeaderValue, Method};

const STATE: AppState = AppState::MenuLogin;
pub(crate) struct MenuLoginPlugin;

impl Plugin for MenuLoginPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoginCredentials>()
            .add_systems(OnEnter(STATE), logout)
            .add_systems(Update, (ui_login, on_login).run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup_system::<Cleanup>);
    }
}

#[derive(Resource, Default)]
struct LoginCredentials(Credentials);

#[derive(Resource)]
pub struct User(pub UserData);

fn logout(
    mut commands: Commands,
    mut ev_polling_status: EventWriter<PollingStatus>,
    asset_server: Res<AssetServer>,
    res_anchors: Res<Anchors>,
) {
    debug!("Logout start");
    commands.remove_resource::<User>();
    ev_polling_status.send(PollingStatus::Stop);

    commands
        .entity(res_anchors.get(AnchorType::TOP_RIGHT).unwrap())
        .with_children(|parent| {
            parent.spawn((TimerTextBundle::new(
                &asset_server,
                Transform::from_translation(Vec3::new(-30.0, -15.0, 100.0)),
            ),));
        });

    debug!("Logout end")
}

fn ui_login(
    mut contexts: EguiContexts,
    mut network: ResMut<NetworkingRessource>,
    mut credentials: ResMut<LoginCredentials>,
    mut ev_exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Login");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut credentials.0.username);
        });
        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut credentials.0.password).password(true));
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Login").clicked() {
                network.request_data(Method::POST, "users", &credentials.0);
            }
            if ui.button("Register").clicked() {
                network.request_data(Method::PUT, "users", &credentials.0);
            }
        });
        if ui.button("Exit").clicked() {
            ev_exit.send(AppExit);
        }
    });
}

fn on_login(
    mut commands: Commands,
    mut network: ResMut<NetworkingRessource>,
    mut ev_networking: EventReader<NetworkingEvent>,
    mut ev_polling_status: EventWriter<PollingStatus>,
    mut ev_state_change: EventWriter<StateChangeEvent>,
) {
    for ev in ev_networking.read() {
        if let Protocol::LoginResponse(login) = &ev.0 {
            network.headers.insert(
                "x-api-key",
                HeaderValue::from_str(login.key.as_str()).unwrap(),
            );

            commands.insert_resource(User(login.user.clone()));
            debug!("Logged in as {}", login.user.username);

            ev_polling_status.send(PollingStatus::Start);
            if login.user.display_name.is_none() {
                ev_state_change.send(StateChangeEvent(AppState::MenuSetDisplayName));
            } else {
                ev_state_change.send(StateChangeEvent(AppState::MenuMain));
            }
        }
    }
}
