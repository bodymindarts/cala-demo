mod list_accounts;
mod cli;

use cala_ledger::{CalaLedger, CalaLedgerConfig};
use clap::{Parser};

use cli::*;

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let pg_con = "postgres://user:password@localhost:5432/pg";
    let cala_config = CalaLedgerConfig::builder().pg_con(pg_con).exec_migrations(true).build()?;
    let cala = CalaLedger::init(cala_config).await?;

    match cli.command {
        Command::ListAccounts => {
            list_accounts::run(cala).await?;
        }
    }
    Ok(())
}
