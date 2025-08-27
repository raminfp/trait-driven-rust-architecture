use serde::{Deserialize, Serialize};
use crate::{ErrorCode, Result};
use crate::traits::owner::Owner;
use super::state::VaultState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vault {
    pub id: u64,
    pub authority: u64,
    pub balance: u64,
    pub state: VaultState,
}

impl Vault {
    pub fn new(id: u64, authority: u64) -> Self {
        Self { id, authority, balance: 0, state: VaultState::Uninitialized }
    }

    pub fn activate(&mut self) { self.state = VaultState::Active { frozen: false }; }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        match self.state {
            VaultState::Active { frozen: false } => {
                self.balance = self.balance.checked_add(amount).ok_or(ErrorCode::InvalidData)?;
                Ok(())
            }
            VaultState::Active { frozen: true } => Err(ErrorCode::NotOwner),
            _ => Err(ErrorCode::InvalidData),
        }
    }

    pub fn withdraw(&mut self, owner: u64, amount: u64) -> Result<u64> {
        if owner != self.authority { return Err(ErrorCode::NotOwner); }
        match self.state {
            VaultState::Active { frozen: false } => {
                if self.balance < amount { return Err(ErrorCode::InvalidData); }
                self.balance -= amount;
                Ok(amount)
            }
            _ => Err(ErrorCode::InvalidData),
        }
    }
}

impl Owner for Vault {
    type OwnerId = u64;
    fn owner_id(&self) -> Self::OwnerId { self.authority }
}
