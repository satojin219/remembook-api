use async_trait::async_trait;
use derive_new::new;
use kernel::model::id::UserId;
use kernel::model::user::{
    event::{CreateUser, DeleteUser, UpdateUserPassword},
    User,
};
use kernel::repository::user::UserRepository;
use shared::error::{AppError, AppResult};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
       todo!()
    }

    async fn create(&self, event: CreateUser) -> AppResult<User> {
        let user_id = UserId::new();
        let hashed_password = hash_password(&event.password)?;

        let res = sqlx::query!(
            r#"
        INSERT INTO users(user_id,name,email,password_hash)
        VALUES($1,$2,$3,$4)
        "#,
            user_id as _,
            event.name,
            event.email,
            hashed_password
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No user has been created".into(),
            ));
        }
        Ok(User {
            id: user_id,
            name: event.name,
            email: event.email,
        })
    }

    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()> {
        todo!()
    }

    async fn delete(&self, event: DeleteUser) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
        DELETE FROM users WHERE user_id = $1
        "#,
            event.user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "Specified user does not exist".into(),
            ));
        }

        Ok(())
    }
}

fn hash_password(password: &str) -> AppResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(AppError::from)
}

fn verify_password(password: &str, hash: &str) -> AppResult<()> {
    let valid = bcrypt::verify(password, hash)?;
    if !valid {
        return Err(AppError::UnauthenticatedError);
    }
    Ok(())
}
