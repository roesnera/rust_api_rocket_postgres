use std::process::Command;

use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

static ENDPOINT: &'static str = "login";
#[test]
fn test_login() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output();

    println!("{:?}", output);
    let client = Client::new();
    
    
    let response = client.post(format!("{}/{}", common::APP_HOST, ENDPOINT))
        .json(&json!({
            "username": "test_admin",
            "password": "1234"
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);
}


#[test]
fn test_login_wrong_password() {
    let client = Client::new();

    let response = client.post(format!("{}/{}", common::APP_HOST, ENDPOINT))
    .json(&json!({
        "username": "test_admin",
        "password": "12345"
    }))
    .send().unwrap();
assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_login_wrong_username() {
    let client = Client::new();

    let response = client.post(format!("{}/{}", common::APP_HOST, ENDPOINT))
    .json(&json!({
        "username": "test_admi",
        "password": "1234"
    }))
    .send().unwrap();
assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn _login_test_user(client: &Client) -> Value {
    let response = client.post(format!("{}/{}", common::APP_HOST, ENDPOINT))
        .json(&json!({
            "username": "test_admin",
            "password": "1234"
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response.json().unwrap()
}