use bevy::{prelude::*, render::view::visibility};
use protocol::protocol::{GameOpponentInfo, GameUserInfo};

use crate::{
    components::{
        anchors::{AnchorType, Anchors},
        hover::{BoundingBox, Hoverable},
    },
    modules::god::God,
    states::startup::startup::UiAssets,
    AppState,
};

pub(crate) struct GameUserInfoPlugin;

impl Plugin for GameUserInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            on_user_info_added.run_if(resource_added::<GameUserRes>),
        )
        .add_systems(
            Update,
            on_user_info_update.run_if(resource_exists_and_changed::<GameUserRes>),
        )
        .add_systems(
            Update,
            on_user_info_removed.run_if(resource_removed::<GameUserRes>()),
        )
        .add_systems(OnEnter(AppState::GameShop), on_shop_enter)
        .add_systems(OnExit(AppState::GameShop), on_shop_exit);
    }
}

#[derive(Resource, Debug)]
pub struct GameUserRes(pub GameUserInfo);

#[derive(Component, Debug)]
pub struct UserMoney;

#[derive(Component, Debug)]
pub struct UserProfile;

fn on_user_info_added(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    game_user_info: Res<GameUserRes>,
    res_anchor: Res<Anchors>,
) {
    info!("Game user info added");

    commands
        .entity(res_anchor.get(AnchorType::TOP).unwrap())
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("$: {}", game_user_info.0.money),
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(0.0, -12.0, 10.0)),
                    ..Default::default()
                },
                UserMoney,
            ));
        });

    commands
        .entity(res_anchor.get(AnchorType::BOTTOM_RIGHT).unwrap())
        .with_children(|parent| {
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(-128.0, 128.0, 10.0))
                        .with_scale(Vec3::splat(3.0)),
                    ..Default::default()
                },
                Hoverable("hover".to_string(), "leave".to_string()),
                BoundingBox(
                    Vec3::new(48.0, 48.0, 0.0),
                    Quat::from_rotation_z(45.0f32.to_radians()),
                ),
                God(GameOpponentInfo {
                    name: game_user_info.0.name.clone(),
                    experience: game_user_info.0.experience,
                    health: game_user_info.0.health,
                    character_id: game_user_info.0.avatar.unwrap_or_default(),
                    is_next_opponent: true,
                }),
                UserProfile,
            ));
        });
}

fn on_user_info_update(
    mut commands: Commands,
    game_user_info: Res<GameUserRes>,
    mut q_money: Query<&mut Text, With<UserMoney>>,
    q_profile: Query<Entity, With<UserProfile>>,
) {
    info!("Game user info updated: {:?}", game_user_info);
    for mut text in q_money.iter_mut() {
        text.sections[0].value = format!("$: {}", game_user_info.0.money);
    }

    if let Ok(profile) = q_profile.get_single() {
        commands.entity(profile).remove::<God>();
        commands.entity(profile).despawn_descendants();
        commands.entity(profile).insert(God(GameOpponentInfo {
            name: game_user_info.0.name.clone(),
            experience: game_user_info.0.experience,
            health: game_user_info.0.health,
            character_id: game_user_info.0.avatar.unwrap_or_default(),
            is_next_opponent: true,
        }));
    }
}

fn on_user_info_removed(
    mut commands: Commands,
    q_info: Query<Entity, Or<(With<UserMoney>, With<UserProfile>)>>,
) {
    info!("Game user info removed");
    for entity in q_info.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn on_shop_enter(mut q_money: Query<&mut Visibility, With<UserMoney>>) {
    let _ = q_money.get_single_mut().map(|mut visibility| {
        *visibility = Visibility::Visible;
    });
}

fn on_shop_exit(mut q_money: Query<&mut Visibility, With<UserMoney>>) {
    let _ = q_money.get_single_mut().map(|mut visibility| {
        *visibility = Visibility::Hidden;
    });
}
