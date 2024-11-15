use cala_ledger::{account::*, CalaLedger};

pub async fn list(cala: CalaLedger) -> anyhow::Result<()> {
    let accounts = cala.accounts().list(Default::default()).await?.entities;

    println!("ALL ACCOUNTS");
    for account in accounts {
        println!(
            "{}",
            serde_json::to_string(&account.values()).expect("serde")
        );
    }
    println!("DONE");
    Ok(())
}

pub async fn create(cala: CalaLedger, name: String) -> anyhow::Result<()> {
    let new_account = NewAccount::builder()
        .id(AccountId::new())
        .name(name.clone())
        .code(name)
        .build()?;

    let account = cala.accounts().create(new_account).await?;

    println!("CREATED ACCOUNT");
    println!(
        "{}",
        serde_json::to_string(&account.values()).expect("serde")
    );
    println!("DONE");
    Ok(())
}

pub const ASSETS_ACCOUNT_ID: uuid::Uuid = uuid::uuid!("00000000-0000-0000-0000-000000000000");
pub async fn assets() -> NewAccount {
    let new_account = NewAccount::builder()
        .id(ASSETS_ACCOUNT_ID)
        .name("ASSETS")
        .code("ASSETS")
        .build()
        .expect("assets account");
    new_account
}
