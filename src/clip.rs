use super::*;

/// Represents a single clip from a texture atlas (supports negative-direction indexes).
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct Clip(pub usize, pub usize);

/// Represents a map of clip identifiers to the clip information.
#[derive(Debug, Clone, Default, Deref, DerefMut, Component)]
pub struct ClipMap(pub HashMap<String, Clip>);
