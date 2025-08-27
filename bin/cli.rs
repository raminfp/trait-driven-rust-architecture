#![cfg(feature = "cli")]
use traity_app::prelude::*;
use traity_app::{User, Vault};

fn main() -> Result<()> {
    let user = User::new(1, "ramin", Some("ramin@example.com"))?;
    let mut vault = Vault::new(100, user.id);

    vault.activate();
    vault.deposit(25000)?;


    let _ = vault.withdraw(user.id, 5000)?;

    let mut buf = Vec::new();
    #[allow(unused_mut)]
    let mut writer = std::io::Cursor::new(&mut buf);
    use traity_app::traits::serialize::MySerialize;
    vault.try_serialize(&mut writer)?;

    println!("ok: balance={}", vault.balance);
    Ok(())
}
