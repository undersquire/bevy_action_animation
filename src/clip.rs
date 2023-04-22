use super::*;

/// Represents a single clip from a texture atlas (supports negative-direction indexes).
#[derive(Clone, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct Clip {
    pub first: usize,
    pub last: usize,
}
