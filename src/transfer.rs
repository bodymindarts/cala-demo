use cala_ledger::{account::*, tx_template::*, CalaLedger, JournalId, TransactionId};
use rust_decimal::Decimal;

pub fn template() -> NewTxTemplate {
    let entries = vec![
        NewTxTemplateEntry::builder()
            .entry_type("'TRANSFER_DR'")
            .account_id("params.sender")
            .layer("'SETTLED'")
            .direction("'DEBIT'")
            .units("params.amount")
            .currency("'BTC'")
            .build()
            .unwrap(),
        NewTxTemplateEntry::builder()
            .entry_type("'TRANSFER_CR'")
            .account_id("params.recipient")
            .layer("'SETTLED'")
            .direction("'CREDIT'")
            .units("params.amount")
            .currency("'BTC'")
            .build()
            .unwrap(),
    ];
    let params = vec![
        NewParamDefinition::builder()
            .name("recipient")
            .r#type(ParamDataType::Uuid)
            .build()
            .unwrap(),
        NewParamDefinition::builder()
            .name("sender")
            .r#type(ParamDataType::Uuid)
            .build()
            .unwrap(),
        NewParamDefinition::builder()
            .name("journal_id")
            .r#type(ParamDataType::Uuid)
            .build()
            .unwrap(),
        NewParamDefinition::builder()
            .name("amount")
            .r#type(ParamDataType::Decimal)
            .build()
            .unwrap(),
    ];
    let transaction = NewTxTemplateTransaction::builder()
        .journal_id("params.journal_id")
        .effective("date()")
        .build()
        .expect("transaction");
    let tx_template_id = TxTemplateId::new();

    let new_tx_template = NewTxTemplate::builder()
        .id(tx_template_id)
        .code("TRANSFER")
        .params(params)
        .transaction(transaction)
        .entries(entries)
        .build()
        .unwrap();
    new_tx_template
}

pub async fn execute(
    cala: CalaLedger,
    sender_code: String,
    recipient_code: String,
    amount: Decimal,
) -> anyhow::Result<()> {
    let sender_account = cala.accounts().find_by_code(sender_code).await?;
    let recipient_account = cala.accounts().find_by_code(recipient_code).await?;
    let mut params = Params::new();
    params.insert("journal_id", super::journal::JOURNAL_ID);
    params.insert("sender", sender_account.id());
    params.insert("recipient", recipient_account.id());
    params.insert("amount", amount);

    let transaction = cala
        .post_transaction(TransactionId::new(), "TRANSFER", params)
        .await?;
    println!("CREATED TRANSACTION");
    println!(
        "{}",
        serde_json::to_string_pretty(&transaction.values()).expect("serde")
    );
    Ok(())
}
