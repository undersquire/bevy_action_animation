use bevy_action::Action;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationMode {
    #[default]
    Once,
    Repeating,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AnimationAttribute<T: Action> {
    FlipX,
    FlipY,
    Trigger(T),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Animation<T: Action> {
    pub clip: usize,
    pub rate: f32,
    pub mode: AnimationMode,
    pub attributes: Vec<AnimationAttribute<T>>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum AnimationGroupOrderMode {
    #[default]
    Sequential,
    Random,
    RandomSelect,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AnimationGroup<T: Action> {
    pub clips: Vec<Animation<T>>,
    pub ordering: Option<AnimationGroupOrderMode>,
}
