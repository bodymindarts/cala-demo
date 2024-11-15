use cala_ledger::CalaLedger;

pub async fn fetch(cala: CalaLedger, account_code: String) -> anyhow::Result<()> {
    let account = cala.accounts().find_by_code(account_code).await?;
    let balance = cala
        .balances()
        .find(
            super::journal::JOURNAL_ID.into(),
            account.id(),
            "BTC".parse().unwrap(),
        )
        .await?;
    println!("BALANCE");
    println!("Settled Balance: {}", balance.settled());
    println!("DETAILS");
    println!(
        "{}",
        serde_json::to_string_pretty(&balance.details).expect("serde")
    );

    Ok(())
}
