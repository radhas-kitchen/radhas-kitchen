use super::error::*;
use super::model::*;
use crate::prelude::*;

#[post("/auth")]
async fn post(
    pool: Data<Pool<Postgres>>,
    json: Json<LoginForm>,
) -> Result<impl Responder, AuthError> {
    let LoginForm { email, password } = json.into_inner();

    let AuthUser {
        id: user_id,
        password: passhash,
        salt,
        kind,
    } = sqlx::query_as!(
        AuthUser,
        r#"select id,password,salt,kind as "kind!: _" from users where email = $1"#,
        email
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AuthError::UserNotFound)?;

    let prehash = format!("{password}{salt}");
    let hash = sha256::digest(prehash.as_bytes());

    if hash != passhash {
        return Err(AuthError::BadPassword);
    }

    let mut tokens = sqlx::query_as!(
        PartialToken,
        r#"select user_id,token,expires from tokens where expires > now() and user_id = $1"#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await?;

    let token = if let Some(PartialToken { token, expires, .. }) = tokens.pop() {
        LoginResponse {
            user_id,
            token,
            expires,
            kind,
        }
    } else {
        sqlx::query!(
            r#"delete from tokens where expires < now() and user_id = $1"#,
            user_id
        )
        .execute(pool.get_ref())
        .await?;

        let PartialToken { token, expires, .. } = sqlx::query_as!(
            PartialToken,
            r#"insert into tokens (user_id) values ($1) returning token,expires,user_id"#,
            user_id
        )
        .fetch_one(pool.get_ref())
        .await?;

        LoginResponse {
            user_id,
            token,
            expires,
            kind,
        }
    };

    return Ok(HttpResponse::Ok().json(token));
}

#[patch("/auth")]
async fn patch(
    pool: Data<Pool<Postgres>>,
    json: Json<UpdatePassword>,
) -> Result<impl Responder, AuthError> {
    let UpdatePassword { user_id, old, new } = json.into_inner();

    let Some(PartialUser { password, salt, .. }) = sqlx::query_as!(
        PartialUser,
        r#"select email,password,salt,kind as "kind!: _" from users where id = $1"#,
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await?
    else {
        return Err(AuthError::UserNotFound);
    };

    let prehash = format!("{old}{salt}");
    let hash = sha256::digest(prehash.as_bytes());

    if hash != password {
        return Err(AuthError::BadPassword);
    }

    let prehash = format!("{new}{salt}");
    let newhash = sha256::digest(prehash.as_bytes());

    sqlx::query!(
        r#"update users set password = $1 where id = $2"#,
        newhash,
        user_id,
    )
    .execute(pool.get_ref())
    .await?;

    sqlx::query!(r#"delete from tokens where user_id = $1"#, user_id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
