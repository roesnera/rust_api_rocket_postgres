use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

static ENDPOINT: &'static str = "rustaceans";

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    let response = client.get(format!("{}/{}", common::APP_HOST, ENDPOINT)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}
#[test]
fn test_create_rustacean() {
    let client = Client::new();

    let rustacean: Value = common::create_test_rustacean(&client);
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Test rustacean",
        "email": "test@email.com",
        "created_at": rustacean["created_at"]
    }));
    
    common::delete_test_rustacean(&client, rustacean);
}
#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client.put(format!("{}/{}/{}", common::APP_HOST, ENDPOINT, rustacean["id"]))
        .json(&json!({
            "name": "Another Test Rustacean",
            "email": "test@example.com",
        })).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Another Test Rustacean",
        "email": "test@example.com",
        "created_at": rustacean["created_at"],
    }));
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);

    let saved_rustacean: Value = client.get(format!("{}/{}/{}", common::APP_HOST, ENDPOINT, rustacean["id"])).send().unwrap().json().unwrap();
    assert_eq!(rustacean, saved_rustacean);
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    
    let response = client.delete(format!("{}/{}/{}", common::APP_HOST, ENDPOINT, rustacean["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}