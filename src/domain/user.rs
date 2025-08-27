use serde::{Deserialize, Serialize};
use crate::{ErrorCode, Result};
use crate::traits::owner::Owner;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
}

impl User {
    pub fn new(id: u64, username: &str, email: Option<&str>) -> Result<Self> {
        if username.is_empty() { return Err(ErrorCode::MissingField("username")); }
        Ok(Self { id, username: username.into(), email: email.map(|s| s.into()) })
    }
}

impl Owner for User {
    type OwnerId = u64;
    fn owner_id(&self) -> Self::OwnerId { self.id }
}
