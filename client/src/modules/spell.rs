use bevy::prelude::*;
use protocol::protocol_types;

use crate::{
    components::{
        dragndrop::{DragEvent, Dragged},
        hover::HoverEvent,
        tooltip::SetTooltipEvent,
    },
    states::startup::{SpellAssets, UiAssets},
    util::text::break_text,
    Cleanup,
};

const PORTRAIT_HEIGHT: f32 = 192.0;
const TOOLTIP_SCALE: f32 = 1.5;
const TOOLTIP_NAME_HEIGHT: f32 = 28.0 * TOOLTIP_SCALE;
const TOOLTIP_NAME_WIDTH: f32 = 150.0 * TOOLTIP_SCALE;
const TOOLTIP_DESCRIPTION_WIDTH: f32 = 250.0 * TOOLTIP_SCALE;
const TOOLTIP_DESCRIPTION_HEIGHT: f32 = 200.0 * TOOLTIP_SCALE;
const TOOLTIP_COLOR: Color = Color::rgba(0.75, 0.75, 0.25, 0.75);

pub(crate) struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_spawn, on_character_hover, on_character_drag));
    }
}

#[derive(Component, Debug)]
pub struct Spell(pub protocol_types::spell::Spell);

#[derive(Component, Debug)]
pub struct Health;

#[derive(Component, Debug)]
pub struct Attack;

fn on_spawn(
    mut commands: Commands,
    spell_assets: Res<SpellAssets>,
    ui_assets: Res<UiAssets>,
    q_added: Query<(&Spell, Entity), Added<Spell>>,
) {
    for (character, entity) in q_added.iter() {
        commands
            .entity(entity)
            .insert(Cleanup)
            .with_children(|parent| {
                spawn_spell_portrait(parent, &character.0, &spell_assets, &ui_assets, false);
            });
    }
}

fn spawn_spell_portrait(
    parent: &mut ChildBuilder,
    spell: &protocol_types::spell::Spell,
    spell_assets: &SpellAssets,
    ui_assets: &UiAssets,
    full_info: bool,
) {
    parent
        .spawn(SpriteBundle {
            texture: spell_assets.spell_frame.clone(),
            transform: Transform::from_scale(Vec3::splat(0.1))
                .with_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: spell_assets.spell_portraits.get(&spell.id).unwrap().clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0))
                        .with_scale(Vec3::splat(1.0)),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(SpatialBundle {
                            transform: Transform::from_scale(Vec3::splat(6.0))
                                .with_translation(Vec3::new(0.0, 0.0, 5.0)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            if full_info {
                                parent
                                    .spawn(SpatialBundle {
                                        transform: Transform::from_scale(Vec3::splat(0.2)),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        // Name
                                        parent
                                            .spawn(SpriteBundle {
                                                sprite: Sprite {
                                                    color: TOOLTIP_COLOR,
                                                    custom_size: Some(Vec2::new(
                                                        TOOLTIP_NAME_WIDTH,
                                                        TOOLTIP_NAME_HEIGHT,
                                                    )),
                                                    ..default()
                                                },
                                                transform: Transform::from_translation(Vec3::new(
                                                    0.,
                                                    -PORTRAIT_HEIGHT,
                                                    5.,
                                                )),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(Text2dBundle {
                                                    text: Text::from_section(
                                                        spell.name.clone(),
                                                        TextStyle {
                                                            font: ui_assets.font.clone(),
                                                            font_size: 36.0 * TOOLTIP_SCALE,
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    transform: Transform::from_translation(
                                                        Vec3::new(0.0, 0.0, 1.0),
                                                    ),
                                                    ..Default::default()
                                                });
                                            });

                                        // Description
                                        parent
                                            .spawn(SpriteBundle {
                                                sprite: Sprite {
                                                    color: TOOLTIP_COLOR,
                                                    custom_size: Some(Vec2::new(
                                                        TOOLTIP_DESCRIPTION_WIDTH,
                                                        TOOLTIP_DESCRIPTION_HEIGHT,
                                                    )),
                                                    ..default()
                                                },
                                                transform: Transform::from_translation(Vec3::new(
                                                    0.,
                                                    -PORTRAIT_HEIGHT
                                                        - TOOLTIP_NAME_HEIGHT
                                                        - TOOLTIP_DESCRIPTION_HEIGHT / 2.0,
                                                    5.,
                                                )),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(Text2dBundle {
                                                    text: Text::from_section(
                                                        break_text(
                                                            spell.description.clone(),
                                                            TOOLTIP_DESCRIPTION_WIDTH,
                                                            24.0 * TOOLTIP_SCALE,
                                                            true,
                                                        ),
                                                        TextStyle {
                                                            font: ui_assets.font.clone(),
                                                            font_size: 24.0 * TOOLTIP_SCALE,
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    transform: Transform::from_translation(
                                                        Vec3::new(0.0, 0.0, 1.0),
                                                    ),
                                                    ..Default::default()
                                                });
                                            });
                                    });
                            }
                        });
                });
        });
}

fn on_character_hover(
    mut commands: Commands,
    mut ev_hover: EventReader<HoverEvent>,
    mut ev_tooltip: EventWriter<SetTooltipEvent>,
    q_spell: Query<&Spell, Without<Dragged>>,
    spell_assets: Res<SpellAssets>,
    ui_assets: Res<UiAssets>,
) {
    for HoverEvent(entity, is_hovered) in ev_hover.iter() {
        if let Ok(spell) = q_spell.get(*entity) {
            if *is_hovered {
                let tooltip = commands
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_scale(Vec3::splat(6.0))
                                .with_translation(Vec3::new(0.0, 150.0, 0.0)),
                            ..Default::default()
                        },
                        Cleanup,
                    ))
                    .with_children(|parent| {
                        spawn_spell_portrait(parent, &spell.0, &spell_assets, &ui_assets, true);
                    })
                    .id();
                ev_tooltip.send(SetTooltipEvent(*entity, Some(tooltip)));
            } else {
                ev_tooltip.send(SetTooltipEvent(*entity, None));
            }
        }
    }
}

fn on_character_drag(
    mut ev_drag: EventReader<DragEvent>,
    mut ev_tooltip: EventWriter<SetTooltipEvent>,
) {
    for DragEvent(entity) in ev_drag.iter() {
        ev_tooltip.send(SetTooltipEvent(*entity, None));
    }
}
