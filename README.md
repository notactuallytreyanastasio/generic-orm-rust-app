# Alloy Todo App -- Rust

A fully-functional todo list manager demonstrating **every** Alloy ORM feature. Built with Axum + Askama + rusqlite, styled with a retro Mac System 6 / Windows 95 hybrid UI.

## Overview

This application exercises the complete surface area of the Alloy ORM -- a compile-to-many-languages SQL query builder and changeset validation system written in Temper. Every route uses the ORM to generate SQL queries; the ORM API is called to build each query, and the generated SQL strings are displayed in the UI alongside the results.

24 routes cover: query building, all 5 join types, all 6 aggregate functions, 4 set operations, subqueries, changeset validation with all 14 validators, pagination, locking, safe identifiers, SqlBuilder with all append methods, timestamps, null ordering, and more.

## Data Model

Four-table schema with foreign key relationships:

```
lists (id, name, description, created_at)
  |
  +--< todos (id, title, completed, priority 1-5, due_date, list_id FK, created_at)
         |
         +--< todo_tags (id, todo_id FK, tag_id FK)
                                           |
                                     tags (id, name) >--+
```

- **lists** -- containers for grouping todos
- **todos** -- individual tasks with priority (1=low, 5=critical) and optional due date
- **tags** -- labels (urgent, home, office, errand) that can be assigned to any todo
- **todo_tags** -- join table linking todos to tags (many-to-many)

## Complete ORM Feature Coverage

Every ORM export is exercised by at least one route:

| ORM Feature | Category | Route(s) That Exercise It |
|---|---|---|
| `from` | Query Builder | All routes |
| `update` | Query Builder | `POST /todos/:id/toggle`, `POST /todos/bulk-complete`, `POST /lists/:id/update-desc` |
| `delete_from` | Query Builder | `POST /lists/:id/delete`, `POST /todos/:id/delete`, `POST /lists/:id/delete-completed` |
| `where` | WHERE | `GET /lists/:id`, `GET /overdue`, all CRUD routes |
| `or_where` | WHERE | `GET /search`, `POST /lists/:id/update-desc` |
| `where_not` | WHERE | `GET /overdue` |
| `where_null` | WHERE | `GET /overdue` (no-due-date query) |
| `where_not_null` | WHERE | `GET /lists/:id/high-priority`, `GET /overdue` |
| `where_between` | WHERE | `GET /lists/:id/high-priority` (priority 4-5), `GET /reports/combined` |
| `where_like` | WHERE | `GET /search` |
| `where_in` | WHERE | `POST /todos/bulk-complete` |
| `where_in_subquery` | WHERE | `GET /lists/:id/has-completed` |
| `select` | SELECT | `GET /lists/:id`, `GET /stats`, `GET /reports/combined` |
| `select_expr` | SELECT | `GET /` (aggregates), `GET /stats`, `GET /joins` |
| `distinct` | SELECT | `GET /stats` |
| `col` | Column Ref | `GET /`, `GET /safe-ids` |
| `count_all` | Aggregate | `GET /`, `GET /stats`, `GET /joins` |
| `count_col` | Aggregate | `GET /stats` |
| `sum_col` | Aggregate | `GET /stats` |
| `avg_col` | Aggregate | `GET /stats` |
| `min_col` | Aggregate | `GET /stats` |
| `max_col` | Aggregate | `GET /stats` |
| `count_sql` | Counting | `GET /stats`, `GET /lists/:id/page/:page` |
| `group_by` | Grouping | `GET /`, `GET /stats` |
| `having` | Grouping | `GET /stats` |
| `or_having` | Grouping | `GET /stats` |
| `order_by` | Ordering | `GET /`, `GET /lists/:id`, `GET /search`, `GET /stats` |
| `order_by_nulls` | Ordering | `GET /lists/:id` (NullsLast), `GET /order-nulls` |
| `NullsFirst` | Ordering | `GET /order-nulls` |
| `NullsLast` | Ordering | `GET /lists/:id`, `GET /order-nulls` |
| `limit` | Pagination | `GET /lists/:id`, `GET /lists/:id/high-priority`, `POST /todos/:id/delete`, `GET /lists/:id/page/:page`, `POST /lists/:id/delete-completed` |
| `offset` | Pagination | `GET /lists/:id/high-priority`, `GET /lists/:id/page/:page` |
| `safe_to_sql` | Pagination | `GET /search` (default limit 50) |
| `lock` (ForUpdate) | Locking | `GET /locks` |
| `lock` (ForShare) | Locking | `GET /locks` |
| `inner_join` | Joins | `GET /joins` |
| `left_join` | Joins | `GET /` (lists + todos), `GET /search`, `GET /joins` |
| `right_join` | Joins | `GET /joins` |
| `full_join` | Joins | `GET /joins` |
| `cross_join` | Joins | `GET /joins` (lists x tags) |
| `union_sql` | Set Operations | `GET /reports/combined` |
| `union_all_sql` | Set Operations | `GET /reports/combined` |
| `intersect_sql` | Set Operations | `GET /reports/combined` |
| `except_sql` | Set Operations | `GET /reports/combined` |
| `subquery` | Subqueries | `GET /lists/:id/has-completed` |
| `exists_sql` | Subqueries | `GET /lists/:id/has-completed` |
| `changeset` | Changeset | `POST /lists`, `POST /lists/:id/todos`, `POST /todos/:id/edit`, `GET /changeset-demo` |
| `cast` | Changeset | All changeset routes |
| `validate_required` | Validation | `POST /lists`, `POST /lists/:id/todos`, `GET /changeset-demo` |
| `validate_length` | Validation | `POST /lists`, `POST /lists/:id/todos`, `GET /changeset-demo` |
| `validate_int` | Validation | `POST /lists/:id/todos`, `GET /changeset-demo` |
| `validate_int64` | Validation | `GET /changeset-demo` |
| `validate_float` | Validation | `GET /changeset-demo` |
| `validate_bool` | Validation | `GET /changeset-demo` |
| `validate_number` | Validation | `POST /lists/:id/todos`, `GET /changeset-demo` |
| `validate_inclusion` | Validation | `POST /lists/:id/todos`, `GET /changeset-demo` |
| `validate_exclusion` | Validation | `POST /lists/:id/todos`, `GET /changeset-demo` |
| `validate_acceptance` | Validation | `GET /changeset-demo` |
| `validate_confirmation` | Validation | `GET /changeset-demo` |
| `validate_contains` | Validation | `GET /changeset-demo` |
| `validate_starts_with` | Validation | `GET /changeset-demo` |
| `validate_ends_with` | Validation | `GET /changeset-demo` |
| `to_insert_sql` | Changeset SQL | `POST /lists`, `POST /lists/:id/todos`, `GET /changeset-demo` |
| `to_update_sql` | Changeset SQL | `POST /todos/:id/edit`, `GET /changeset-demo` |
| `put_change` | Changeset Mutation | `POST /todos/:id/edit`, `GET /changeset-demo` |
| `get_change` | Changeset Mutation | `POST /todos/:id/edit`, `GET /changeset-demo` |
| `delete_change` | Changeset Mutation | `POST /todos/:id/edit`, `GET /changeset-demo` |
| `delete_sql` | Delete Helper | `POST /lists/:id/delete` |
| `timestamps` | Schema Helper | `GET /timestamps` |
| `safe_identifier` | Types | All routes, `GET /safe-ids` |
| `SafeIdentifier` | Types | All routes |
| `TableDef` | Types | Schema definitions |
| `FieldDef` | Types | Schema definitions, `GET /timestamps` |
| `StringField` | Types | Schema definitions |
| `IntField` | Types | Schema definitions |
| `SqlBuilder` | Types | All routes, `GET /safe-ids` (all append methods) |
| `SqlFragment` | Types | All routes |
| `SqlPart` | Types | `POST /todos/:id/toggle`, joins, set operations |
| `SqlString` | Types | `POST /lists/:id/update-desc` |
| `SqlInt32` | Types | `POST /todos/:id/toggle`, `GET /lists/:id/high-priority`, `POST /todos/bulk-complete` |
| `NumberValidationOpts` | Types | `POST /lists/:id/todos`, `GET /changeset-demo` |

## Route Reference

| # | Route | Method | Purpose | Key ORM Features |
|---|---|---|---|---|
| 1 | `/` | GET | Index -- lists with counts | `from`, `left_join`, `group_by`, `select_expr`, `count_all`, `col`, `order_by` |
| 2 | `/lists/:id` | GET | Show list with todos | `from`, `where`, `select`, `order_by`, `order_by_nulls` (NullsLast), `limit` |
| 3 | `/lists` | POST | Create list | `changeset`, `cast`, `validate_required`, `validate_length`, `to_insert_sql` |
| 4 | `/lists/:id/delete` | POST | Delete list cascade | `delete_from`, `where`, `delete_sql` |
| 5 | `/lists/:id/todos` | POST | Create todo | `changeset`, `cast`, `validate_required`, `validate_length`, `validate_int`, `validate_number`, `validate_inclusion`, `validate_exclusion`, `to_insert_sql` |
| 6 | `/todos/:id/toggle` | POST | Toggle completion | `update`, `set`, `where`, `SqlInt32` |
| 7 | `/todos/:id/edit` | POST | Edit todo | `changeset`, `put_change`, `get_change`, `delete_change`, `to_update_sql` |
| 8 | `/todos/:id/delete` | POST | Delete todo | `delete_from`, `where`, `limit` |
| 9 | `/search` | GET | Search todos | `from`, `left_join`, `where_like`, `or_where`, `safe_to_sql` |
| 10 | `/stats` | GET | Dashboard stats | `count_all`, `count_col`, `sum_col`, `avg_col`, `min_col`, `max_col`, `group_by`, `having`, `or_having`, `distinct`, `count_sql` |
| 11 | `/lists/:id/high-priority` | GET | High-priority view | `where_between`, `where_not_null`, `limit`, `offset` |
| 12 | `/overdue` | GET | Overdue todos | `where_not`, `where_not_null`, `where_null` |
| 13 | `/reports/combined` | GET | Set operations | `union_sql`, `union_all_sql`, `intersect_sql`, `except_sql` |
| 14 | `/lists/:id/has-completed` | GET | Subquery demo | `exists_sql`, `subquery`, `where_in_subquery` |
| 15 | `/todos/bulk-complete` | POST | Bulk complete | `where_in`, `update`, `set` |
| 16 | `/lists/:id/page/:page` | GET | Paginated todos | `limit`, `offset`, `count_sql` |
| 17 | `/locks` | GET | Lock modes demo | `lock` (ForUpdate, ForShare) |
| 18 | `/joins` | GET | All join types | `inner_join`, `left_join`, `right_join`, `full_join`, `cross_join` |
| 19 | `/changeset-demo` | GET | All 14 validators | `validate_int`, `validate_int64`, `validate_float`, `validate_bool`, `validate_inclusion`, `validate_exclusion`, `validate_number`, `validate_contains`, `validate_starts_with`, `validate_ends_with`, `validate_acceptance`, `validate_confirmation`, `to_insert_sql`, `to_update_sql` |
| 20 | `/timestamps` | GET | Timestamps helper | `timestamps` (returns FieldDef array) |
| 21 | `/lists/:id/update-desc` | POST | Update with or_where | `update`, `set`, `where`, `or_where`, `SqlString` |
| 22 | `/lists/:id/delete-completed` | POST | Delete with limit | `delete_from`, `where`, `limit` |
| 23 | `/safe-ids` | GET | SqlBuilder demo | `safe_identifier`, `SqlBuilder` (`append_safe`, `append_int32`, `append_int64`, `append_float64`, `append_string`, `append_fragment`), `col` |
| 24 | `/order-nulls` | GET | Null ordering demo | `order_by_nulls`, `NullsFirst`, `NullsLast` |

## Code Examples

### Query with LEFT JOIN, GROUP BY, and Aggregates (Index)

```rust
let query = from(state.sid_lists.clone())
    .left_join(
        state.sid_todos.clone(),
        join_on(&state.sid_lists, &state.sid_id, &state.sid_todos, &state.sid_list_id),
    )
    .select_expr(
        [
            col(state.sid_lists.clone(), state.sid_id.clone()),
            col(state.sid_lists.clone(), state.sid_name.clone()),
            count_all(),
        ].to_list(),
    )
    .group_by(state.sid_id.clone())
    .order_by(state.sid_name.clone(), true);

let sql = sql_to_string(query.to_sql());
```

### Changeset with All Validators (Create Todo)

```rust
let cs = changeset(state.todo_table.clone(), values)
    .cast(cast_fields.to_list())
    .validate_required([state.sid_title.clone(), state.sid_list_id.clone()].to_list())
    .validate_length(state.sid_title.clone(), 1, 200)
    .validate_int(state.sid_list_id.clone())
    .validate_int(state.sid_priority.clone())
    .validate_number(
        state.sid_priority.clone(),
        NumberValidationOpts::new(Some(0.0), Some(6.0), None, None, None),
    )
    .validate_inclusion(
        state.sid_priority.clone(),
        ["1", "2", "3", "4", "5"].map(|s| Arc::new(s.to_string())).to_list(),
    )
    .validate_exclusion(
        state.sid_priority.clone(),
        ["0", "6"].map(|s| Arc::new(s.to_string())).to_list(),
    );

let sql = sql_to_string(cs.to_insert_sql().expect("to_insert_sql failed"));
conn.execute(&sql, []).unwrap();
```

### Set Operations (Combined Reports)

```rust
let query_a = from(state.sid_todos.clone())
    .select([state.sid_title.clone()].to_list())
    .where_between(
        state.sid_priority.clone(),
        SqlPart::new(SqlInt32::new(4)),
        SqlPart::new(SqlInt32::new(5)),
    );

let query_b = from(state.sid_todos.clone())
    .select([state.sid_title.clone()].to_list())
    .where_not_null(state.sid_due_date.clone());

let union_frag     = union_sql(query_a.clone(), query_b.clone());
let intersect_frag = intersect_sql(query_a.clone(), query_b.clone());
let except_frag    = except_sql(query_a.clone(), query_b.clone());
```

### SqlBuilder with All Append Methods (Safe IDs Demo)

```rust
let b = SqlBuilder::new();
b.append_safe("SELECT * FROM todos WHERE id = ");
b.append_int32(42);
let frag = b.accumulated();

let b2 = SqlBuilder::new();
b2.append_safe("name = ");
b2.append_string("O'Brien");  // auto-escapes to 'O''Brien'
let frag2 = b2.accumulated();
```

## Security Model

The Alloy ORM provides five defense layers against SQL injection:

1. **SafeIdentifier** -- all table and column names are validated at construction via `safe_identifier()`; only `[a-zA-Z_][a-zA-Z0-9_]*` pass
2. **SqlBuilder** -- typed append methods (`append_int32`, `append_string`, `append_safe`) with automatic escaping for string literals (single-quote doubling)
3. **Changeset** -- `cast()` whitelists only permitted fields; user input never becomes raw SQL
4. **No string interpolation** -- the ORM API makes it structurally impossible to concatenate user input into SQL
5. **Type-safe values** -- `SqlInt32`, `SqlString`, `SqlFloat64` etc. enforce type-correct literal emission

For full details, see [SECURITY_ANALYSIS.md](SECURITY_ANALYSIS.md) and the main repo's [MITRE CWE analysis](https://github.com/notactuallytreyanastasio/alloy).

## Running the App

### Prerequisites

- Rust 1.70+ (with Cargo)
- No external SQLite needed (rusqlite bundles it)

### Build and Run

```bash
cargo run
```

The app starts at **http://localhost:5003**.

### Dependencies

| Crate | Purpose |
|---|---|
| `axum` 0.8 | HTTP server and routing |
| `askama` 0.13 | HTML templating (compile-time) |
| `rusqlite` 0.31 (bundled) | SQLite database driver |
| `tokio` 1.x | Async runtime |
| `serde` 1.x | Form deserialization |
| `tower-http` 0.6 | Static file serving |
| `orm` (vendored) | Alloy ORM -- compiled Temper-to-Rust crate |
| `temper-core` (vendored) | Temper runtime (Map, List, Arc types) |

## Links

- **Main Alloy ORM repo**: https://github.com/notactuallytreyanastasio/alloy
- **Compiled Rust library**: https://github.com/notactuallytreyanastasio/alloy-rust
