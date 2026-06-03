use crate::domain::types::*;
use crate::domain::user::*;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqlUserRepository {
    pool: SqlitePool,
}

impl SqlUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: UserId,
    username: String,
    created_at: Timestamp,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            username: row.username,
            created_at: row.created_at,
        }
    }
}

// ── Trait implementation ───────────────────────────────────

#[async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, user: User) -> RepoResult<User> {
        sqlx::query("INSERT INTO users (id, username, created_at) VALUES (?, ?, ?)")
            .bind(user.id)
            .bind(&user.username)
            .bind(user.created_at)
            .execute(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_user(&self, user_id: UserId) -> RepoResult<Option<User>> {
        let row =
            sqlx::query_as::<_, UserRow>("SELECT id, username, created_at FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(User::from))
    }

    async fn get_users(&self, ids: Vec<UserId>) -> RepoResult<Vec<User>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        // SQLite doesn't support array binds — build placeholders dynamically.
        let placeholders: String = std::iter::repeat("?")
            .take(ids.len())
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "SELECT id, username, created_at FROM users WHERE id IN ({})",
            placeholders
        );

        let mut query = sqlx::query_as::<_, UserRow>(&sql);
        for id in &ids {
            query = query.bind(id);
        }

        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(User::from).collect())
    }
}
