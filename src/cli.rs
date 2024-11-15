use clap::{Parser, Subcommand};
use rust_decimal::Decimal;

#[derive(Parser)]
#[clap(long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(env = "PG_CON")]
    pg_con: String,
}

#[derive(Subcommand)]
pub enum Command {
    ListAccounts,
    CreateAccount { name: String },
    Deposit { name: String, amount: Decimal },
}
