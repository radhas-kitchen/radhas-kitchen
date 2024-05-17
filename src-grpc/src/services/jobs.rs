use crate::prelude::*;
use rand::prelude::*;
use std::{pin::Pin, sync::Arc};
use tokio_stream::Stream;

#[derive(Debug)]
pub struct JobsService {
    pool: Arc<Pool<Postgres>>,
}

impl JobsService {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    fn pool_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

async fn do_auth(pool: &Pool<Postgres>, auth: &Authorization) -> Result<(), Status> {
    let expiry = sqlx::query!(
        r#"select expires from tokens where user_id = $1 and token = $2"#,
        &auth.user_id,
        &auth.token
    )
    .map(|row| row.expires)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch token: {e}");
        Status::internal("Failed to fetch token")
    })?;

    let Some(expiry) = expiry else {
        error!("Token not found");
        return Err(Status::unauthenticated("Token not found"));
    };

    if OffsetDateTime::new_utc(expiry.date(), expiry.time()) < OffsetDateTime::now_utc() {
        error!("Token expired");
        return Err(Status::unauthenticated("Token expired"));
    }

    Ok(())
}

#[tonic::async_trait]
impl Jobs for JobsService {
    type JobsStream = Pin<Box<dyn Stream<Item = Result<Job, Status>> + Send + Sync + 'static>>;

    async fn post(&self, request: Request<Authorization>) -> Result<Response<Empty>, Status> {
        let auth = request.into_inner();
        do_auth(self.pool_ref(), &auth).await?;

        let Authorization {
            user_id: provider_id,
            ..
        } = auth;

        let pickup = sqlx::query!(
            r#"select location from providers where id = $1"#,
            provider_id
        )
        .map(|row| row.location)
        .fetch_optional(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to fetch provider: {e}");
            Status::internal("Failed to fetch resteraunt data")
        })?;

        let Some(pickup) = pickup else {
            error!("User not found or is not a provider");
            return Err(Status::not_found("User not found or is not a resteraunt"));
        };

        let consumers = sqlx::query!(r#"select id, location from consumers"#)
            .map(|row| (row.id, row.location))
            .fetch_all(self.pool_ref())
            .await
            .map_err(|e| {
                error!("Failed to fetch consumers: {e}");
                Status::internal("Failed to fetch farms")
            })?;

        let (consumer_id, dropoff) = match consumers.choose(&mut thread_rng()) {
            Some(consumer) => consumer,
            None => {
                error!("No consumers found");
                return Err(Status::not_found("No farms found"));
            }
        };

        sqlx::query!(
            r#"insert into jobs (provider, consumer, pickup_location, dropoff_location) values ($1,$2,$3,$4)"#,
            provider_id,
            consumer_id,
            pickup,
            dropoff
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to create job: {e}");
            Status::internal("Failed to create job")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn pickup(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when updating job (pickup)");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set driver = $1, status = 'PickedUp', pickup_time = now() where id = $2"#,
            user_id,
            job_id
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to update job: {e}");
            Status::internal("Failed to update job.")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn dropoff(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when updating job (dropoff)");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set status = 'DroppedOff', dropoff_time = now() where id = $1 and driver = $2"#,
            job_id,
            user_id
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to update job: {e}");
            Status::internal("Failed to update job")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn confirm(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when updating job (confirm)");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set status = 'Confirmed', confirm_time = now() where id = $1 and consumer = $2"#,
            job_id,
            user_id
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to update job: {e}");
            Status::internal("Failed to update job")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn cancel(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when updating job (cancel)");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        // TODO: Cancel Time
        sqlx::query!(
            r#"update jobs set status = 'Cancelled' where id = $1 and status = 'Pending' and provider = $2"#,
            job_id,
            user_id
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to update job: {e}");
            Status::internal("Failed to update job")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn jobs(&self, request: Request<Empty>) -> Result<Response<Self::JobsStream>, Status> {
        unimplemented!()
    }

    async fn get(&self, request: Request<JobId>) -> Result<Response<Job>, Status> {
        unimplemented!()
    }
}
