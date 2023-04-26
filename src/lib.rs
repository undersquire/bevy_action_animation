mod clip;
mod group;
mod system;

use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

use bevy_action::Action;
use bevy_app::{App, CoreSet, Plugin};
use bevy_asset::{AddAsset, Handle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::{Bundle, Component, IntoSystemConfigs};
use bevy_reflect::TypeUuid;
use bevy_time::{Timer, TimerMode};

#[cfg(feature = "derive")]
use serde::{Deserialize, Serialize};

pub use clip::*;
use group::*;

/// Represents a map of action to corresponding animation.
#[derive(Clone, Default, Deref, DerefMut, TypeUuid)]
#[uuid = "8c54c8ca-0f56-4463-a1ca-e4c7bbc5a76b"]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct AnimationMap<T: Action + TypeUuid>(HashMap<T, AnimationSet<T>>);

/// Represents the current animation timing information (used by internal systems).
#[derive(Clone, Default, Component)]
pub struct AnimationState<T: Action> {
    pub clip: Clip,
    pub timer: Timer,
    pub looping: bool,
    pub triggers: Vec<T>,
}

/// Represents the animation queue (used by internal systems).
#[derive(Clone, Default, Deref, DerefMut, Component)]
pub struct AnimationQueue<T: Action>(pub VecDeque<AnimationClip<T>>);

/// A bundle of all necessary components for the animation system to work.
#[derive(Clone, Bundle)]
pub struct AnimationBundle<T: Action + TypeUuid> {
    pub map: Handle<AnimationMap<T>>,
    pub state: AnimationState<T>,
    pub queue: AnimationQueue<T>,
}

impl<T: Action + TypeUuid> Default for AnimationBundle<T> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            state: AnimationState {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                ..Default::default()
            },
            queue: AnimationQueue::<T>::default(),
        }
    }
}

/// Plugin registered for every `Action` type, registering asset, asset loading, and internal systems for processing these animations.
///
/// NOTE: Internal animation systems are added to `CoreSet::PostUpdate`.
#[derive(Default)]
pub struct AnimationPlugin<T: Action + TypeUuid>(PhantomData<T>);

impl<T: Action + TypeUuid> Plugin for AnimationPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_asset::<AnimationMap<T>>().add_systems(
            (
                system::queue_animations::<T>,
                system::process_animations::<T>,
            )
                .chain()
                .in_base_set(CoreSet::PostUpdate),
        );
    }
}
