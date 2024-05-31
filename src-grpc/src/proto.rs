#![allow(unused_imports)]

tonic::include_proto!("proto");

pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("proto_descriptor");

pub use google::protobuf::*;
pub use grpc::health::v1::{health_check_response::*, health_server::*, *};
pub use rkapi::auth::{auth_server::*, create_user_request::*, *};
pub use rkapi::jobs::{jobs_server::*, *};
