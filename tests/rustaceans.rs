use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Foo bar",
            "email" : "foo@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans() {
    // Setup
    let client = Client::new();
    let rustacean1: Value = create_test_rustacean(&client);
    let rustacean2: Value = create_test_rustacean(&client);

    // Test
    let response = client.get("http://127.0.0.1:8000/rustaceans")
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
    let client = Client::new();
    // Test
    let response = client.post("http://127.0.0.1:8000/rustaceans")
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
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client.get(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
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

    // Clean up
    delete_test_rustacean(&client, rustacean);
}   

#[test]
fn test_update_rustacean() {
    // Setup
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client.put(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
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
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

}