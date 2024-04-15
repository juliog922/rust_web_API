use crate::repositories::CrateRepository;
use crate::routes::DbConn;
use crate::models::{NewCrate, Crate};
use rocket::serde::json::{json, Value, Json};
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;


#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    CrateRepository::find_multiple(&mut db, 100).await
        .map(|crates| json!(crates))
        .map_err(| _ | Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    CrateRepository::find(&mut db, id).await
        .map(|a_crate| json!(a_crate))
        .map_err(| _ | Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>>{
    CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(| _ | Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/crates/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, a_crate: Json<Crate>) -> Result<Value, Custom<Value>>{
    CrateRepository::update(&mut db, id, a_crate.into_inner()).await
        .map(|a_crate| json!(a_crate))
        .map_err(| _ | Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
    CrateRepository::delete(&mut db, id).await
        .map(| _ | NoContent)
        .map_err(| _ | Custom(Status::InternalServerError, json!("Error")))
}