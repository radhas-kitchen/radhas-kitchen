use super::error::*;
use super::model::*;
use crate::prelude::*;

#[post("/user")]
async fn post(
    pool: Data<Pool<Postgres>>,
    json: Json<NewUser>,
) -> Result<impl Responder, UserError> {
    let NewUser {
        email,
        password,
        name,
        kind,
    } = json.into_inner();

    let user = sqlx::query!(
        r#"insert into users (email,password,name,kind) values ($1,'_',$2,$3) returning salt"#,
        email,
        name,
        kind as UserKind,
    )
    .fetch_one(pool.get_ref())
    .await?;

    let salt = user.salt;
    let prehash = format!("{password}{salt}");
    let hashed = sha256::digest(prehash.as_bytes());

    sqlx::query!(
        r#"update users set password = $1 where email = $2"#,
        hashed,
        email,
    )
    .execute(pool.get_ref())
    .await?;

    return Ok(HttpResponse::NoContent());
}
