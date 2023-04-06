mod asset;
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
use bevy_ecs::{
    prelude::{Bundle, Component},
    schedule::IntoSystemConfigs,
};
use bevy_reflect::TypeUuid;
use bevy_time::{Timer, TimerMode};
use serde::{Deserialize, Serialize};

use self::{
    asset::{AnimationMapLoader, ClipMapLoader},
    clip::Clip,
};

pub use self::group::{Animation, AnimationGroup, AnimationMode};

#[derive(Clone, Default, TypeUuid, Serialize, Deserialize)]
#[uuid = "ffb2128b-453f-41d2-a174-022aa35e71d7"]
pub struct ClipMap {
    pub clips: Vec<Clip>,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Default, Deref, DerefMut, TypeUuid, Serialize, Deserialize)]
#[uuid = "8c238f61-5218-45b0-8d57-e3b77e275173"]
pub struct AnimationMap<T: Action>(HashMap<T, AnimationGroup<T>>);

#[derive(Clone, Default, Deref, DerefMut, Component)]
pub struct AnimationClip(pub Clip);

#[derive(Clone, Default, Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub mode: AnimationMode,
}

#[derive(Clone, Default, Deref, DerefMut, Component)]
pub struct AnimationQueue<T: Action>(pub VecDeque<Animation<T>>);

#[derive(Clone, Default, Deref, DerefMut, Component)]
pub struct AnimationTriggers<T: Action>(pub Vec<T>);

#[derive(Clone, Bundle)]
pub struct AnimationBundle<T: Action> {
    pub clip_map: Handle<ClipMap>,
    pub animation_map: Handle<AnimationMap<T>>,
    pub clip: AnimationClip,
    pub timer: AnimationTimer,
    pub queue: AnimationQueue<T>,
    pub triggers: AnimationTriggers<T>,
}

impl<T: Action> Default for AnimationBundle<T> {
    fn default() -> Self {
        Self {
            clip_map: Default::default(),
            animation_map: Default::default(),
            clip: Default::default(),
            timer: AnimationTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                mode: AnimationMode::Once,
            },
            queue: AnimationQueue::<T>::default(),
            triggers: AnimationTriggers::<T>::default(),
        }
    }
}

pub struct ClipPlugin;

impl Plugin for ClipPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<ClipMap>()
            .init_asset_loader::<ClipMapLoader>();
    }
}

#[derive(Default)]
pub struct AnimationPlugin<T: Action + for<'a> Deserialize<'a>>(PhantomData<T>);

impl<T: Action + for<'a> Deserialize<'a>> Plugin for AnimationPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_asset::<AnimationMap<T>>()
            .init_asset_loader::<AnimationMapLoader<T>>()
            .add_systems(
                (
                    system::queue_animations::<T>,
                    system::process_animations::<T>,
                )
                    .chain()
                    .in_base_set(CoreSet::PostUpdate),
            );
    }
}
