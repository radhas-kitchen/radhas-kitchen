#![allow(unused_imports)]

tonic::include_proto!("proto");

pub use google::protobuf::*;
pub use rkapi::auth::{auth_server::*, *};
pub use rkapi::jobs::{jobs_server::*, *};
