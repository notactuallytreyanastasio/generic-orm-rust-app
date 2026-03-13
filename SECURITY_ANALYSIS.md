# SQL Security Analysis: Rust Todo App

SQL injection analysis of the Rust todo app built on the Generic Temper ORM. This analysis focuses exclusively on SQL generation and execution — the core value proposition of the ORM.

**Analysis Date:** 2026-03-12
**Updated:** 2026-03-13
**Framework:** Axum + rusqlite + askama
**ORM:** Generic Temper ORM (compiled to Rust)

---

## How This App Uses the ORM

All user-facing CRUD operations flow through the Temper ORM's type-safe SQL generation:

| Operation | Method | SQL Source |
|-----------|--------|------------|
| SELECT lists/todos | `from(safe_identifier("...")?).where(...).to_sql()` | ORM |
| INSERT list/todo | `changeset(table, params).cast(fields).validate_required(fields).to_insert_sql()` | ORM |
| UPDATE todo title | `changeset(table, params).cast(fields).to_update_sql(id)` | ORM |
| UPDATE completed | `changeset(table, params).cast(fields).to_update_sql(id)` | ORM |
| DELETE list/todo | `delete_sql(table, id)` | ORM |
| WHERE clauses | `SqlBuilder::append_safe()` + `append_int32()` | ORM |
| Cascade delete | `DELETE FROM todos WHERE list_id = ?1` | Raw (parameterized) |
| JOIN + aggregate | `SELECT l.*, COUNT(t.id) ... LEFT JOIN ...` | Raw (hardcoded — no user input) |
| DDL | `CREATE TABLE IF NOT EXISTS ...` | Raw (static) |

### ORM SQL Generation Path

```
User input (form field)
  -> Axum Form<T> deserialization
    -> Temper Map construction
      -> changeset(table_def, params_map)         [factory -- sealed interface]
        -> .cast(allowed_fields)                   [SafeIdentifier whitelist]
          -> .validate_required(fields)            [non-null enforcement]
            -> .to_insert_sql()?                   [type-dispatched escaping]
              -> .to_string()                      [rendered SQL string]
                -> conn.execute(&sql, [])           [rusqlite execution]
```

For queries:
```
Route parameter (Path<i64>)
  -> Axum typed extraction (guaranteed i64)
    -> id as i32                                   [truncation -- see RS-SQL-1]
      -> SqlBuilder::append_int32(id)              [bare integer]
        -> from(safe_identifier("lists")?).where(fragment).to_sql()
          -> conn.prepare(&sql)?.query_map([], ...)
```

---

## SQL Injection Analysis

### ORM-Generated SQL: Protected

**SafeIdentifier validation** -- Rust's `Result` type makes `safe_identifier()` returns explicit -- the `?` operator propagates failures. All identifiers validated against `[a-zA-Z_][a-zA-Z0-9_]*`.

**SqlString escaping** -- String values pass through `SqlString` which escapes `'` to `''`.

**Changeset field whitelisting** -- `cast()` restricts columns in INSERT/UPDATE. This app uses the ORM for toggle too (via `changeset().to_update_sql()`), unlike most other apps which use raw SQL for toggle.

**Type safety** -- Rust's ownership model ensures `SqlBuilder` parts cannot be modified after construction. The `SqlFragment` is consumed once and rendered.

### Raw SQL: Mixed

```rust
// Cascade delete -- parameterized with ?1
conn.execute("DELETE FROM todos WHERE list_id = ?1", params![list_id])?;

// JOIN aggregate -- hardcoded, no user input
conn.prepare("SELECT l.id, l.name, COUNT(t.id) ... FROM lists l LEFT JOIN todos t ...")?;

// Seed data -- hardcoded literals
conn.execute("INSERT INTO todos (title, list_id) VALUES (?1, ?2)", params![title, id])?;
```

The JOIN query has no user input -- it returns all lists with counts. The cascade delete uses rusqlite's `params!` macro for safe binding.

### DDL: Static

Schema creation uses hardcoded `CREATE TABLE` statements.

---

## SQL Findings

| # | Severity | CWE | Finding |
|---|----------|-----|---------|
| RS-SQL-1 | LOW | CWE-681 | `Path<i64>` route params are cast to `i32` via `id as i32` for ORM calls. IDs above `i32::MAX` (2,147,483,647) silently wrap, potentially targeting a different record. SQLite uses `i64` rowids, so the ORM's `Int32` type creates a mismatch. Not exploitable for injection, but could cause incorrect deletes/updates. |
| RS-SQL-2 | INFO | CWE-89 | ORM SQL is executed via `conn.execute(&sql, [])` with an empty parameter array -- the SQL is pre-rendered with escaped values. Correct, but parameterized would be strictly safer. |
| RS-SQL-3 | INFO | CWE-400 | SELECT queries use `to_sql()` without limits. The ORM's `safe_to_sql(default_limit)` is available but unused. |

### ORM-Level Concerns (shared across all apps)

| # | Severity | CWE | Finding |
|---|----------|-----|---------|
| ORM-1 | MEDIUM | CWE-89 | `to_insert_sql`/`to_update_sql` pass `pair.key` (a `String`) to `append_safe`. Safe by construction but not type-enforced at the call site. |
| ORM-2 | LOW | CWE-89 | `SqlDate.format_to` wraps `value.to_string()` in quotes without escaping. |
| ORM-3 | LOW | CWE-20 | `SqlFloat64.format_to` can produce `NaN`/`Infinity`. |

---

## Verdict

**No SQL injection vulnerabilities found.** The ORM covers all user-input-to-SQL paths. This is the only app that uses the ORM for toggle-completed (rather than raw SQL), giving it the highest ORM coverage. The `i64` to `i32` truncation is a correctness concern, not an injection vector.

---

## Evolution: Temper-Level Remediation

**Date:** 2026-03-12
**Commit:** [`1df8c7a`](https://github.com/notactuallytreyanastasio/generic_orm/commit/1df8c7a)

The security analysis above identified 3 ORM-level concerns (ORM-1, ORM-2, ORM-3) shared across all 6 app implementations. Because the ORM is written once in Temper and compiled to all backends, fixing these issues at the Temper source level automatically resolves them in every language -- including this Rust app.

### What Changed

**ORM-1 (MEDIUM -> RESOLVED): Column name type safety in INSERT/UPDATE SQL**

The `to_insert_sql()` and `to_update_sql()` methods previously passed `pair.key` (a raw `String`) to `append_safe()`. While safe by construction (keys originated from `cast()` via `SafeIdentifier::sql_value()`), the type system didn't enforce this. A future refactor could have silently introduced an unvalidated code path.

The fix routes column names through the looked-up `FieldDef.name.sql_value()` -- a `SafeIdentifier` -- so the column name in the generated SQL always comes from a validated identifier, not a raw map key.

**ORM-2 (LOW -> RESOLVED): SqlDate quote escaping**

`SqlDate::format_to()` previously wrapped `value.to_string()` in quotes without escaping. The fix adds character-by-character quote escaping identical to `SqlString::format_to()`, ensuring defense-in-depth against any future Date format that might contain single quotes.

**ORM-3 (LOW -> RESOLVED): SqlFloat64 NaN/Infinity handling**

`SqlFloat64::format_to()` previously called `value.to_string()` directly, which could produce `NaN`, `inf`, or `-inf` -- none valid SQL literals. The fix checks for these values and renders `NULL` instead, which is the safest SQL representation for non-representable floating-point values.

### Why This Matters

This is the core value proposition of a cross-compiled ORM: **one fix in Temper source propagates to all 6 backends simultaneously.** The same commit that fixed the Rust compiled output also fixed JavaScript, Python, Java, Lua, and C#. No per-language patches needed. No risk of inconsistent fixes across implementations.

### Updated Status

| Finding | Original | Current | Resolution |
|---------|----------|---------|------------|
| ORM-1 | MEDIUM | RESOLVED | Column names routed through `SafeIdentifier` |
| ORM-2 | LOW | RESOLVED | `SqlDate::format_to()` now escapes quotes |
| ORM-3 | LOW | RESOLVED | `SqlFloat64::format_to()` renders NaN/Infinity as `NULL` |
| ORM-4 | INFO | ACKNOWLEDGED | Design limitation -- escaping-based, not parameterized |

---

## JOIN Feature Security Analysis

**Date:** 2026-03-13
**Feature:** JOIN support (INNER, LEFT, RIGHT, FULL OUTER) added to the Temper ORM's Query builder.

### Architecture

The ORM's JOIN implementation follows the same security model as the rest of the query builder:

```
Query.innerJoin(table: SafeIdentifier, onCondition: SqlFragment) -> Query
Query.leftJoin(table: SafeIdentifier, onCondition: SqlFragment) -> Query
Query.rightJoin(table: SafeIdentifier, onCondition: SqlFragment) -> Query
Query.fullJoin(table: SafeIdentifier, onCondition: SqlFragment) -> Query
```

The generic `join()` method takes three arguments:
1. `joinType: JoinType` -- a **sealed interface** with exactly 4 implementations (`InnerJoin`, `LeftJoin`, `RightJoin`, `FullJoin`). Each returns a hardcoded keyword string (`"INNER JOIN"`, `"LEFT JOIN"`, `"RIGHT JOIN"`, `"FULL OUTER JOIN"`).
2. `table: SafeIdentifier` -- the joined table name, validated against `[a-zA-Z_][a-zA-Z0-9_]*`.
3. `onCondition: SqlFragment` -- the ON clause, which must be constructed via `SqlBuilder` (type-safe escaping) or the `sql` tagged template literal.

### SQL Generation

The `toSql()` method renders JOINs between FROM and WHERE:

```
SELECT [fields] FROM [table]
  [JOIN_TYPE] [join_table] ON [on_condition]
  [JOIN_TYPE] [join_table] ON [on_condition]
  WHERE [conditions]
  ORDER BY [clauses]
  LIMIT [n] OFFSET [m]
```

Key security properties:
- **JOIN keyword:** Comes from `jc.joinType.keyword()` which is a sealed enum returning only the four valid SQL JOIN keywords. No user string can reach this path.
- **Table name:** `jc.table.sqlValue` -- passes through `SafeIdentifier` validation. Rejects spaces, semicolons, quotes, and all non-identifier characters.
- **ON condition:** `jc.onCondition` -- a `SqlFragment` appended via `appendFragment()`. The fragment's parts are individually typed (`SqlSource`, `SqlString`, `SqlInt32`, etc.) with per-type escaping. User values embedded in ON conditions are escaped by the same mechanisms as WHERE conditions.

### col() Helper

A new `col(table, column)` helper produces qualified column references:

```
col(sid("users"), sid("id"))  ->  "users.id"
```

Both arguments require `SafeIdentifier`, so the result is always safe to use in ON conditions.

### Threat Analysis

| Threat | Status | Rationale |
|--------|--------|-----------|
| SQL injection via JOIN table name | **Mitigated** | `SafeIdentifier` rejects all non-`[a-zA-Z_][a-zA-Z0-9_]*` input. Test case confirms: `safe_identifier("users; DROP TABLE users; --")` returns `Err`. |
| SQL injection via ON condition | **Mitigated** | ON conditions are `SqlFragment` values assembled via `SqlBuilder`. String interpolation uses `SqlString` (quote escaping). Integer interpolation uses `SqlInt32`/`SqlInt64` (bare numeric rendering). No raw string concatenation path exists. |
| JOIN type confusion | **Mitigated** | `JoinType` is a sealed interface. Only the 4 built-in implementations exist. No user-provided join type string is possible. |
| Unbounded JOIN result sets | **Partially Mitigated** | JOINs can produce large result sets (Cartesian products in worst case). The `safe_to_sql(defaultLimit)` method would apply a LIMIT, but the app uses `to_sql()` without limits. Same concern as RS-SQL-3. |
| Information disclosure via JOIN | **N/A in current app** | The app does not yet use the ORM's JOIN feature (the one JOIN query in `index` is raw SQL). When adopted, JOIN queries could expose data from related tables. This is a data modeling concern, not an injection concern. |

### Current App Usage

The Rust todo app currently uses a **raw hardcoded SQL JOIN** in the `index` handler:

```rust
"SELECT l.id, l.name, COUNT(t.id) as todo_count
 FROM lists l
 LEFT JOIN todos t ON t.list_id = l.id
 GROUP BY l.id
 ORDER BY l.created_at DESC"
```

This is safe because it contains no user input. If the app were updated to use the ORM's new JOIN API, it would look like:

```rust
let on_cond = {
    let b = SqlBuilder::new();
    b.append_safe("t.list_id = l.id");
    b.accumulated()
};
let q = from(state.list_table.table_name())
    .left_join(state.todo_table.table_name(), on_cond)
    .to_sql();
```

This would provide the same safety guarantees with the added benefit of validated table names.

### JOIN Feature Findings

| # | Severity | CWE | Finding |
|---|----------|-----|---------|
| JOIN-1 | INFO | CWE-400 | JOIN queries can produce large Cartesian products. The ORM provides `safe_to_sql(defaultLimit)` but does not enforce its use. Applications should use `safe_to_sql()` for any JOIN query in production. |
| JOIN-2 | INFO | -- | The `col()` helper constructs qualified references via `SafeIdentifier` + `appendSafe(".")`. The dot is hardcoded, preventing injection of additional SQL syntax through the separator. |
| JOIN-3 | INFO | -- | Multiple JOINs are supported via chaining (each call returns a new immutable `Query`). Join order is preserved, which matters for LEFT/RIGHT JOIN semantics but has no security implications. |

### Recommendations

1. **Migrate the raw JOIN query to the ORM.** The `index` handler's raw SQL JOIN should be replaced with the ORM's `leftJoin()` API once GROUP BY / aggregate support is added to the ORM. This would eliminate the last significant raw SQL statement in the app.

2. **Use `safe_to_sql()` for JOIN queries.** JOIN queries can produce large result sets. Always apply a default limit via `safe_to_sql(100)` or similar.

3. **Build ON conditions with `col()` helper.** Use `col(sid("users"), sid("id"))` rather than `append_safe("users.id")` to get compile-time safety on both table and column names in ON conditions.

---

## SQL-Related CWE Mapping

Relevant Common Weakness Enumeration (CWE) classifications for SQL security in this application:

| CWE | Name | Status | Analysis |
|-----|------|--------|----------|
| CWE-89 | SQL Injection | **Mitigated** | All user-input-to-SQL paths go through the ORM's type-safe SQL generation: `SafeIdentifier` validates table/column names against `[a-zA-Z_][a-zA-Z0-9_]*`; `SqlString` escapes `'` to `''`; `SqlBuilder::append_int32/64` renders bare integers; `changeset().cast()` whitelists allowed fields. The one raw SQL statement with user data (`DELETE FROM todos WHERE list_id = ?1`) uses rusqlite parameterized binding. The JOIN aggregate query is entirely hardcoded. |
| CWE-20 | Improper Input Validation | **Partially Mitigated** | The ORM's `validate_required()` checks for non-empty fields and `SafeIdentifier` validates all SQL identifiers. However, no length limits on `name` or `title` fields and the `i64` to `i32` truncation in `id as i32` is unchecked (RS-SQL-1). |
| CWE-190 | Integer Overflow or Wraparound | **Partially Mitigated** | Rust's debug-mode integer arithmetic panics on overflow. In release mode, `id as i32` performs a wrapping truncation of `i64` values -- IDs above `i32::MAX` silently wrap to negative values, potentially targeting a different database record (RS-SQL-1). |
| CWE-400 | Uncontrolled Resource Consumption | **Partially Mitigated** | SELECT queries use `to_sql()` without limits. The ORM's `safe_to_sql(default_limit)` is available but unused (RS-SQL-3). JOIN queries can produce large Cartesian products (JOIN-1). |
| CWE-915 | Improperly Controlled Modification of Dynamically-Determined Object Attributes | **Mitigated** | The `changeset().cast()` method explicitly whitelists allowed fields for INSERT/UPDATE operations. No mass-assignment vulnerability exists. |

---

## Combined Finding Summary

**Updated: 2026-03-13**

### Application-Level SQL Findings

| # | Severity | CWE | Status | Finding |
|---|----------|-----|--------|---------|
| RS-SQL-1 | LOW | CWE-681/190 | OPEN | `i64` to `i32` truncation on route params passed to ORM |
| RS-SQL-2 | INFO | CWE-89 | OPEN | ORM SQL executed with empty params (escaping-based, not parameterized) |
| RS-SQL-3 | INFO | CWE-400 | OPEN | SELECT queries use `to_sql()` without limits |

### ORM-Level Findings (Resolved)

| # | Severity | CWE | Status | Finding |
|---|----------|-----|--------|---------|
| ORM-1 | MEDIUM | CWE-89 | RESOLVED | Column names now routed through `SafeIdentifier` |
| ORM-2 | LOW | CWE-89 | RESOLVED | `SqlDate::format_to()` now escapes quotes |
| ORM-3 | LOW | CWE-20 | RESOLVED | `SqlFloat64::format_to()` renders NaN/Infinity as `NULL` |
| ORM-4 | INFO | -- | ACKNOWLEDGED | Design limitation: escaping-based, not parameterized |

### JOIN Feature Findings

| # | Severity | CWE | Status | Finding |
|---|----------|-----|--------|---------|
| JOIN-1 | INFO | CWE-400 | OPEN | JOIN queries can produce unbounded result sets |
| JOIN-2 | INFO | -- | OK | `col()` helper uses hardcoded dot separator |
| JOIN-3 | INFO | -- | OK | Chained JOINs preserve order, immutable Query pattern |

### SQL Security Totals

| Severity | Open | Resolved | Total |
|----------|------|----------|-------|
| MEDIUM | 0 | 1 (ORM-1) | 1 |
| LOW | 1 (RS-SQL-1) | 2 (ORM-2, ORM-3) | 3 |
| INFO | 4 (RS-SQL-2, RS-SQL-3, JOIN-1, ORM-4) | 0 | 4 |
| **Total** | **5** | **3** | **8** |
