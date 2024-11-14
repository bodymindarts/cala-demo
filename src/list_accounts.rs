use cala_ledger::CalaLedger;

pub async fn run(cala: CalaLedger) -> anyhow::Result<()> {
    let accounts = cala.accounts().list(Default::default()).await?.entities;
    println!("ALL ACCOUNTS");
    for account in accounts {
        println!("{}", serde_json::to_string(&account.values()).expect("serde"));
    }
    println!("DONE");
    Ok(())
}
