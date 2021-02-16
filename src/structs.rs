#[derive(serde::Serialize, serde::Deserialize)]
pub struct SignupForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

// TODO add salt.
#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub hash: u64,
}