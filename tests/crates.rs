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

fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client.post(format!("{}/{}", common::APP_HOST, ENDPOINT))
        .json(&json!({
            "name": "Test Crate",
            "code": "pub fn main() { print!('foo'); }",
            "version": "1.0",
            "rustacean_id": rustacean["id"]
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);

    let da_crate = create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/{}/{}", common::APP_HOST, ENDPOINT, da_crate["id"])).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(da_crate, response.json::<Value>().unwrap());
    delete_test_crate(&client, da_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_invalid_crate() {
    let client = Client::new();
    let response = client.get(format!("{}/{}/-1", common::APP_HOST, ENDPOINT)).send().unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean1: Value = common::create_test_rustacean(&client);
    let rustacean2: Value = common::create_test_rustacean(&client);
    let crate1: Value = create_test_crate(&client, &rustacean1);
    let crate2: Value = create_test_crate(&client, &rustacean2);

    let response = client.get(format!("{}/{}",common::APP_HOST, ENDPOINT)).send().unwrap();

    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&crate1));
    assert!(json.as_array().unwrap().contains(&crate2));

    delete_test_crate(&client, crate1);
    delete_test_crate(&client, crate2);

    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);

    let a_crate = create_test_crate(&client, &rustacean);

    let response = client.put(format!("{}/{}/{}", common::APP_HOST, ENDPOINT, a_crate["id"]))
        .json(&json!({
            "name": a_crate["name"],
            "code": a_crate["code"],
            "version": "1.1",
            "rustacean_id": rustacean["id"]
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let updated_crate: Value = response.json().unwrap();
    assert_eq!(updated_crate, json!({
        "id": a_crate["id"],
        "name": "Test Crate",
        "code": "pub fn main() { print!('foo'); }",
        "version": "1.1",
        "description": a_crate["description"],
        "rustacean_id": a_crate["rustacean_id"],
        "created_at": a_crate["created_at"]
    }));
    delete_test_crate(&client, updated_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();

    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);
    let response = client.delete(format!("{}/{}/{}", common::APP_HOST, ENDPOINT, a_crate["id"]))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    common::delete_test_rustacean(&client, rustacean);
}