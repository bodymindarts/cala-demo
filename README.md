# Demo for using Cala Ledger rust library

We are going to discuss:
- some accounting basics
- how to implement a simple example in Rust

## Accounting basics

In accounting we record financial transactions in a ledger.
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


In Cala entries actually have more information - including the currency and the layer as well as some metadata.
ACTUALLY
| ENTRY        | ACCOUNT  | DEBIT | CREDIT | CURRENCY | LAYER |
|--------------|---|---|---|---|---|
| Entry 1      |Account 1 | 25| | BTC | SETTLED |


In Cala the `debit == credit` rule is extended and must hold for each currency and each layer (`ENCUMBRANCE`, `PENDING`, `SETTLED`).

## Demo setup

Using the following command we can setup an environment.

```sh
git clone git@github.com:bodymindarts/cala-demo.git
cd cala-demo
direnv reload           # downloads the dependencies via nix
docker compose up -d    # starts the postgres container
tree src
```

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

let journal = cala.journals().create(new_journal).await?;
```

```sh
alias demo="cargo run --"
demo create-journal
```
### Making a deposit

In order to corretly record a deposit we need 2 accounts.
One account represents the assets that the bank holds.
The other account represents the liability it has towards a particular customer.

DEPOSIT TRANSACTION
| ENTRY        | ACCOUNT  | DEBIT | CREDIT |
|--------------|---|---|---|
| Entry 1      | ASSETS | 1000 | |
| Entry 2      |CUSTOMER 1 | | 1000 |

accounts.rs
```rust
pub const ASSETS_ACCOUNT_ID: uuid::Uuid = uuid::uuid!("00000000-0000-0000-0000-000000000000");
let new_account = NewAccount::builder()
    .id(ASSETS_ACCOUNT_ID)
    .name("ASSETS")
    .code("ASSETS")
    .normal_balance_type(DebitOrCredit::Debit)
    .build()?;
cala.accounts().create(new_account).await?;
```

```sh
demo create-assets-account
demo create-account "Alice"
```
### Transaction templates
In order to record a transaction in Cala we first need to create a template.
In this case a 'deposit' template.

```yaml
code: "DEPOSIT"
transaction:
  journal_id: "params.journal_id"
  effective: "date()"
params:                         # Template inputs that will be interpolated into the transaction
  - name: "assets"
    type: "UUID"
  - name: "recipient"
    type: "UUID"
  - name: "journal_id"
    type: "UUID"
  - name: "amount"
    type: "DECIMAL"
entries:
  - entry_type: "DEPOSIT_DR"
    account_id: "params.assets" # Extracting the injected sender account from params
    layer: "SETTLED"            # Cala support 3 'layers': ENCUMBRANCE, PENDING, SETTLED
    direction: "CREDIT"
    units: "params.amount"
    currency: "BTC"
  - entry_type: "DEPOSIT_CR"
    account_id: "params.recipient"
    layer: "SETTLED"
    direction: "DEBIT"
    units: "params.amount"
    currency: "BTC"
```

The template is shown here in yaml to make it more compact.
In rust code it is a bit more verbose.

Once the template is created we can execute it:
```rust
let recipient_account = cala.accounts().find_by_code(account_code).await?;
let mut params = Params::new();
params.insert("journal_id", super::journal::JOURNAL_ID);
params.insert("assets", super::accounts::ASSETS_ACCOUNT_ID);
params.insert("recipient", recipient_account.id());
params.insert("amount", amount);

let transaction = cala
    .post_transaction(TransactionId::new(), "DEPOSIT", params)
    .await?;
```

```sh
demo deposit "Alice" 1000
demo balance "Alice"
demo balance "ASSETS"
```

### Transfer and Withdraw

To transfer money from one account to another we need to create a new template.
This template doesn't affect the assets account because we are only moving money between customers.

TRANSFER TRANSACTION
| ENTRY        | ACCOUNT  | DEBIT | CREDIT |
|--------------|---|---|---|
| Entry 1      | CUSTOMER 1 | 1000 | |
| Entry 2      |CUSTOMER 2 | | 1000 |

```yaml
code: "TRANSFER"
transaction:
  journal_id: "params.journal_id"
  effective: "date()"
params:
  - name: "recipient"
    type: "UUID"
  - name: "assets"
    type: "UUID"
  - name: "journal_id"
    type: "UUID"
  - name: "amount"
    type: "DECIMAL"
entries:
  - entry_type: "TRANSFER_CR"
    account_id: "params.sender"
    layer: "SETTLED"
    direction: "CREDIT"
    units: "params.amount"
    currency: "BTC"
  - entry_type: "TRANSFER_DR"
    account_id: "params.recipient"
    layer: "SETTLED"
    direction: "DEBIT"
    units: "params.amount"
    currency: "BTC"
```

```sh
demo create-account "Bob"
demo transfer "Alice" "Bob" 200
demo balance "Alice"
demo balance "Bob"
demo balance "ASSETS"
```
