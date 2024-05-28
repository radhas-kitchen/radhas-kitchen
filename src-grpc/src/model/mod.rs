use crate::prelude::*;

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "UserKind")]
pub enum UserKind {
    Provider,
    Driver,
    Consumer,
}

impl From<UserKind> for UserKindResponse {
    fn from(kind: UserKind) -> Self {
        match kind {
            UserKind::Provider => UserKindResponse::Provider,
            UserKind::Driver => UserKindResponse::Driver,
            UserKind::Consumer => UserKindResponse::Consumer,
        }
    }
}

impl From<UserKindResponse> for UserKind {
    fn from(kind: UserKindResponse) -> Self {
        match kind {
            UserKindResponse::Provider => UserKind::Provider,
            UserKindResponse::Driver => UserKind::Driver,
            UserKindResponse::Consumer => UserKind::Consumer,
        }
    }
}

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "JobStatus")]
pub enum JobStatus {
    Created,
    Claimed,
    PickedUp,
    DroppedOff,
    Cancelled,
}

impl From<JobStatus> for JobStatusResponse {
    fn from(status: JobStatus) -> Self {
        match status {
            JobStatus::Created => JobStatusResponse::Created,
            JobStatus::Claimed => JobStatusResponse::Claimed,
            JobStatus::PickedUp => JobStatusResponse::PickedUp,
            JobStatus::DroppedOff => JobStatusResponse::DroppedOff,
            JobStatus::Cancelled => JobStatusResponse::Cancelled,
        }
    }
}

impl From<JobStatusResponse> for JobStatus {
    fn from(status: JobStatusResponse) -> Self {
        match status {
            JobStatusResponse::Created => JobStatus::Created,
            JobStatusResponse::Claimed => JobStatus::Claimed,
            JobStatusResponse::PickedUp => JobStatus::PickedUp,
            JobStatusResponse::DroppedOff => JobStatus::DroppedOff,
            JobStatusResponse::Cancelled => JobStatus::Cancelled,
        }
    }
}
