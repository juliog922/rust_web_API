use crate::{auth::{authorize_user, Credentials},
    repositories::UserRepository,
    routes::{server_error, DbConn, CacheConn}};

use rocket::{http::Status, serde::json::{json, Json, Value}};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use rocket::response::status::Custom;

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db, &credentials.username).await
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => Custom(Status::Unauthorized, json!("Wrong credentials")),
                _ => server_error(e.into()),
            }
        })?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache.set_ex::<String, i32, ()>(
        format!("sessions/{}", session_id),
        user.id,
        3*60*60
    )
    .await
    .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id,
    }))
}
