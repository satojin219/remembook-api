use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use kernel::model::auth::AccessToken;
use kernel::model::id::UserId;
use kernel::model::user::event::UpdateCoin;
use kernel::model::user::User;
use registry::AppRegistry;
use shared::error::AppError;

// リクエスト受信時のアクセストークンの検証処理
pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AppError;

    // handlerメソッドの引数にAuthorizedUserを追加したときはこのメソッドが呼ばれる
    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        // HTTPヘッダからアクセストークンを取り出す
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthorizedError)?;

        let access_token = AccessToken(bearer.token().to_string());

        // アクセストークンが紐づくユーザーIDを抽出
        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        // ユーザーIDでデータベースからユーザーのレコードを引く
        let user = registry
            .user_repository()
            .find_current_user(user_id)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        // logined_atが現在の日付より過去かどうかを確認
        let now = chrono::Utc::now();
        let days_diff = (now.date() - user.logined_at.date()).num_days();
        if days_diff >= 1 {
            registry
                .user_repository()
                .update_logined_at(user_id)
                .await?;
            registry
                .user_repository()
                .update_coin(UpdateCoin { user_id, amount: 1 })
                .await?;
        }
        Ok(Self { access_token, user })
    }
}
