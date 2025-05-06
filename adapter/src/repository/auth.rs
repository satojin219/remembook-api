use std::sync::Arc;

use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        auth::{
            event::{ChangeEmail, ChangePassword, CreateToken},
            AccessToken,
        },
        id::UserId,
    },
    repository::auth::AuthRepository,
};

use shared::error::{AppError, AppResult};

use crate::{
    database::{
        model::auth::{from, AuthorizationKey, AuthorizedUserId, UserItem},
        ConnectionPool,
    },
    redis::RedisClient,
};

#[derive(new)]
pub struct AuthRepositoryImpl {
    db: ConnectionPool,
    kv: Arc<RedisClient>,
    ttl: u64,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AppResult<Option<UserId>> {
        let key: AuthorizationKey = access_token.into();
        self.kv
            .get(&key)
            .await
            .map(|x| x.map(AuthorizedUserId::into_inner))
    }

    async fn verify_user(&self, email: &str, password: &str) -> AppResult<UserId> {
        let user_item = sqlx::query_as!(
            UserItem,
            r#"SELECT user_id,password_hash,email FROM users WHERE email = $1;"#,
            email
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let valid = bcrypt::verify(password, &user_item.password_hash)?;

        if !valid {
            return Err(AppError::UnauthenticatedError);
        }
        Ok(user_item.user_id)
    }

    async fn register_user(&self, name: &str, email: &str, password: &str) -> AppResult<UserId> {
        let user_id = UserId::new();
        let hashed_password = hash_password(&password)?;

        sqlx::query!(
            r#"
        INSERT INTO users(user_id,name,email,password_hash)
        VALUES($1,$2,$3,$4)
        "#,
            user_id as _,
            name,
            email,
            hashed_password
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(user_id)
    }

    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken> {
        let (key, value) = from(event);
        self.kv.set_ex(&key, &value, self.ttl).await?;
        Ok(key.into())
    }

    async fn delete_token(&self, access_token: &AccessToken) -> AppResult<()> {
        let key: AuthorizationKey = access_token.into();
        self.kv.delete(&key).await
    }

    async fn change_email(&self, event: ChangeEmail) -> AppResult<()> {
        let user_item = sqlx::query_as!(
            UserItem,
            r#"SELECT user_id,email,password_hash FROM users WHERE user_id = $1;"#,
            event.user_id as _
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let valid = event.old_email == user_item.email;

        if !valid {
            return Err(AppError::UnauthenticatedError);
        }

        sqlx::query!(
            r#"
            UPDATE users
            SET email =  $1
            WHERE user_id = $2
        "#,
            event.new_email,
            event.user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(())
    }

    async fn change_password(&self, event: ChangePassword) -> AppResult<()> {
        let user_item = sqlx::query_as!(
            UserItem,
            r#"SELECT user_id,password_hash,email FROM users WHERE user_id = $1;"#,
            event.user_id as _
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let valid = bcrypt::verify(event.old_password, &user_item.password_hash)?;
        if !valid {
            return Err(AppError::UnauthenticatedError);
        }
        let hashed_password = hash_password(&event.new_password)?;

        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash =  $1
            WHERE user_id = $2
        "#,
            hashed_password,
            event.user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(())
    }
}

fn hash_password(password: &str) -> AppResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(AppError::from)
}
