use super::{game_shop::BoardCharacter, startup::loading::SoundEffectAssets};
use crate::{
    cleanup_system,
    components::{
        anchors::{AnchorType, Anchors},
        animation::{
            Animation, AnimationDirection, AnimationEvent, AnimationEventKind, AnimationRepeatType,
            TransformAnimation,
        },
        hover::{BoundingBox, Hoverable},
    },
    modules::{character::Character, god::God},
    AppState, Cleanup,
};
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use protocol::protocol::{BattleActionType, BattleResponse, CharacterInstance};

const STATE: AppState = AppState::GameBattle;

pub(crate) struct GameCombatPlugin;

impl Plugin for GameCombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameCombatState>()
            .add_event::<BattleBoardChangedEvent>()
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, generate_board.run_if(in_state(STATE)))
            .add_systems(
                OnExit(GameCombatState::PlayAnimation),
                animation_finished.run_if(in_state(STATE)),
            )
            .add_systems(
                Update,
                play_next_animation
                    .run_if(in_state(GameCombatState::AnimationFinished))
                    .run_if(in_state(STATE)),
            )
            .add_systems(
                Update,
                (animation_timer, attack_hit)
                    .run_if(in_state(GameCombatState::PlayAnimation))
                    .run_if(in_state(STATE)),
            )
            .add_systems(OnExit(STATE), cleanup_system::<Cleanup>)
            .init_resource::<AnimationTimer>();
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum GameCombatState {
    #[default]
    Setup,
    PlayAnimation,
    AnimationFinished,
    WaitingForShop,
}

#[derive(Resource, Default)]
struct AnimationTimer(Timer);

#[derive(Component, Debug)]
pub struct BoardOwn;

#[derive(Component, Debug)]
pub struct BoardOpponent;

#[derive(Component, Debug)]
pub struct OpponentProfile;

#[derive(Resource, Debug)]
pub struct BattleRes(pub BattleResponse);

#[derive(Debug, Event)]
pub struct BattleBoardChangedEvent(pub [Vec<Option<CharacterInstance>>; 2]);

fn setup(
    mut commands: Commands,
    state: Res<BattleRes>,
    mut ev_board_change: EventWriter<BattleBoardChangedEvent>,
    res_anchor: Res<Anchors>,
) {
    info!("Setting up game combat state");

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-64.0 * 4.0, -128.0, 0.0)),
            ..Default::default()
        },
        BoardOwn,
        Cleanup,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-64.0 * 4.0, 128.0, 0.0)),
            ..Default::default()
        },
        BoardOpponent,
        Cleanup,
    ));

    // Spawn enemy profile
    commands
        .entity(res_anchor.get(AnchorType::TOP_RIGHT).unwrap())
        .with_children(|parent| {
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(-128.0, -128.0, 10.0))
                        .with_scale(Vec3::splat(3.0)),
                    ..Default::default()
                },
                God(state.0.opponent.clone()),
                OpponentProfile,
                Hoverable("hover".to_string(), "leave".to_string()),
                BoundingBox(
                    Vec3::new(48.0, 48.0, 0.0),
                    Quat::from_rotation_z(45.0f32.to_radians()),
                ),
                Cleanup,
            ));
        });

    ev_board_change.send(BattleBoardChangedEvent([
        state.0.start_own.clone(),
        state.0.start_opponent.clone(),
    ]));

    info!("Combat set up");
}

fn play_next_animation(
    mut commands: Commands,
    mut state: ResMut<BattleRes>,
    mut combat_state: ResMut<NextState<GameCombatState>>,
    q_board_character: Query<(
        Entity,
        &BoardCharacter,
        &Children,
        &GlobalTransform,
        &Transform,
    )>,
    q_animation: Query<(Entity, &Animation)>,
    q_target: Query<(&GlobalTransform, &BoardCharacter)>,
    mut ev_board_change: EventWriter<BattleBoardChangedEvent>,
    mut animation_timer: ResMut<AnimationTimer>,
) {
    let current_action = state.0.actions.first().cloned();
    debug!("Playing next animation {:?}", current_action);
    if let Some(current_action) = current_action {
        if let Some((entity, character, children, source_global_transform, source_transform)) =
            q_board_character
                .iter()
                .find(|(_, board_character, _, _, _)| {
                    board_character.character.id == current_action.source
                })
        {
            let duration = match current_action.action {
                protocol::protocol::BattleActionType::Attack => {
                    if let Some(target) = current_action.target {
                        if let Some((transform, _)) = q_target
                            .iter()
                            .find(|(_, board_character)| board_character.character.id == target)
                        {
                            debug!("Playing animation for {:?}", character);
                            let target_transform = transform.compute_transform().translation;
                            let target_transform = target_transform
                                - (source_global_transform.compute_transform().translation
                                    - source_transform.translation);
                            commands.entity(entity).insert(TransformAnimation {
                                source: *source_transform,
                                target: Transform::from_translation(target_transform)
                                    .with_scale(source_transform.scale),
                                speed: 6.0,
                                repeat: AnimationRepeatType::PingPongOnce,
                            });
                        } else {
                            warn!("No target found for {:?}", current_action);
                        }
                    } else {
                        warn!("No target found for {:?}", current_action);
                    }

                    1.5
                }
                protocol::protocol::BattleActionType::Die => {
                    debug!("Playing animation for {:?}", character);
                    if let Some((entity, animation)) = children
                        .iter()
                        .find_map(|entity| q_animation.get(*entity).ok())
                    {
                        commands
                            .entity(entity)
                            .insert(animation.get_transition("die").unwrap());
                    } else {
                        warn!("No animation found for {:?}", character);
                        state.0.actions.remove(0);
                        ev_board_change.send(BattleBoardChangedEvent([
                            current_action.result_own.clone(),
                            current_action.result_opponent.clone(),
                        ]));
                    }
                    0.0
                }
                _ => 0.0,
            };
            animation_timer.0 = Timer::from_seconds(duration, TimerMode::Once);

            debug!("Changing state to PlayAnimation");
            combat_state.set(GameCombatState::PlayAnimation);
        } else {
            warn!("No character found for {:?}", current_action);
        }
    } else {
        combat_state.set(GameCombatState::WaitingForShop);
    }
}

fn animation_timer(
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    mut next_state: ResMut<NextState<GameCombatState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(GameCombatState::AnimationFinished);
    }
}

fn attack_hit(
    mut commands: Commands,
    battle: Res<BattleRes>,
    mut ev_animation: EventReader<AnimationEvent>,
    q_board_character: Query<(&BoardCharacter, Entity)>,
    res_sound_effects: Res<SoundEffectAssets>,
) {
    let action = battle.0.actions.first();
    if let Some(action) = action {
        if let BattleActionType::Attack = action.action {
            for ev in ev_animation.read() {
                if let AnimationEventKind::ChangeDirection(AnimationDirection::Backward) = ev.kind {
                    debug!("Attack hit {:?}", action);
                    let (_, entity) = q_board_character
                        .iter()
                        .find(|(c, _)| c.character.id == action.source)
                        .unwrap();

                    if entity != ev.entity {
                        continue;
                    }

                    commands.spawn(AudioBundle {
                        source: res_sound_effects.attack.clone(),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new(0.5),
                            ..Default::default()
                        },
                    });
                }
            }
        }
    }
}

fn animation_finished(
    mut battle: ResMut<BattleRes>,
    mut ev_board_change: EventWriter<BattleBoardChangedEvent>,
) {
    if battle.0.actions.is_empty() {
        return;
    }
    let action = battle.0.actions.remove(0);
    debug!("Animation finished {:?}", action);
    ev_board_change.send(BattleBoardChangedEvent([
        action.result_own.clone(),
        action.result_opponent.clone(),
    ]));
}

fn generate_board(
    mut commands: Commands,
    mut combat_state: ResMut<NextState<GameCombatState>>,
    mut ev_shop_change: EventReader<BattleBoardChangedEvent>,
    mut q_board_character: Query<(Entity, &mut BoardCharacter)>,
    q_own: Query<Entity, With<BoardOwn>>,
    q_opponent: Query<Entity, With<BoardOpponent>>,
) {
    for ev in ev_shop_change.read() {
        debug!("Generating board");

        // Update existing characters
        let mut updated_characters = Vec::new();
        for (entity, mut bc) in q_board_character.iter_mut() {
            let own_update = ev.0[0].iter().flatten().find(|c| c.id == bc.character.id);
            let opponent_update = ev.0[1].iter().flatten().find(|c| c.id == bc.character.id);

            if (bc.own && own_update.is_none()) || (!bc.own && opponent_update.is_none()) {
                commands.entity(entity).despawn_recursive();
            } else if let Some(update) = own_update {
                updated_characters.push(update.id);
                bc.update(update.clone());
            } else if let Some(update) = opponent_update {
                updated_characters.push(update.id);
                bc.update(update.clone());
            }
        }

        for (player_idx, idx, board, character) in
            ev.0.iter()
                .enumerate()
                .map(|(player_idx, player)| {
                    (
                        player_idx,
                        if player_idx == 0 {
                            q_own.get_single()
                        } else {
                            q_opponent.get_single()
                        },
                        player,
                    )
                })
                .filter(|(_, entity, _)| entity.is_ok())
                .map(|(player_idx, entity, player)| (player_idx, entity.unwrap(), player))
                .flat_map(|(player_idx, entity, player)| {
                    player
                        .iter()
                        .enumerate()
                        .map(move |(idx, character)| (player_idx, idx, entity, character.as_ref()))
                })
                .filter_map(|(player_idx, idx, entity, character)| {
                    if let Some(character) = character {
                        Some((player_idx, idx, entity, character))
                    } else {
                        None
                    }
                })
                .filter(|(_, _, _, character)| !updated_characters.contains(&character.id))
                .map(|(player_idx, idx, entity, character)| (player_idx, idx, entity, character))
        {
            commands.entity(board).with_children(|parent| {
                parent.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(Vec3::new(
                            68.0 * 2.0 * (idx % 4) as f32 + if idx < 4 { 0.0 } else { 68.0 } as f32,
                            if idx < 4 {
                                0.0
                            } else if player_idx == 0 {
                                -136.0
                            } else {
                                136.0
                            },
                            0.0,
                        ))
                        .with_scale(Vec3::splat(2.0)),
                        ..Default::default()
                    },
                    Character(character.clone()),
                    BoardCharacter::new(idx as u8, player_idx == 0, character.clone()),
                    Hoverable("hover".to_string(), "leave".to_string()),
                    BoundingBox(Vec3::new(64.0, 64.0, 0.0), Quat::from_rotation_z(0.0)),
                ));
            });
        }
        combat_state.set(GameCombatState::AnimationFinished);
    }
}
