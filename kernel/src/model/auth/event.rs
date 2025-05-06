use crate::model::id::UserId;
use uuid::Uuid;

pub struct CreateToken {
    pub user_id: UserId,
    pub access_token: String,
}

impl CreateToken {
    pub fn new(user_id: UserId) -> Self {
        let access_token = Uuid::new_v4().simple().to_string();
        Self {
            user_id,
            access_token,
        }
    }
}

#[derive(Debug)]
pub struct ChangeEmail {
    pub user_id: UserId,
    pub new_email: String,
    pub old_email: String,
}

#[derive(Debug)]
pub struct ChangePassword {
    pub user_id: UserId,
    pub new_password: String,
    pub old_password: String,
}
