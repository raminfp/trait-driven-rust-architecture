use traity_app::{User, Vault, VaultState, Result};
use traity_app::traits::serialize::{MyDeserialize, MySerialize};

#[test]
fn flow_deposit_withdraw_and_serde() -> Result<()> {
    let user = User::new(42, "tester", None)?;
    let mut vault = Vault::new(1, user.id);
    vault.activate();

    vault.deposit(10_000)?;
    assert_eq!(vault.balance, 10_000);

    let got = vault.withdraw(user.id, 7_000)?;
    assert_eq!(got, 7_000);
    assert_eq!(vault.balance, 3_000);

    // roundtrip serialization
    let mut buf = Vec::new();
    vault.try_serialize(&mut buf)?;
    let copy: Vault = MyDeserialize::try_deserialize(&buf)?;
    assert_eq!(copy.balance, 3_000);

    Ok(())
}
