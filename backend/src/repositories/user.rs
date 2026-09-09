use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{CreateUser, User};

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &CreateUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                username,
                email,
                password_hash
            )
            VALUES ($1, $2, $3)
            RETURNING
                user_id,
                username,
                email,
                password_hash,
                created_at,
                last_seen_at
            "#,
            user.username,
            user.email,
            user.password_hash,
        )
        .fetch_one(self.pool)
        .await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self.pool)
        .await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await
    }

    async fn update(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query!("")
    }

    async fn delete(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }


}