pub mod prelude;
pub mod error;
pub mod traits;
pub mod domain;

pub mod infra;


pub use crate::{
    error::{ErrorCode, Result},
    domain::{user::User, vault::Vault, state::VaultState},
    traits::{serialize::*, owner::Owner},
};
