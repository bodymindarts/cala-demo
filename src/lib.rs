#![allow(unused_imports)]
#![allow(dead_code)]

mod accounts;
mod balance;
mod cli;
mod deposit;
mod journal;
mod transfer;

use cala_ledger::{tx_template::NewTxTemplate, CalaLedger, CalaLedgerConfig};
use clap::Parser;

use cli::*;

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let pg_con = "postgres://user:password@localhost:5432/pg";
    let cala_config = CalaLedgerConfig::builder()
        .pg_con(pg_con)
        .exec_migrations(true)
        .build()?;
    let cala = CalaLedger::init(cala_config).await?;

    let _ = init_template(&cala, deposit::template()).await;
    let _ = init_template(&cala, transfer::template()).await;

    match cli.command {
        Command::CreateJournal => {
            journal::init(&cala).await?;
        }
        Command::ListAccounts => {
            accounts::list(cala).await?;
        }
        Command::CreateAccount { name } => {
            accounts::create(cala, name).await?;
        }
        Command::CreateAssetsAccount => {
            accounts::init_assets(cala).await?;
        }
        Command::Deposit { name, amount } => {
            deposit::execute(cala, name, amount).await?;
        }
        Command::Transfer {
            sender,
            recipient,
            amount,
        } => {
            transfer::execute(cala, sender, recipient, amount).await?;
        }
        Command::Balance { name } => {
            balance::fetch(cala, name).await?;
        }
    }
    Ok(())
}

async fn init_template(cala: &CalaLedger, template: NewTxTemplate) -> anyhow::Result<()> {
    cala.tx_templates().create(template).await?;
    Ok(())
}
