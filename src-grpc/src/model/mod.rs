use crate::prelude::*;

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "UserKind")]
pub enum UserKind {
    Driver,
    Restaurant,
    Farm,
}

impl From<UserKind> for UserKindResponse {
    fn from(kind: UserKind) -> Self {
        match kind {
            UserKind::Driver => UserKindResponse::Driver,
            UserKind::Restaurant => UserKindResponse::Provider,
            UserKind::Farm => UserKindResponse::Consumer,
        }
    }
}

impl From<UserKindResponse> for UserKind {
    fn from(kind: UserKindResponse) -> Self {
        match kind {
            UserKindResponse::Driver => UserKind::Driver,
            UserKindResponse::Provider => UserKind::Restaurant,
            UserKindResponse::Consumer => UserKind::Farm,
        }
    }
}
