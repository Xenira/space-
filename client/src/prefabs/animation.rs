use crate::components::animation::{
    Animation, AnimationDirection, AnimationIndices, AnimationRepeatType, AnimationState,
    AnimationTransition, AnimationTransitionType,
};
use bevy::prelude::*;

pub struct AnimationBuilder {
    path: Name,
    duration: f32,
    curve: AnimationCurve,
    target: AnimationTarget,
}

impl AnimationBuilder {
    pub fn new(path: Name) -> Self {
        Self {
            path,
            duration: 0.0,
            curve: AnimationCurve::Linear,
            target: AnimationTarget::Position(Vec3::ZERO),
        }
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_curve(mut self, curve: AnimationCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn with_target(mut self, target: AnimationTarget) -> Self {
        self.target = target;
        self
    }

    pub fn build(self) -> AnimationClip {
        let mut animation = AnimationClip::default();
        let (timesteps, keyframes) = self.build_timesteps();

        animation.add_curve_to_path(
            EntityPath {
                parts: vec![self.path],
            },
            VariableCurve {
                keyframe_timestamps: timesteps,
                keyframes,
            },
        );

        animation
    }

    fn build_timesteps(&self) -> (Vec<f32>, Keyframes) {
        let mut timesteps = Vec::new();
        let mut keyframes = Keyframes::Translation(Vec::new());

        let mut current_time = 0.0;

        timesteps.push(current_time);

        (timesteps, keyframes)
    }
}

pub enum AnimationCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

pub enum AnimationTarget {
    Position(Vec3),
    Rotation(Quat),
    Scale(Vec3),
}

impl AnimationTarget {
    pub fn from_entities(from: Entity, to: Entity) -> Self {
        let from_transform = Vec3::default();//get_transform(from);
        let to_transform = Vec3::default(); //get_transform(to);

        let position = to_transform - from_transform;

        Self::Position(position)
    }
}



pub fn simple(first: usize, last: usize) -> Animation {
    Animation::default()
        .with_state(AnimationState::new(
            "idle",
            AnimationIndices { first, last },
        ))
        .with_current_state("idle")
}

pub fn add_hover_state(animation: &mut Animation, first: usize, last: usize) {
    animation
        .add_state(
            AnimationState::new("hover", AnimationIndices { first, last })
                .with_repeat_type(AnimationRepeatType::Once)
                .with_fps(32.0),
        )
        .add_state(
            AnimationState::new("leave", AnimationIndices { first, last })
                .with_repeat_type(AnimationRepeatType::Once)
                .with_direction(AnimationDirection::Backward)
                .with_fps(32.0),
        )
        .add_global_transition(AnimationTransition {
            name: "hover".to_string(),
            transition_type: AnimationTransitionType::Imediate,
            to_state: "hover".to_string(),
        })
        .add_global_transition(AnimationTransition {
            name: "leave".to_string(),
            transition_type: AnimationTransitionType::Imediate,
            to_state: "leave".to_string(),
        });
}
