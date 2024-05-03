pub mod common;

use reqwest:: {blocking::Client, StatusCode};
use serde_json::{json, Value};

use common::{
    APP_HOST,
    create_test_rustacean,
    create_test_crate,
    delete_test_rustacean,
    delete_test_crate,
    get_client_with_logged_in_admin,
    get_client_with_logged_in_viewer,
};

#[test]
fn test_get_crates() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);
    let crate_a1: Value = create_test_crate(&client, &rustacean);
    let crate_a2: Value = create_test_crate(&client, &rustacean);

    // Authorized Test
    let client = get_client_with_logged_in_viewer();
    let response = client.get(format!("{}/crates", APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&crate_a1));
    assert!(json.as_array().unwrap().contains(&crate_a2));

    // Unauthorized Test
    let unauthorized_client = Client::new();
    let response = unauthorized_client.get(format!("{}/crates", APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Cleanup
    let client = get_client_with_logged_in_admin();
    delete_test_crate(&client, crate_a1);
    delete_test_crate(&client, crate_a2);
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);

    // Test
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
    
    let crate_a: Value = response.json().unwrap();
    assert_eq!(crate_a, json!({
        "id": crate_a["id"],
        "rustacean_id": &rustacean["id"],
        "code": "Bar",
        "name": "Foo",
        "version": "Bar",
        "description": "Test description",
        "created_at": crate_a["created_at"]
    }));

    //Cleanup
    delete_test_crate(&client, crate_a);
    delete_test_rustacean(&client, rustacean);
    
}

#[test]
fn test_view_crate() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);
    let crate_a: Value = create_test_crate(&client, &rustacean);

    let client = get_client_with_logged_in_viewer();
    let response = client.get(format!("{}/crates/{}", APP_HOST, crate_a["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test
    let crate_a: Value = response.json().unwrap();
    assert_eq!(crate_a, json!({
        "id": crate_a["id"],
        "rustacean_id": &rustacean["id"],
        "code": "Bar",
        "name": "Foo",
        "version": "Bar",
        "description": "Test description",
        "created_at": crate_a["created_at"]
    }));

    // Test Not Found Error
    let response = client.get(format!("{}/crates/{}", APP_HOST, 9999))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Clean up
    let client = get_client_with_logged_in_admin();
    delete_test_crate(&client, crate_a);
    delete_test_rustacean(&client, rustacean);
}   

#[test]
fn test_update_crate() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);
    let crate_a: Value = create_test_crate(&client, &rustacean);

    // Test
    let response = client.put(format!("{}/crates/{}", APP_HOST, crate_a["id"]))
        .json(&json!({
            "rustacean_id": &rustacean["id"],
            "code": "Barz",
            "name": "Fooz",
            "version": "Barz",
            "description": "Test description Updated"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let crate_a: Value = response.json().unwrap();
    assert_eq!(crate_a, json!({
        "id": crate_a["id"],
        "rustacean_id": &rustacean["id"],
        "code": "Barz",
        "name": "Fooz",
        "version": "Barz",
        "description": "Test description Updated",
        "created_at": crate_a["created_at"]
    }));

    // Test author-switching for a crate
    let response = client.put(format!("{}/crates/{}", APP_HOST, crate_a["id"]))
        .json(&json!({
            "rustacean_id": 99999,
            "code": "Barz",
            "name": "Fooz",
            "version": "Barz",
            "description": "Test description Updated"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    // Cleanup
    delete_test_crate(&client, crate_a);
    delete_test_rustacean(&client, rustacean);
}  

#[test]
fn test_delete_crate() {
    // Setup
    let client = get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);
    let crate_a: Value = create_test_crate(&client, &rustacean);

    // Test
    let response = client.delete(format!("{}/crates/{}", APP_HOST, crate_a["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    delete_test_rustacean(&client, rustacean);
}