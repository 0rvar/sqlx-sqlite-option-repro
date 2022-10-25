use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, types::chrono::NaiveDateTime, ConnectOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut connection = SqliteConnectOptions::from_str("db.sqlite3")?
        .connect()
        .await?;

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
