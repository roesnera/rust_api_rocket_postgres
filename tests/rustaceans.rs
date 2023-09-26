use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
    delete_test_rustacean(&client, rustacean1);
    delete_test_rustacean(&client, rustacean2);
}
#[test]
fn test_create_rustacean() {
    let client = Client::new();

    let rustacean: Value = create_test_rustacean(&client);
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Test rustacean",
        "email": "test@email.com",
        "created_at": rustacean["created_at"]
    }));
    
    delete_test_rustacean(&client, rustacean);
}
#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client.put(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
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
    delete_test_rustacean(&client, rustacean);
}

fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Test rustacean",
            "email": "test@email.com"
        }))
        .send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let saved_rustacean: Value = client.get(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"])).send().unwrap().json().unwrap();
    assert_eq!(rustacean, saved_rustacean);
    delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);
    
    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}