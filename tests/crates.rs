use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

static ENDPOINT: &'static str = "crates";

#[test]
fn test_create_crate() {
    let client = Client::new();
    
    let response = client.post(format!("{}/{}", common::APP_HOST, ENDPOINT))
        .json(&json!({
            "name": "Test rustacean",
            "code": "pub fn main() { }",
            "version": "1.0",
            "rustacean_id": 2
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    delete_test_crate(&client, response.json().unwrap())
}

fn delete_test_crate(client: &Client, da_crate: Value) {
    let response = client.delete(format!("{}/{}/{}",common::APP_HOST, ENDPOINT, da_crate["id"]))
    .send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}