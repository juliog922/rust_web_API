use reqwest::{
    blocking::{Client, ClientBuilder},
    StatusCode, 
    header};
use serde_json::{
    json, 
    Value};
use std::process::Command;

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email" : "foo@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client.post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": &rustacean["id"],
            "code": "Bar",
            "name": "Foo",
            "version": "Bar",
            "description": "Test description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_crate(client: &Client, crate_a: Value) {
    let response = client.delete(format!("{}/crates/{}", APP_HOST, crate_a["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_client_with_logged_in_client(username: &str, role: &str) -> Client {
    // Setup
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg("1234")
        .arg(role)
        .output()
        .unwrap();

    let client = Client::new();

    // Authorized Test
    let response = client.post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": "1234",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
         header::HeaderValue::from_str(&header_value).unwrap()
        );
    ClientBuilder::new().default_headers(headers).build().unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    get_client_with_logged_in_client("test_viewer", "viewer")
}

pub fn get_client_with_logged_in_admin() -> Client {
    get_client_with_logged_in_client("test_admin", "admin")
}