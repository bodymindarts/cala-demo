use cala_ledger::{journal::*, CalaLedger};

pub const JOURNAL_ID: uuid::Uuid = uuid::uuid!("00000000-0000-0000-0000-000000000000");

pub async fn init(cala: &CalaLedger) -> anyhow::Result<()> {
    let new_journal = NewJournal::builder()
        .id(JOURNAL_ID)
        .name("MAIN JOURNAL")
        .build()?;

    let journal = cala.journals().create(new_journal).await?;
    println!("CREATED Journal");
    println!(
        "{}",
        serde_json::to_string_pretty(&journal.values()).expect("serde")
    );
    println!("DONE");

    Ok(())
}
