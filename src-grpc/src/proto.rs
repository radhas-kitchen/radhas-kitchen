tonic::include_proto!("rkapi.jobs");
tonic::include_proto!("rkapi.auth");
tonic::include_proto!("google.protobuf");

pub use auth_server::*;
pub use jobs_server::*;
