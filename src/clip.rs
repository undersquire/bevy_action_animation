use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Clip {
    pub first: usize,
    pub last: usize,
}
