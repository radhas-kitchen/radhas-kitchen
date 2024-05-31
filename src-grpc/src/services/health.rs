use crate::prelude::*;
use std::iter::{self, FromFn};

#[derive(Debug)]
pub struct HealthService;

type StreamF = fn() -> Option<Result<HealthCheckResponse, Status>>;
const F: StreamF = || {
    Some(Ok(HealthCheckResponse {
        status: ServingStatus::Serving.into(),
    }))
};

#[tonic::async_trait]
impl Health for HealthService {
    async fn check(
        &self,
        _: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let response = HealthCheckResponse {
            status: ServingStatus::Serving.into(),
        };

        Ok(Response::new(response))
    }

    type WatchStream = tokio_stream::Iter<FromFn<StreamF>>;

    async fn watch(
        &self,
        _: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        Ok(Response::new(tokio_stream::iter(iter::from_fn(F))))
    }
}
