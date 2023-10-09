use std::process::Command;

use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

use crate::common::LOGIN_ENDPOINT;

pub mod common;

// static ENDPOINT: &'static str = "login";

// tests login endpoint with valid credentials
#[test]
fn test_login_valid_credentials() {
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
    
    
    let response = client.post(format!("{}/{}", common::APP_HOST, common::LOGIN_ENDPOINT))
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


// tests login endpoint with valid username and invalid password
#[test]
fn test_login_wrong_password() {
    let client = Client::new();

    let response = client.post(format!("{}/{}", common::APP_HOST, LOGIN_ENDPOINT))
    .json(&json!({
        "username": "test_admin",
        "password": "12345"
    }))
    .send().unwrap();
assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// tests login endpoint with invalid username and valid password
/* 
    **EXPECTED BEHAVIOR**
    Action: makes a login attempt to the right endpoint that is properly formatted
        login credentials have an invalid username and a valid password for a diferent user
    Result: endpoint returns a status of 401
     
*/
#[test]
fn test_login_wrong_username() {
    let client = Client::new();

    let response = client.post(format!("{}/{}", common::APP_HOST, LOGIN_ENDPOINT))
    .json(&json!({
        "username": "test_admi",
        "password": "1234"
    }))
    .send().unwrap();
assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn _login_test_user(client: &Client) -> Value {
    let response = client.post(format!("{}/{}", common::APP_HOST, LOGIN_ENDPOINT))
        .json(&json!({
            "username": "test_admin",
            "password": "1234"
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response.json().unwrap()
}