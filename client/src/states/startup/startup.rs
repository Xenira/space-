use crate::{components::background::Background, AppState, StateChangeEvent};
use bevy::prelude::*;

const STATE: AppState = AppState::Startup;
pub(crate) struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>()
            .init_resource::<BackgroundAssets>()
            .add_systems(
                OnEnter(STATE),
                (load_ui_assets, load_background_assets).before(setup),
            )
            .add_systems(OnEnter(STATE), setup);
    }
}

#[derive(Resource, Default)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub cursor: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct BackgroundAssets {
    pub(crate) background: Handle<Image>,
}

fn load_ui_assets(asset_server: Res<AssetServer>, mut ui_assets: ResMut<UiAssets>) {
    ui_assets.font = asset_server.load("fonts/monogram-extended.ttf");
    ui_assets.cursor = asset_server.load("textures/ui/cursor.png");
}

fn load_background_assets(
    asset_server: Res<AssetServer>,
    mut background_assets: ResMut<BackgroundAssets>,
) {
    background_assets.background = asset_server.load("textures/background/bg.png");
}

fn setup(
    mut ev_state_change: EventWriter<StateChangeEvent>,
    background_assets: ResMut<BackgroundAssets>,
    mut background_resource: ResMut<Background>,
) {
    ev_state_change.send(StateChangeEvent(AppState::Loading));
    background_resource.0 = background_assets.background.clone();
}
