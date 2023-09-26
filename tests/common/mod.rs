use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Test rustacean",
            "email": "test@email.com"
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}