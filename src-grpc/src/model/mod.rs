use crate::prelude::*;

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "JobStatus")]
#[repr(i32)]
pub enum JobStatus {
    Pending = 0,
    PickedUp = 1,
    DroppedOff = 2,
    Confirmed = 3,
    Cancelled = 4,
}

impl From<JobStatus> for JobStatusResponse {
    fn from(status: JobStatus) -> Self {
        unsafe { std::mem::transmute(status) }
    }
}

impl From<JobStatusResponse> for JobStatus {
    fn from(status: JobStatusResponse) -> Self {
        unsafe { std::mem::transmute(status) }
    }
}
