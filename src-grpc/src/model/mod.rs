use crate::prelude::*;

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "UserKind")]
#[repr(i32)]
pub enum UserKind {
    Provider = 0,
    Driver = 1,
    Consumer = 2,
}

impl From<UserKind> for UserKindResponse {
    fn from(kind: UserKind) -> Self {
        unsafe { std::mem::transmute(kind) }
    }
}

impl From<UserKindResponse> for UserKind {
    fn from(kind: UserKindResponse) -> Self {
        unsafe { std::mem::transmute(kind) }
    }
}
