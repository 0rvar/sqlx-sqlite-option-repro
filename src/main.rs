use sqlx::types::chrono::NaiveDateTime;

struct Foo {
    id: String,
    name: String,
    updated_at: NaiveDateTime,
}

fn main() {
    sqlx::query_as!(
        Foo,
        r#"
            SELECT *
            FROM foos
            ORDER BY updated_at DESC
            LIMIT 10
        "#,
    );
}
