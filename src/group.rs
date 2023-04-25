use super::*;

/// Represents a single animation clip.
#[derive(Clone)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct AnimationClip<T: Action> {
    pub id: String,
    pub frame_speed: f32,
    pub looping: Option<bool>,
    pub triggers: Vec<T>,
}

/// The order in which the animations in an animation group are added to the queue.
#[derive(Clone, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub enum AnimationClipOrder {
    /// In the order they appear, this is the default.
    #[default]
    Sequential,
    /// In a random order.
    Random,
    /// Randomly select one animation from the group to add to the queue.
    RandomSelect,
}

/// Represents a group of animations bound to an action.
#[derive(Clone)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct AnimationSet<T: Action> {
    pub clips: Vec<AnimationClip<T>>,
    pub ordering: Option<AnimationClipOrder>,
}
