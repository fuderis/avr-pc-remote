use crate::{ prelude::*, Key };

/// The remote bind action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Handler { handler: String },
    Shortcut { shortcut: Vec<Key> },
    Press { press: Vec<Key> },
    Open { open: String },
}

/// The remote bind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bind {
    pub name: String,
    pub action: Action,
    pub repeat: bool
}
