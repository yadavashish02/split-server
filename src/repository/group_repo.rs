use crate::domain::group::*;
use crate::domain::types::*;
use crate::domain::user::UserId;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqlGroupRepository {
    pool: SqlitePool,
}

impl SqlGroupRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// ── Private row structs ────────────────────────────────────

#[derive(sqlx::FromRow)]
struct GroupRow {
    id: GroupId,
    name: String,
    created_by: UserId,
    created_at: Timestamp,
}

impl From<GroupRow> for Group {
    fn from(row: GroupRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            created_by: row.created_by,
            created_at: row.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GroupMemberRow {
    group_id: GroupId,
    user_id: UserId,
    role: String,
    joined_at: Timestamp,
}

impl TryFrom<GroupMemberRow> for GroupMember {
    type Error = anyhow::Error;

    fn try_from(row: GroupMemberRow) -> Result<Self, Self::Error> {
        let role = match row.role.as_str() {
            "admin" => GroupRole::Admin,
            "member" => GroupRole::Member,
            other => anyhow::bail!("unknown group role: {other}"),
        };
        Ok(Self {
            group_id: row.group_id,
            user_id: row.user_id,
            role,
            joined_at: row.joined_at,
        })
    }
}

/// Map a `GroupRole` to the lowercase string stored in SQLite.
fn role_to_str(role: &GroupRole) -> &'static str {
    match role {
        GroupRole::Admin => "admin",
        GroupRole::Member => "member",
    }
}

// ── Trait implementation ───────────────────────────────────

#[async_trait]
impl GroupRepository for SqlGroupRepository {
    async fn create_group(&self, group: Group) -> RepoResult<Group> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("INSERT INTO groups (id, name, created_by, created_at) VALUES (?, ?, ?, ?)")
            .bind(group.id)
            .bind(&group.name)
            .bind(group.created_by)
            .bind(group.created_at)
            .execute(&mut *tx)
            .await?;

        // Creator is automatically added as an admin member.
        sqlx::query(
            "INSERT INTO group_members (group_id, user_id, role, joined_at) VALUES (?, ?, 'admin', ?)",
        )
        .bind(group.id)
        .bind(group.created_by)
        .bind(group.created_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(group)
    }

    async fn get_group(&self, group_id: GroupId) -> RepoResult<Option<Group>> {
        let row = sqlx::query_as::<_, GroupRow>(
            "SELECT id, name, created_by, created_at FROM groups WHERE id = ?",
        )
        .bind(group_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Group::from))
    }

    async fn get_user_groups(
        &self,
        user_id: UserId,
        limit: i64,
        created_before: Option<Timestamp>,
    ) -> RepoResult<Vec<Group>> {
        let rows = match created_before {
            Some(before) => {
                sqlx::query_as::<_, GroupRow>(
                    "SELECT g.id, g.name, g.created_by, g.created_at \
                     FROM groups g \
                     JOIN group_members gm ON gm.group_id = g.id \
                     WHERE gm.user_id = ? AND g.created_at < ? \
                     ORDER BY g.created_at DESC \
                     LIMIT ?",
                )
                .bind(user_id)
                .bind(before)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, GroupRow>(
                    "SELECT g.id, g.name, g.created_by, g.created_at \
                     FROM groups g \
                     JOIN group_members gm ON gm.group_id = g.id \
                     WHERE gm.user_id = ? \
                     ORDER BY g.created_at DESC \
                     LIMIT ?",
                )
                .bind(user_id)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(rows.into_iter().map(Group::from).collect())
    }

    async fn add_member(
        &self,
        group_id: GroupId,
        user_id: UserId,
        role: GroupRole,
    ) -> RepoResult<()> {
        sqlx::query("INSERT INTO group_members (group_id, user_id, role) VALUES (?, ?, ?)")
            .bind(group_id)
            .bind(user_id)
            .bind(role_to_str(&role))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn remove_member(&self, group_id: GroupId, user_id: UserId) -> RepoResult<()> {
        sqlx::query("DELETE FROM group_members WHERE group_id = ? AND user_id = ?")
            .bind(group_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_members(
        &self,
        group_id: GroupId,
        limit: i64,
        joined_before: Option<Timestamp>,
    ) -> RepoResult<Vec<GroupMember>> {
        let rows = match joined_before {
            Some(before) => {
                sqlx::query_as::<_, GroupMemberRow>(
                    "SELECT group_id, user_id, role, joined_at \
                     FROM group_members \
                     WHERE group_id = ? AND joined_at < ? \
                     ORDER BY joined_at DESC \
                     LIMIT ?",
                )
                .bind(group_id)
                .bind(before)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, GroupMemberRow>(
                    "SELECT group_id, user_id, role, joined_at \
                     FROM group_members \
                     WHERE group_id = ? \
                     ORDER BY joined_at DESC \
                     LIMIT ?",
                )
                .bind(group_id)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
        };

        rows.into_iter()
            .map(GroupMember::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
}
