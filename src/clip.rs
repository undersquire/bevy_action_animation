use serde::{Deserialize, Serialize};

/// Represents a single clip from a texture atlas (supports negative-direction indexes).
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Clip {
    pub first: usize,
    pub last: usize,
}
