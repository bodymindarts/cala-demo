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

pub async fn fetch_liabilities(cala: CalaLedger) -> anyhow::Result<()> {
    let balance = cala
        .balances()
        .find(
            super::journal::JOURNAL_ID.into(),
            super::account_sets::LIABILITIES_ACCOUNT_SET_ID,
            "BTC".parse().unwrap(),
        )
        .await?;
    println!("LIABILITIES BALANCE");
    println!("Settled Balance: {}", balance.settled());
    println!("DETAILS");
    println!(
        "{}",
        serde_json::to_string_pretty(&balance.details).expect("serde")
    );

    Ok(())
}
