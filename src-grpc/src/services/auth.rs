use crate::prelude::*;
use std::{borrow::Cow, sync::Arc};

#[derive(Debug)]
pub struct AuthService {
    pool: Arc<Pool<Postgres>>,
}

impl AuthService {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    fn pool_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
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
        .fetch_optional(self.pool_ref())
        .await
        .map_err(|err| {
            error!("Failed to fetch user: {}", err);
            Status::internal("Failed to fetch user")
        })?;

        let Some(user) = user else {
            error!("User not found");
            return Err(Status::not_found("User not found"));
        };

        let password = sha256::digest(format!("{}{}", request.password, user.salt));

        if password != user.password {
            error!("Invalid password for user {}", user.id);
            return Err(Status::unauthenticated("Invalid password"));
        }

        let tokens = sqlx::query!(
            r#"select token, expires from tokens where user_id = $1 and expires > now()"#,
            user.id
        )
        .map(|row| (row.token, row.expires))
        .fetch_optional(self.pool_ref())
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
            .fetch_one(self.pool_ref())
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

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<Empty>, Status> {
        let CreateUserRequest {
            email,
            password,
            name,
            kind: kind_data,
        } = request.into_inner();

        let kind = match &kind_data {
            Some(Kind::Consumer(_)) => UserKind::Consumer,
            Some(Kind::Provider(_)) => UserKind::Provider,
            None => UserKind::Driver,
        };

        let (user, salt) = sqlx::query!(
            r#"insert into users (email, password, name, kind) values ($1, $2, $3, ($4::text)::userkind) returning id, salt"#,
            email,
            "__placeholder",
            name,
            kind as UserKind
        )
        .map(|row| (row.id, row.salt))
        .fetch_one(self.pool_ref())
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::Database(edb) if edb.code() == Some(Cow::Borrowed("23505")) => {
                    error!("User already exists");
                    Status::already_exists("User already exists")
                }
                _ => {
                    error!("Failed to create user: {}", err);
                    Status::internal("Failed to create user")
                }
            }
        })?;

        let password = sha256::digest(format!("{}{}", password, salt));

        sqlx::query!(
            r#"update users set password = $1 where id = $2"#,
            password,
            user
        )
        .execute(self.pool_ref())
        .await
        .map_err(|err| {
            error!("Failed to update password: {}", err);
            Status::internal("Failed to update password")
        })?;

        match kind_data {
            Some(Kind::Consumer(DataUserConsumer { location })) => {
                sqlx::query!(
                    r#"insert into consumers (id, location) values ($1, $2)"#,
                    user,
                    location
                )
                .execute(self.pool_ref())
                .await
                .map_err(|err| {
                    error!("Failed to create consumer: {}", err);
                    Status::internal("Failed to create consumer")
                })?;
            }
            Some(Kind::Provider(DataUserProvider { location })) => {
                sqlx::query!(
                    r#"insert into providers (id, location) values ($1, $2)"#,
                    user,
                    location
                )
                .execute(self.pool_ref())
                .await
                .map_err(|err| {
                    error!("Failed to create provider: {}", err);
                    Status::internal("Failed to create provider")
                })?;
            }
            None => {
                sqlx::query!(r#"insert into drivers (id) values ($1)"#, user)
                    .execute(self.pool_ref())
                    .await
                    .map_err(|err| {
                        error!("Failed to create driver: {}", err);
                        Status::internal("Failed to create driver")
                    })?;
            }
        }

        log::info!("User {} created as {:?}", user, kind);

        Ok(Response::new(Empty::default()))
    }

    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<Empty>, Status> {
        let UpdatePasswordRequest { user_id, old, new } = request.into_inner();

        let user = sqlx::query!(r#"select password, salt from users where id = $1"#, user_id)
            .fetch_optional(self.pool_ref())
            .await
            .map_err(|err| {
                error!("Failed to fetch user: {}", err);
                Status::internal("Failed to fetch user")
            })?;

        let Some(user) = user else {
            error!("User {} not found", user_id);
            return Err(Status::not_found("User not found"));
        };

        let old = sha256::digest(format!("{}{}", old, user.salt));
        let new = sha256::digest(format!("{}{}", new, user.salt));

        if old != user.password {
            error!("Invalid password for user {}", user_id);
            return Err(Status::unauthenticated("Invalid password"));
        }

        sqlx::query!(
            r#"update users set password = $1 where id = $2"#,
            new,
            user_id
        )
        .execute(self.pool_ref())
        .await
        .map_err(|err| {
            error!("Failed to update password: {}", err);
            Status::internal("Failed to update password")
        })?;

        log::info!("User {} updated password", user_id);

        Ok(Response::new(Empty::default()))
    }
}
