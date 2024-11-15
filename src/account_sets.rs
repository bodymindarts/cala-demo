use cala_ledger::{account_set::*, CalaLedger};

pub const LIABILITIES_ACCOUNT_SET_ID: uuid::Uuid =
    uuid::uuid!("10000000-0000-0000-0000-000000000000");
pub async fn create(cala: CalaLedger) -> anyhow::Result<()> {
    let new_set = NewAccountSet::builder()
        .id(LIABILITIES_ACCOUNT_SET_ID)
        .name("LIABILITIES")
        .journal_id(super::journal::JOURNAL_ID)
        .build()
        .unwrap();

    let account_set = cala.account_sets().create(new_set).await?;

    println!("CREATED ACCOUNT SET");
    println!(
        "{}",
        serde_json::to_string_pretty(&account_set.values()).expect("serde")
    );
    println!("DONE");
    Ok(())
}

pub async fn add_member(cala: CalaLedger, name: String) -> anyhow::Result<()> {
    let member = cala.accounts().find_by_code(name).await?;
    cala.account_sets()
        .add_member(LIABILITIES_ACCOUNT_SET_ID.into(), member.id())
        .await?;
    println!("DONE");
    Ok(())
}
