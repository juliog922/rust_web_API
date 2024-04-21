use diesel_async::{AsyncPgConnection, AsyncConnection};

use crate::{models::NewUser, repositories::{UserRepository, RoleRepository}};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Connot load DB url from enviroment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Connot connect to Postgres")

}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let new_user = NewUser {username, password};
    let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;
}

pub async fn delete_users(id: i32) {
    let mut c = load_db_connection().await;
} 