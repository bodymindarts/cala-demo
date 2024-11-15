use cala_ledger::{account::*, tx_template::*, CalaLedger, JournalId, TransactionId};
use rust_decimal::Decimal;

pub fn template() -> NewTxTemplate {
    let entries = vec![
        NewTxTemplateEntry::builder()
            .entry_type("'DEPOSIT_CR'")
            .account_id("params.recipient")
            .layer("'SETTLED'")
            .direction("'CREDIT'")
            .units("params.amount")
            .currency("'BTC'")
            .build()
            .unwrap(),
        NewTxTemplateEntry::builder()
            .entry_type("'DEPOSIT_DR'")
            .account_id("params.assets")
            .layer("'SETTLED'")
            .direction("'DEBIT'")
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
            .name("assets")
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
        .code("DEPOSIT")
        .params(params)
        .transaction(transaction)
        .entries(entries)
        .build()
        .unwrap();
    new_tx_template
}

pub async fn execute(
    cala: CalaLedger,
    account_code: String,
    amount: Decimal,
) -> anyhow::Result<()> {
    let recipient_account = cala.accounts().find_by_code(account_code).await?;
    let mut params = Params::new();
    params.insert("journal_id", super::journal::JOURNAL_ID);
    params.insert("assets", super::accounts::ASSETS_ACCOUNT_ID);
    params.insert("recipient", recipient_account.id());
    params.insert("amount", amount);

    cala.post_transaction(TransactionId::new(), "DEPOSIT", params)
        .await
        .unwrap();

    Ok(())
}
