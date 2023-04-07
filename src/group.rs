use bevy_action::Action;
use serde::{Deserialize, Serialize};

/// Represents the animation-play mode.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationMode {
    /// Plays the entire animation clip, once.
    #[default]
    Once,
    /// Cycles the animation clip, infinitely, until cancelled (by another animation on the queue).
    Repeating,
}

/// Represents attributes that can apply to different animation clips.
#[derive(Clone, Serialize, Deserialize)]
pub enum AnimationAttribute<T: Action> {
    /// Flips the sprite horizontally during the animation.
    FlipX,
    /// Flips the sprite vertically during the animation.
    FlipY,
    /// Triggers the given action when the animation finishes (`Once`) or is cancelled (`Repeating`).
    Trigger(T),
}

/// Represents a single animation clip.
#[derive(Clone, Serialize, Deserialize)]
pub struct Animation<T: Action> {
    pub clip: usize,
    pub rate: f32,
    pub mode: AnimationMode,
    pub attributes: Vec<AnimationAttribute<T>>,
}

/// The order in which the animations in an animation group are added to the queue.
#[derive(Clone, Default, Serialize, Deserialize)]
pub enum AnimationGroupOrderMode {
    /// In the order they appear, this is the default.
    #[default]
    Sequential,
    /// In a random order.
    Random,
    /// Randomly select one animation from the group to add to the queue.
    RandomSelect,
}

/// Represents a group of animations bound to an action.
#[derive(Clone, Serialize, Deserialize)]
pub struct AnimationGroup<T: Action> {
    pub clips: Vec<Animation<T>>,
    pub ordering: Option<AnimationGroupOrderMode>,
}
