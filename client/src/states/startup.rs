use crate::{components::background::Background, AppState, StateChangeEvent};
use bevy::{prelude::*, utils::HashMap};
use protocol::{characters::get_characters, gods::get_gods, spells::get_spells};

const STATE: AppState = AppState::Startup;
pub(crate) struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>()
            .init_resource::<GodAssets>()
            .init_resource::<CharacterAssets>()
            .init_resource::<BackgroundAssets>()
            .init_resource::<SpellAssets>()
            .add_systems(
                OnEnter(STATE),
                (
                    load_ui_assets,
                    load_gods_assets,
                    load_character_assets,
                    load_background_assets,
                    load_spell_assets,
                )
                    .before(setup),
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
pub struct GodAssets {
    pub(crate) god_portraits: HashMap<i32, Handle<Image>>,
    pub god_frame: Handle<TextureAtlas>,
    pub lvl_orb: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct CharacterAssets {
    pub(crate) character_portraits: HashMap<i32, Handle<Image>>,
    pub character_frame: Handle<Image>,
    pub health_orb: Handle<Image>,
    pub attack_orb: Handle<Image>,
    pub price_orb: Handle<Image>,
    pub upgrded: Handle<TextureAtlas>,
    pub upgradable: Handle<TextureAtlas>,
    pub duplicate: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct SpellAssets {
    pub(crate) spell_portraits: HashMap<i32, Handle<Image>>,
    pub spell_frame: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct BackgroundAssets {
    pub(crate) background: Handle<Image>,
}

fn load_ui_assets(asset_server: Res<AssetServer>, mut ui_assets: ResMut<UiAssets>) {
    ui_assets.font = asset_server.load("fonts/monogram-extended.ttf");
    ui_assets.cursor = asset_server.load("textures/ui/cursor.png");
}

fn load_gods_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut god_assets: ResMut<GodAssets>,
) {
    for god in get_gods().iter() {
        god_assets.god_portraits.insert(
            god.id,
            asset_server.load(format!("generated/gods/{}.png", god.id)),
        );
    }

    let god_frame = asset_server.load("textures/ui/god_frame2.png");
    let god_frame_atlas =
        TextureAtlas::from_grid(god_frame, Vec2::new(64.0, 64.0), 18, 1, None, None);

    god_assets.god_frame = texture_atlases.add(god_frame_atlas);
    god_assets.lvl_orb = asset_server.load("textures/ui/lvl_orb.png");
}

fn load_character_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut character_assets: ResMut<CharacterAssets>,
) {
    for character in get_characters().iter() {
        character_assets.character_portraits.insert(
            character.id,
            asset_server.load(format!("generated/characters/{}.png", character.id)),
        );
    }

    character_assets.character_frame = asset_server.load("textures/ui/character_frame.png");
    character_assets.health_orb = asset_server.load("textures/ui/health_orb.png");
    character_assets.attack_orb = asset_server.load("textures/ui/attack_orb.png");
    character_assets.price_orb = asset_server.load("textures/ui/price_orb.png");

    let upgrded = asset_server.load("textures/ui/Effect_EldenRing_1_421x425.png");
    let upgrded_atlas = TextureAtlas::from_grid(upgrded, Vec2::new(421.0, 425.0), 6, 5, None, None);
    character_assets.upgrded = texture_atlases.add(upgrded_atlas);

    let upgradable = asset_server.load("textures/ui/Effect_Wheel_1_273x273.png");
    let upgradable_atlas =
        TextureAtlas::from_grid(upgradable, Vec2::new(273.0, 273.0), 6, 5, None, None);
    character_assets.upgradable = texture_atlases.add(upgradable_atlas);

    let duplicate = asset_server.load("textures/ui/Effect_ElectricShield_1_265x265.png");
    let duplicate_atlas =
        TextureAtlas::from_grid(duplicate, Vec2::new(265.0, 265.0), 6, 5, None, None);
    character_assets.duplicate = texture_atlases.add(duplicate_atlas);
}

fn load_spell_assets(asset_server: Res<AssetServer>, mut spell_assets: ResMut<SpellAssets>) {
    for spell in get_spells().iter() {
        spell_assets.spell_portraits.insert(
            spell.id,
            asset_server.load(format!("generated/spells/{}.png", spell.id)),
        );
    }

    spell_assets.spell_frame = asset_server.load("textures/ui/character_frame.png");
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
    ev_state_change.send(StateChangeEvent(AppState::MenuLogin));
    background_resource.0 = background_assets.background.clone();
}
