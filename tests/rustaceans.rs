pub mod common;

use reqwest::StatusCode;
use serde_json::{json, Value};

use common::{
    APP_HOST,
    create_test_rustacean,
    delete_test_rustacean,
    get_client_with_logged_in_admin
};

#[test]
fn test_get_rustaceans() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean1: Value = create_test_rustacean(&client);
    let rustacean2: Value = create_test_rustacean(&client);

    // Test
    let response = client.get(format!("{}/rustaceans", APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    // Cleanup
    delete_test_rustacean(&client, rustacean1);
    delete_test_rustacean(&client, rustacean2);

}

#[test]
fn test_create_rustacean() {
    // Setup
    let client = get_client_with_logged_in_admin();
    // Test
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email" : "foo@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Foo bar",
        "email" : "foo@bar.com",
        "created_at": rustacean["created_at"]
    }));

    //Cleanup
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client.get(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Foo bar",
        "email" : "foo@bar.com",
        "created_at": rustacean["created_at"]
    }));

    // Test Not Found Error
    let response = client.get(format!("{}/rustaceans/{}", APP_HOST, 9999))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Clean up
    delete_test_rustacean(&client, rustacean);
}   

#[test]
fn test_update_rustacean() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client.put(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .json(&json!({
            "name": "Fooz bar",
            "email" : "fooz@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Fooz bar",
        "email" : "fooz@bar.com",
        "created_at": rustacean["created_at"]
    }));

    // Cleanup
    delete_test_rustacean(&client, rustacean);
}  

#[test]
fn test_delete_rustacean() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);

    // Test
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

}