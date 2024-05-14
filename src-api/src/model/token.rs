use crate::prelude::*;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub user_id: String,
    pub created: PrimitiveDateTime,
    pub expires: PrimitiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct PartialToken {
    pub token: String,
    pub user_id: String,
    pub expires: PrimitiveDateTime,
}
