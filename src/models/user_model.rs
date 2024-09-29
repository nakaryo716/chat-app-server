#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub user_mail: String,
    pub user_pass: String,
}

#[derive(Debug, Clone)]
pub struct PubUserInfo {
    pub user_id: String,
    pub user_name: String,
}
