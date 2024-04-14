use bevy::prelude::*;

pub(crate) mod anchors;
pub(crate) mod animation;
pub(crate) mod background;
pub(crate) mod cursor;
pub(crate) mod dragndrop;
pub(crate) mod hover;
pub(crate) mod on_screen_log;
pub(crate) mod timer;
pub(crate) mod tooltip;

#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub enum ChangeDetectionSystemSet {
    MouseDetection,
    MouseDetectionFlush,
    Tooltip,
    TooltipRender,
    Animation,
}

pub(crate) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            ChangeDetectionSystemSet::MouseDetection
                .before(ChangeDetectionSystemSet::MouseDetectionFlush)
                .before(ChangeDetectionSystemSet::Animation)
                .before(ChangeDetectionSystemSet::Tooltip)
                .before(ChangeDetectionSystemSet::TooltipRender),
        )
        .add_systems(
            Update,
            apply_deferred.in_set(ChangeDetectionSystemSet::MouseDetectionFlush),
        )
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(hover::HoverPlugin)
        .add_plugins(dragndrop::DragNDropPlugin)
        .add_plugins(on_screen_log::OnScreenLogPlugin)
        .add_plugins(timer::TimerPlugin)
        .add_plugins(anchors::AnchorsPlugin)
        .add_plugins(cursor::CursorPlugin)
        .add_plugins(tooltip::TooltipPlugin)
        .add_plugins(background::BackgroundPlugin);
    }
}
