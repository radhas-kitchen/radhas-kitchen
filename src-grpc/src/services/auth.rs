use time::OffsetDateTime;

use crate::prelude::*;

#[derive(Debug)]
pub struct AuthService {
    pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();
        let user = sqlx::query!(
            r#"select id, password, salt, kind as "kind!: UserKind" from users where email = $1"#,
            request.email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            error!("Failed to fetch user: {}", err);
            Status::internal("Failed to fetch user")
        })?;

        let Some(user) = user else {
            return Err(Status::not_found("User not found"));
        };

        let password = sha256::digest(format!("{}{}", request.password, user.salt));

        if password != user.password {
            return Err(Status::unauthenticated("Invalid password"));
        }

        let tokens = sqlx::query!(
            r#"select token, expires from tokens where user_id = $1 and expires > now()"#,
            user.id
        )
        .map(|row| (row.token, row.expires))
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            error!("Failed to fetch tokens: {}", err);
            Status::internal("Failed to fetch tokens")
        })?;

        let (token, expires) = if let Some(token) = tokens {
            token
        } else {
            sqlx::query!(
                r#"insert into tokens (user_id) values ($1) returning token, expires"#,
                user.id,
            )
            .map(|row| (row.token, row.expires))
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                error!("Failed to create token: {}", err);
                Status::internal("Failed to create token")
            })?
        };

        log::info!("User {} logged in", user.id);

        Ok(Response::new(LoginResponse {
            token,
            expires: OffsetDateTime::new_utc(expires.date(), expires.time()).to_string(),
            user_id: user.id,
            kind: UserKindResponse::from(user.kind).into(),
        }))
    }

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Empty, Status> {
        unimplemented!()
    }

    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Empty, Status> {
        unimplemented!()
    }
}
