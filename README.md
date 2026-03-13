# Generic ORM — Rust Demo App

A todo-list application built with Axum, rusqlite, and askama, demonstrating the [Generic ORM](https://github.com/notactuallytreyanastasio/generic_orm) compiled from Temper to Rust.

## Stack

| Component | Technology |
|-----------|-----------|
| Framework | Axum 0.8 |
| Templates | askama |
| Database | SQLite via rusqlite (bundled) |
| ORM | [Generic ORM](https://github.com/notactuallytreyanastasio/generic_orm) (vendored) |
| Port | 5003 |

## ORM Usage

```rust
use orm::src::{from, safe_identifier, changeset, delete_sql, SqlBuilder};

// SELECT — type-safe query builder
let q = from(safe_identifier("todos")?)
    .where_(where_fragment)
    .order_by(safe_identifier("created_at")?, true)
    .to_sql().to_string();

// INSERT — changeset pipeline with field whitelisting
let cs = changeset(table_def, params)
    .cast(vec![safe_identifier("title")?, safe_identifier("list_id")?])
    .validate_required(vec![safe_identifier("title")?]);
let sql = cs.to_insert_sql()?.to_string();

// DELETE — validated identifier
let sql = delete_sql(table_def, id)?.to_string();
```

## Security Model

- **Zero SQL injection vulnerabilities** — all queries generated through the ORM
- `safe_identifier()` validates table/column names against `[a-zA-Z_][a-zA-Z0-9_]*`
- Sealed `SqlPart` hierarchy handles per-type value escaping
- `Changeset.cast()` enforces field whitelisting (CWE-915 prevention)
- Async handlers with `tokio::spawn_blocking` for database operations
- DDL (`CREATE TABLE`) is the only raw SQL — static strings with no user input

## Running

```bash
cargo run
# Open http://localhost:5003
```

## Vendored ORM

The `vendor/` directory contains the ORM compiled from Temper to a Rust crate. Updated automatically by CI when the ORM source changes.

---

Part of the [Generic ORM](https://github.com/notactuallytreyanastasio/generic_orm) project — write once in Temper, secure everywhere.
