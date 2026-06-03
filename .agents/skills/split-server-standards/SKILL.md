---
name: split-server-standards
description: Repository standards, architecture guardrails, and business-flow context for the Rust GraphQL Splitwise clone backend in split-server. Use when Codex works in this repo on domain models, repositories, GraphQL resolvers/types/loaders, migrations, auth/current-user behavior, balances, expenses, payments, invites, or any change that needs the repo's clean architecture and product semantics rather than generic Rust advice.
---

# Split Server Standards

## Overview

Use this skill to stay aligned with `split-server`: a Rust 2024, Axum, async-graphql, SQLx/SQLite backend for a Splitwise-style expense sharing product. The important standards are partly encoded in file layout and database triggers, so read this before making architectural or business-logic changes.

For deeper context, read `references/repo-guide.md` when a task touches balances, expenses, payments, auth/RLS, repository wiring, migrations, or cross-layer design.

## Working Rules

- Preserve clean architecture boundaries:
  - `src/domain/` owns domain structs, type aliases, enums, and repository traits.
  - `src/repository/` owns SQLx/SQLite implementations and row mapping.
  - `src/graphql/` owns async-graphql schema, inputs, object types, loaders, and resolver adaptation.
  - `src/db/` owns pool/database setup.
- Add or change business concepts domain-first, then persistence, then GraphQL. Do not let GraphQL input shapes or SQL rows become the source of truth.
- Keep repository traits storage-agnostic. SQL strings, `sqlx::FromRow`, and SQLite-specific details belong in `src/repository/`, not in domain or GraphQL.
- Treat GraphQL as the public API layer. Parse `ID` values, map domain enums to stable GraphQL strings, and convert domain structs through `From<T>` wrappers in `src/graphql/types/`.
- Prefer `DynRepos` (`Arc<dyn RepositoryProvider>`) for resolver access. The full repository provider is not wired yet; respect the existing TODO instead of inventing a parallel dependency path.
- Do not assume auth exists. `me`, `create_group`, `add_expense`, `settle_payment`, and `invite_member` currently require future auth/current-user context. Third-party auth is intended but not implemented.
- Use integer minor units for money (`i64` amount values plus currency codes). Do not introduce floats for monetary values.
- Use `uuid::Uuid::now_v7()` for newly created domain IDs unless a surrounding pattern requires otherwise.
- Keep timestamps as Unix seconds (`i64`) to match migrations and existing domain aliases.
- After changing Rust code, run `cargo fmt` and `cargo check` when feasible.

## Business Flow

- Users create groups and become members through `group_members`.
- Groups contain expenses. An expense has one payer, a total amount, currency, split type, optional category, and rows in `expense_splits` describing what each participant owes.
- Pairwise balances live in `user_balances`: `from_user` owes `to_user` `amount` in `currency`.
- `group_member_net_balances` is materialized from the pairwise ledger by SQLite triggers. Do not recompute or manually patch net balances from application code unless redesigning the ledger intentionally.
- Payments are settlements. Inserting a `payments` row triggers `payment_settle_balance`, which writes the reverse pairwise ledger entry and lets ledger triggers update net balances.
- Expenses use soft deletion via `deleted_at`. Preserve that semantic when adding queries or mutations.
- Categories can be global defaults, user-created, group-scoped, and hierarchical.
- Invites can target an email or known user and move through pending/accepted/declined/expired.
- Activity logging exists as a domain concept/table but is not yet fully integrated.

## Before Coding Checklist

- Inspect the affected domain trait/struct before changing a repository or resolver.
- Inspect `001_init.sql` before changing money, balances, payments, invites, membership, categories, or auth/RLS assumptions.
- Keep API changes reflected across GraphQL input/output types and domain conversions.
- For balance-affecting changes, reason through both `user_balances` and trigger-maintained `group_member_net_balances`.
- If adding auth, introduce an explicit current-user context and then revisit GraphQL TODOs and SQLite `current_user`/views together.
