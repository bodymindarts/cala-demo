# Demo for using Cala Ledger rust library

We are going to discuss:
- some accounting basics
- how to implement a simple example in Rust

## Accounting basics

In accounting we record fanancial transactions in a ledger.
A ledger has (potentially multiple) journals.
Journals are the things that hold the (ordered) history of transactions
and the balances of the ledger accounts.

| JOURNAL 1     |
|--------------|
| Transaction 1|
| Transaction 2|
| Transaction 3|


Each transaction contains a series of Entries.
Entries record the amount of money that is moved from one account to another.

TRANSACTION 1
| ENTRY        | ACCOUNT  | DEBIT | CREDIT |
|--------------|---|---|---|
| Entry 1      |Account 1 | 100| |
| Entry 2      |Account 2 | | 100 |

The sum of the debits must be equal to the sum of the credits.
This is called the double-entry principle.
We use double-entry bookkeeping because it enforces that no money can be created or destroyed in the system.
Every debit must have a corresponding credit, making it impossible to "lose track" of funds or make unbalanced changes to accounts.

TRANSACTION 2
| ENTRY        | ACCOUNT  | DEBIT | CREDIT |
|--------------|---|---|---|
| Entry 1      |Account 1 | 25| |
| Entry 2      |Account 2 | | 50 |
| Entry 3      |Account 3 | 30 |  |
| Entry 4      |Account 2 | | 5 |

The number of entries per transaction is variable.
Accounts can show up multiple times in one transaction.
As long as the whole transaction balances out, it is valid.

## Code time

Lets setup the boilerplate to initialize the ledger, a journal and some accounts.

lib.rs
```rust
let pg_con = "postgres://user:password@localhost:5432/pg";
let cala_config = CalaLedgerConfig::builder()
    .pg_con(pg_con)
    .exec_migrations(true)
    .build()?;
let cala = CalaLedger::init(cala_config).await?;
// ... //
```
journals.rs
```rust
pub const JOURNAL_ID: uuid::Uuid = uuid::uuid!("00000000-0000-0000-0000-000000000000");

let new_journal = NewJournal::builder()
    .id(JOURNAL_ID)
    .name("MAIN JOURNAL")
    .description("the primary journal")
    .build()?;

if let Ok(journal) = cala.journals().create(new_journal).await {
  //
}
```
