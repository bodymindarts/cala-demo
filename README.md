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
```

## Code time
Overview of the code

```sh
tree src
alias demo="cargo run --"
demo help
```

Lets setup the boilerplate to initialize the ledger, a journal and some accounts.

Initialize the ledger:

[lib.rs](https://github.com/bodymindarts/cala-demo/blob/083e6a40016817a5ea44faa39b0a0490417bcfec/src/lib.rs#L19-L24)
```rust
let pg_con = "postgres://user:password@localhost:5432/pg";
let cala_config = CalaLedgerConfig::builder()
    .pg_con(pg_con)
    .exec_migrations(true)
    .build()?;
let cala = CalaLedger::init(cala_config).await?;
// ... //
```

Create the main journal:

[journals.rs](https://github.com/bodymindarts/cala-demo/blob/083e6a40016817a5ea44faa39b0a0490417bcfec/src/journal.rs#L6-L11)
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

[accounts.rs](https://github.com/bodymindarts/cala-demo/blob/fabab98ffc89e5870a5ec3b5a56a912c38c0d5e1/src/accounts.rs#L36-L45)
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
Templates are used to define the structure of a transaction.
Params can be injected and referenced via [CEL](https://cel.dev).

Here we have a template that represents deposits.

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
    account_id: "params.assets" # Extracting the injected sender from params via CEL
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

[deposit.rs](https://github.com/bodymindarts/cala-demo/blob/fabab98ffc89e5870a5ec3b5a56a912c38c0d5e1/src/deposit.rs#L70-L79)
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

### Transfer

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

[Template definition](https://github.com/bodymindarts/cala-demo/blob/c3d5bafc17404bec4fc08e8f73adeb981458b57d/src/transfer.rs#L4)

```sh
demo create-account "Bob"
demo transfer "Alice" "Bob" 200
demo balance "Alice"
demo balance "Bob"
demo balance "ASSETS"
```

### Account Sets

Account sets are used to keep track of balances of multiple accounts.

For example if I want to know how much money the bank owes in total to all of its customers I can create a 'Liabilities' account set to keep track of that.

[account_sets.rs](https://github.com/bodymindarts/cala-demo/blob/abb4d10af7434f81ec06fc064a73cace2c2ed3e7/src/account_sets.rs#L6-L13)
```rust
pub const LIABILITIES_ACCOUNT_SET_ID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");

let new_set = NewAccountSet::builder()
    .id(LIABILITIES_ACCOUNT_SET_ID)
    .name("LIABILITIES")
    .journal_id(super::journal::JOURNAL_ID) # Account sets are scoped to journals
    .build()
    .unwrap();

let account_set = cala.account_sets().create(new_set).await?;
```

```sh
demo create-account-set
```

In order for the Account Set to be useful we need to add some accounts to it.

[account_sets.rs](https://github.com/bodymindarts/cala-demo/blob/abb4d10af7434f81ec06fc064a73cace2c2ed3e7/src/account_sets.rs#L25-L28)
```rust
let member = cala.accounts().find_by_code(name).await?;
cala.account_sets()
    .add_member(LIABILITIES_ACCOUNT_SET_ID.into(), member.id())
    .await?;
```

```sh
demo add-liabilities-member "Alice"
demo liabilities-balance
demo add-liabilities-member "Bob"
demo liabilities-balance
demo transfer "Bob" "Alice" 100
```
