use crate::prelude::*;

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "UserKind")]
pub enum UserKind {
    Driver,
    Restaurant,
    Farm,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub kind: UserKind,
    pub created: PrimitiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub password: String,
    pub salt: String,
    pub kind: UserKind,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct PartialUser {
    pub email: String,
    pub kind: UserKind,
    pub password: String,
    pub salt: String,
}
