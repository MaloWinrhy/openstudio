
use std::future;
use std::pin;
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_jwt_auth_flow() {
    let client = Client::new();
    // 1. Créer un utilisateur
    let res = client.post("http://localhost:3000/users")
        .json(&json!({"username": "bob", "email": "bob@example.com"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 201);

    // 2. Login pour obtenir un JWT
    let res = client.post("http://localhost:3000/login")
        .json(&json!({"username": "bob", "email": "bob@example.com"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    let token = body["token"].as_str().unwrap();

    // 3. Appeler une route protégée (ex: /users) avec Authorization: Bearer <token>
    let res = client.get("http://localhost:3000/users")
        .bearer_auth(token)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
}
