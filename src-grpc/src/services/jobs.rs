use crate::prelude::*;
use rand::prelude::*;
use std::sync::Arc;

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
    let _ = sqlx::query!(
        r#"select * from tokens where user_id = $1 and token = $2 and expires > now()"#,
        &auth.user_id,
        &auth.token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch token: {e}");
        Status::internal("Failed to fetch token")
    })?
    .ok_or_else(|| {
        error!("Token not found for user {}", auth.user_id);
        Status::unauthenticated("Token not found")
    })?;

    Ok(())
}

#[tonic::async_trait]
impl Jobs for JobsService {
    type JobsStream = tokio_stream::Iter<<Vec<Result<Job, Status>> as IntoIterator>::IntoIter>;

    async fn post(&self, request: Request<Authorization>) -> Result<Response<Empty>, Status> {
        let auth = request.into_inner();
        do_auth(self.pool_ref(), &auth).await?;

        let Authorization {
            user_id: provider_id,
            ..
        } = auth;

        let consumers = sqlx::query!(r#"select id from consumers"#)
            .map(|row| row.id)
            .fetch_all(self.pool_ref())
            .await
            .map_err(|e| {
                error!("Failed to fetch consumers: {e}");
                Status::internal("Failed to fetch farms")
            })?;

        let consumer_id = consumers.choose(&mut thread_rng()).ok_or_else(|| {
            error!("No consumers found when creating job for {provider_id}");
            Status::not_found("No consumers found")
        })?;

        sqlx::query!(
            r#"insert into jobs (provider, consumer) values ($1,$2)"#,
            provider_id,
            consumer_id,
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to create job: {e}");
            Status::internal("Failed to create job")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn claim(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when claiming job {job_id}");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set status = 'Claimed', driver = $1 where id = $2 and status = 'Created'"#,
            user_id,
            job_id
        )
        .execute(self.pool_ref())
        .await
        .map_err(|e| {
            error!("Failed to update job: {e}");
            Status::internal("Failed to update job")
        })?;

        Ok(Response::new(Empty::default()))
    }

    async fn unclaim(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when unclaiming job {job_id}");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set status = 'Created', driver = null where id = $1 and driver = $2 and status = 'Claimed'"#,
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

    async fn pickup(&self, request: Request<JobUpdateRequest>) -> Result<Response<Empty>, Status> {
        let JobUpdateRequest { job_id, auth } = request.into_inner();
        let auth = auth.ok_or_else(|| {
            error!("No auth provided when picking up job {job_id}");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set status = 'PickedUp', pickedup = now() where id = $1 and driver = $2 and status = 'Claimed'"#,
            job_id,
            user_id
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
            error!("No auth provided when dropping off job {job_id}");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        sqlx::query!(
            r#"update jobs set status = 'DroppedOff', droppedoff = now() where id = $1 and driver = $2 and status = 'PickedUp'"#,
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
            error!("No auth provided when cancelling job {job_id}");
            Status::invalid_argument("No auth provided")
        })?;

        do_auth(self.pool_ref(), &auth).await?;

        let Authorization { user_id, .. } = auth;

        // TODO: Cancel Time
        sqlx::query!(
            r#"update jobs set status = 'Cancelled' where id = $1 and (status = 'Created' or status = 'Claimed') and provider = $2"#,
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

    async fn jobs(&self, _: Request<Empty>) -> Result<Response<Self::JobsStream>, Status> {
        let jobs = sqlx::query!(r#"select id,provider,consumer,driver,created,claimed,pickedup,droppedoff,cancelled,status as "status!: JobStatus" from jobs where (status != 'DroppedOff' or droppedoff >= now() - interval '2 days') and status != 'Cancelled'"#)
            .map(|row| Result::<_, Status>::Ok(Job {
                id: row.id,
                created: row.created.into_iso8601(),
                claimed: row.claimed.map(ToIso8601::into_iso8601),
                pickedup: row.pickedup.map(ToIso8601::into_iso8601),
                droppedoff: row.droppedoff.map(ToIso8601::into_iso8601),
                cancelled: row.cancelled.map(ToIso8601::into_iso8601),
                status: JobStatusResponse::from(row.status).into(),
                posted_by: row.provider,
                dropoff_to: row.consumer,
                claimed_by: row.driver,
            }))
            .fetch_all(self.pool_ref())
            .await
            .map_err(|e| {
                error!("Failed to fetch jobs: {e}");
                Status::internal("Failed to fetch jobs")
            })?;

        Ok(Response::new(tokio_stream::iter(jobs)))
    }

    async fn get(&self, request: Request<JobId>) -> Result<Response<Job>, Status> {
        let JobId { job_id } = request.into_inner();

        let job = sqlx::query!(r#"select id,provider,consumer,driver,created,claimed,pickedup,droppedoff,cancelled,status as "status!: JobStatus" from jobs where id = $1"#, job_id)
            .map(|row| Job {
                id: row.id,
                created: row.created.into_iso8601(),
                claimed: row.claimed.map(ToIso8601::into_iso8601),
                pickedup: row.pickedup.map(ToIso8601::into_iso8601),
                droppedoff: row.droppedoff.map(ToIso8601::into_iso8601),
                cancelled: row.cancelled.map(ToIso8601::into_iso8601),
                status: JobStatusResponse::from(row.status).into(),
                posted_by: row.provider,
                dropoff_to: row.consumer,
                claimed_by: row.driver,
            })
            .fetch_optional(self.pool_ref())
            .await
            .map_err(|e| {
                error!("Failed to fetch job: {e}");
                Status::internal("Failed to fetch job")
            })?
            .ok_or_else(|| {
                error!("Job {job_id} not found");
                Status::not_found("Job not found")
            })?;

        Ok(Response::new(job))
    }
}
