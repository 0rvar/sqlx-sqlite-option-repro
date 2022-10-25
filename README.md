# Issue with SQLx and Sqlite when using ORDER BY and LIMIT

This is a minimal example of an issue I'm having with SQLx and Sqlite when using `ORDER BY` and `LIMIT`.
SQLx incorrectly decides that `NOT NULL` fields should map to `Option<T>`.

## Run reproduction

```bash
cargo sqlx database setup
cargo build
```

## Result

The build fails with below error, even though all fields in the table are declared as `NOT NULL`. Removing either ORDER BY or LIMIT will make the build succeed.

```
error[E0308]: mismatched types
  --> src/main.rs:18:16
   |
18 |       let foos = sqlx::query_as!(
   |  ________________^
19 | |         Foo,
20 | |         r#"
21 | |             SELECT *
...  |
26 | |         10
27 | |     )
   | |_____^ expected struct `String`, found enum `Option`
   = note: expected struct `String`
                found enum `Option<String>`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_as` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
  --> src/main.rs:18:16
   |
18 |       let foos = sqlx::query_as!(
   |  ________________^
19 | |         Foo,
20 | |         r#"
21 | |             SELECT *
...  |
26 | |         10
27 | |     )
   | |_____^ expected struct `NaiveDateTime`, found enum `Option`
   |
   = note: expected struct `NaiveDateTime`
                found enum `Option<NaiveDateTime>`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_as` (in Nightly builds, run with -Z macro-backtrace for more info)
```
