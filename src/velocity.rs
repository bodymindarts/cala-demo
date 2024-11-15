use cala_ledger::{velocity::*, CalaLedger};

const ACCOUNT_CONTROL_ID: uuid::Uuid = uuid::uuid!("20000000-0000-0000-0000-000000000000");

pub async fn init_overdraft_protection(cala: CalaLedger) -> anyhow::Result<()> {
    let mut op = cala.begin_operation().await?;

    let new_limit = NewVelocityLimit::builder()
        .id(VelocityLimitId::new())
        .name("Overdraft Protection")
        .description("Limit to prevent an account going negative")
        .window(vec![])
        .limit(
            NewLimit::builder()
                .balance(vec![NewBalanceLimit::builder()
                    .layer("SETTLED")
                    .amount("decimal('0')")
                    .enforcement_direction("DEBIT")
                    .build()
                    .expect("limit")])
                .build()
                .expect("limit"),
        )
        .build()
        .expect("build limit");

    let limit = cala
        .velocities()
        .create_limit_in_op(&mut op, new_limit)
        .await?;

    let control = NewVelocityControl::builder()
        .id(ACCOUNT_CONTROL_ID)
        .name("Customer Account Control")
        .description("Constrains movements of funds on customer accoutns")
        .build()
        .expect("build control");
    let control = cala
        .velocities()
        .create_control_in_op(&mut op, control)
        .await?;

    cala.velocities()
        .add_limit_to_control_in_op(&mut op, control.id(), limit.id())
        .await?;

    op.commit().await?;

    println!("DONE");

    Ok(())
}

pub async fn attach(cala: CalaLedger, account_code: String) -> anyhow::Result<()> {
    let account = cala.accounts().find_by_code(account_code).await?;
    cala.velocities()
        .attach_control_to_account(ACCOUNT_CONTROL_ID.into(), account.id(), Params::new())
        .await?;
    println!("DONE");
    Ok(())
}
