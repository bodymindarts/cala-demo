use clap::{Parser, Subcommand};
use rust_decimal::Decimal;

#[derive(Parser)]
#[clap(long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    CreateJournal,
    ListAccounts,
    CreateAccount {
        name: String,
    },
    CreateAssetsAccount,
    Deposit {
        name: String,
        amount: Decimal,
    },
    Withdraw {
        name: String,
        amount: Decimal,
    },
    Transfer {
        sender: String,
        recipient: String,
        amount: Decimal,
    },
    Balance {
        name: String,
    },
    CreateLiabilitiesAccountSet,
    LiabilitiesBalance,
    AddLiabilitiesMember {
        name: String,
    },
    InitOverdraft,
    AttachOverdraftProtection {
        name: String,
    },
    WatchEvents,
}
