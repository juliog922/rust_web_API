use std::str::FromStr;
use chrono::{Datelike, Utc};
use diesel_async::{AsyncPgConnection, AsyncConnection};
use lettre::{message::{header::ContentType, MessageBuilder}, transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use tera::{Tera, Context};

use crate::{
    auth::hash_password, models::{NewUser, RoleCode}, repositories::{CrateRepository, RoleRepository, UserRepository}
};

fn load_template_engine() -> Tera{
    Tera::new("templates/**/*.html")
        .expect("Cannot load template engine")
}

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

pub async fn digest_send(email: String, hours_since: i32) {
    let mut c = load_db_connection().await;
    let tera = load_template_engine();

    let crates = CrateRepository::find_since(&mut c, hours_since).await.unwrap();
    if crates.len() > 0 {
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);
        let html_body = tera.render("email/digest.html", &context).unwrap();

        let message = MessageBuilder::new()
            .subject("Cr8s digest")
            .from("Cr8s <noreply@cr8s.com".parse().unwrap())
            .to(email.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();

        let smtp_host = std::env::var("SMTP_URL")
            .expect("Connot load SMTP host from enviroment");
        let smtp_username = std::env::var("SMTP_USERNAME")
            .expect("Connot load SMTP usernmae from enviroment");
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .expect("Connot load SMTP password from enviroment");

        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&smtp_host)
            .unwrap()
            .credentials(credentials)
            .build();
        mailer.send(&message).unwrap();
    }
}