use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Todo {
    pub content: String,
    pub author: String,
}
