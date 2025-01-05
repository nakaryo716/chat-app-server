use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub user_mail: String,
    pub user_pass: String,
}
