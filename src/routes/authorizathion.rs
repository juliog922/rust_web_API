use crate::{auth::{authorize_user, Credentials},
    repositories::UserRepository,
    routes::{server_error, DbConn}};

use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use rocket::response::status::Custom;




#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username).await
        .map(|user| {
            if let Ok(token) = authorize_user(&user, credentials.into_inner()) {
                return json!(token);
            }
            json!("Unanthorized")            
        })
        .map_err(|e| server_error(e.into()))
}