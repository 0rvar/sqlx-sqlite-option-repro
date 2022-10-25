use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, types::chrono::NaiveDateTime, ConnectOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "db.sqlite3";
    let mut connection = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
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
