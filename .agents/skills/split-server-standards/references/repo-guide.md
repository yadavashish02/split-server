# Split Server Repo Guide

## Repo Shape

`split-server` is a Rust backend for a Splitwise clone. It uses Axum for HTTP, async-graphql for the API, SQLx with SQLite for persistence, Tokio for async runtime, and `uuid` v7 IDs.

Main layers:

- `src/domain/`: pure domain contracts and data structures. Repository traits live here with `async_trait`.
- `src/repository/`: concrete SQLx repositories and database row mapping.
- `src/graphql/`: schema roots, GraphQL input/output types, resolvers, and DataLoader integration.
- `src/db/`: SQLite pool setup and pragmas.
- `migrations/`: relational schema, seed data, triggers, and simple RLS experiments.
- `docs/schema.puml`: entity relationship diagram and trigger notes.

## Architecture Standards

Prefer this implementation sequence for new features:

1. Define or adjust the domain model and repository trait in `src/domain/`.
2. Implement persistence in `src/repository/` with SQLx and private row structs.
3. Expose the behavior through `src/graphql/query.rs`, `src/graphql/mutation.rs`, and `src/graphql/types/`.
4. Wire shared access through `RepositoryProvider`/`DynRepos`.
5. Update migrations only when the relational model changes.

Domain code should not depend on GraphQL, SQLx, Axum, or SQLite-specific behavior. GraphQL resolvers should orchestrate, validate/parse API inputs, call repository traits, and map domain objects to GraphQL types. Repository implementations should own SQL, transactions, row-to-domain conversion, conflict handling, and SQLite quirks.

## Current Implementation State

The codebase is scaffolded for clean architecture but not complete:

- `SqlUserRepository` is implemented.
- `group_repo`, `expense_repo`, `balance_repo`, `payment_repo`, `invite_repo`, `activity_repo`, `category_repo`, and `currency_repo` are placeholders.
- `RepositoryProvider` is a trait alias-like composition of all repository traits, and `DynRepos` is `Arc<dyn RepositoryProvider>`.
- `main.rs` currently builds `build_schema_standalone()` and explicitly notes that concrete provider wiring is still TODO.
- GraphQL auth-dependent resolvers intentionally return "auth context required" errors.

When adding concrete repositories, avoid one-off resolver/database access. Build out provider wiring so query and mutation code can continue depending on domain traits.

## Product Model

The app models shared expenses inside groups:

- `users`: application users. The current schema only stores `username`; third-party auth is expected but not implemented.
- `groups`: shared spaces created by users.
- `group_members`: user membership and roles (`admin`, `member`).
- `categories`: global defaults plus optional user/group-specific hierarchical categories.
- `expenses`: group expense headers with payer, amount, currency, split type, category, timestamps, and soft delete.
- `expense_splits`: per-user amounts owed for an expense.
- `user_balances`: pairwise ledger rows. `from_user` owes `to_user` in a currency.
- `group_member_net_balances`: materialized per-user net balances maintained by triggers.
- `payments`: settlements from one user to another.
- `invites`: pending or resolved invitations by email or known user.
- `activity_log`: auditable user actions.

Money is stored as integer minor units (`amount INTEGER`, `i64`) plus `currency`; currencies include `minor_unit`. Do not use floating point for amounts.

## Balance and Settlement Semantics

`user_balances` is the source ledger for debt. A positive row means `from_user` owes `to_user`.

SQLite triggers maintain net balances:

- `ledger_insert_update_net` subtracts from the debtor and adds to the creditor.
- `ledger_update_update_net` applies the amount delta.
- `ledger_delete_update_net` reverses the old amount.
- `cleanup_zero_net` removes zero net balance rows after updates.

Payments settle balances through `payment_settle_balance`: inserting a payment creates or increases a reverse `user_balances` row from `paid_to` to `paid_by`. The ledger triggers then update net balances. Application code should not duplicate this payment-to-balance update unless the trigger strategy is being deliberately replaced.

When implementing expense creation or update, ensure expense rows, split rows, and pairwise ledger updates are transactionally consistent. For update/delete flows, account for the previous ledger effect before applying the new one.

## Auth and Current User

Auth is not wired yet. The GraphQL `me`, `create_group`, `add_expense`, `settle_payment`, and `invite_member` resolvers have TODOs because they need authenticated user context.

The migration includes a temporary `current_user` table and views (`v_groups`, `v_expenses`, `v_net_balances`) as a simple RLS experiment. If implementing auth or request scoping, design it explicitly:

- Decide how third-party auth identities map to `users`.
- Add a request-scoped current-user value to GraphQL context.
- Revisit the SQLite `current_user` table/views with connection pooling in mind; temp tables are per connection and can be surprising with pools.
- Do not fake auth by adding `user_id` fields to public mutations unless the task explicitly asks for temporary test scaffolding.

## GraphQL Conventions

Use async-graphql object wrappers like `UserType(pub User)` and `impl From<Domain> for Type`. Resolvers should expose stable scalar shapes:

- UUIDs become GraphQL `ID`.
- Domain enums become lowercase strings matching current API strings (`equal`, `exact`, `percentage`, `shares`).
- Optional strings use `Option<&str>` where borrowing from the wrapped domain object is natural.

Use DataLoader for repeated nested lookups where practical. `UserLoader` already shows the intended pattern, backed by `get_users`.

## SQLite and SQLx Notes

`db::init_pool` enables WAL, foreign keys, normal synchronous mode, memory temp store, a larger cache, and statement logging disabled. Keep foreign key and trigger assumptions intact.

SQLite does not support array binds; `SqlUserRepository::get_users` builds an `IN (?, ?, ...)` placeholder list and binds each UUID. Use the same cautious pattern for batched lookups.

Use private `FromRow` structs inside repository modules and convert into domain structs. Keep SQL column selection explicit.

## Validation

For ordinary code changes, run:

```bash
cargo fmt
cargo check
```

For migration or repository behavior changes, prefer adding focused tests or at least run the affected SQL path against SQLite when feasible.
