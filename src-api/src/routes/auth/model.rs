use crate::prelude::*;

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub user_id: String,
    pub old: String,
    pub new: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub token: String,
    #[serde(with = "serde_iso8601")]
    pub expires: PrimitiveDateTime,
    pub kind: UserKind,
}
