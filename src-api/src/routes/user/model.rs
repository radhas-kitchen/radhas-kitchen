use crate::prelude::*;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub kind: UserKind,
    pub name: String,
}
