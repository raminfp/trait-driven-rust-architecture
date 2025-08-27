use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VaultState {
    Uninitialized,
    Active { frozen: bool },
    Closed,
}
