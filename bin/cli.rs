#![cfg(feature = "cli")]
use traity_app::prelude::*;
use traity_app::{User, Vault};
use traity_app::traits::serialize::MySerialize;
use std::io::Cursor;

fn main() -> Result<()> {
    let user = User::new(1, "ramin", Some("ramin@example.com"))?;
    let mut vault = Vault::new(100, user.id);

    vault.activate();
    vault.deposit(25000)?;

    // let mut vault = Vault::new(1, 100);
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    vault.try_serialize(&mut cursor)?;
    println!("{:?}", vault);
    let _ = vault.withdraw(user.id, 5000)?;

    let mut buf = Vec::new();
    #[allow(unused_mut)]
    let mut writer = Cursor::new(&mut buf);
    vault.try_serialize(&mut writer)?;

    println!("ok: balance={}", vault.balance);
    Ok(())
}
