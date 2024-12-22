use crate::model::id::UserId;

#[derive(Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UpdateUserPassword {
    pub user_id: UserId,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug)]
pub struct DeleteUser {
    pub user_id: UserId,
}
