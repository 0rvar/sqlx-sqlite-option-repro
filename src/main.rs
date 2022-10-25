use std::{env, str::FromStr};

use sqlx::{
    migrate::Migrator, sqlite::SqliteConnectOptions, types::chrono::NaiveDateTime, ConnectOptions,
};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();
    let url = env::var("DATABASE_URL").unwrap();
    let mut connection = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
        .connect()
        .await?;

    MIGRATOR.run(&mut connection).await?;

    struct Foo {
        id: String,
        name: String,
        updated_at: NaiveDateTime,
    }

    let foos = sqlx::query_as!(
        Foo,
        r#"
            SELECT *
            FROM foos
            ORDER BY updated_at DESC
            LIMIT ?
        "#,
        10
    )
    .fetch_all(&mut connection)
    .await
    .expect("Failed to fetch last 10");

    Ok(())
}
