use std::str::FromStr;

use diesel_async::{AsyncPgConnection, AsyncConnection};

use crate::{
    auth::hash_password, models::{NewUser, RoleCode}, repositories::{RoleRepository, UserRepository}};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Connot load DB url from enviroment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Connot connect to Postgres")

}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let password_hash = hash_password(password).unwrap();

    let new_user = NewUser {username, password: password_hash.to_string()};
    let role_enums = role_codes.iter().map(|v| RoleCode::from_str(v.as_str()).unwrap()).collect();
    let user = UserRepository::create(&mut c, new_user, role_enums).await.unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_users(id: i32) {
    let mut c = load_db_connection().await;
    UserRepository::delete(&mut c, id).await.unwrap();
} 