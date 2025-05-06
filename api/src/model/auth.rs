use garde::Validate;
use kernel::model::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    #[garde(skip)]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub password: String,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResponse {
    pub user_id: UserId,
    pub access_token: String,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEmailRequest {
    #[garde(email)]
    pub new_email: String,
    #[garde(email)]
    pub old_email: String,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    #[garde(skip)]
    pub new_password: String,
    #[garde(skip)]
    pub old_password: String,
}
