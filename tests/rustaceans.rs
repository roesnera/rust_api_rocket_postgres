use reqwest::{blocking::Client, StatusCode};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}