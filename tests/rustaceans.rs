use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Test Rustacean",
            "email": "test@example.com",
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Test Rustacean",
        "email": "test@example.com",
        "created_at": rustacean["created_at"]
    }));
}
#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let response = client.put("http://127.0.0.1:8000/rustaceans/1")
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
}
// #[test]
// fn test_delete_rustacean() {
//     let client = Client::new();
//     let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
//     assert_eq!(response.status(), StatusCode::OK);
// }