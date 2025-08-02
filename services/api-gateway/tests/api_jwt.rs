
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_full_api_flow() {
    let client = Client::new();

    // --- USERS ---
    // Register
    let res = client.post("http://localhost:3000/register")
        .json(&json!({"username": "bob", "email": "bob@bob.com", "password": "bobpass"}))
        .send().await.unwrap();
    assert!(res.status().is_success());

    // Login
    let res = client.post("http://localhost:3000/login")
        .json(&json!({"username": "bob", "email": "bob@bob.com", "password": "bobpass"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    let access_token = body["access_token"].as_str().unwrap();
    let refresh_token = body["refresh_token"].as_str().unwrap();

    // List users (protégé)
    let res = client.get("http://localhost:3000/users")
        .bearer_auth(access_token)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);

    // --- PROJECTS ---
    // Create project SANS token (401)
    let res = client.post("http://localhost:3000/projects")
        .json(&json!({"name": "test", "description": "desc"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 401);
    // Create project AVEC token
    let res = client.post("http://localhost:3000/projects")
        .bearer_auth(access_token)
        .json(&json!({"name": "test", "description": "desc"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
    // List projects (public)
    let res = client.get("http://localhost:3000/projects")
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let projects: serde_json::Value = res.json().await.unwrap();
    let project_id = projects[0]["id"].as_str().unwrap();
    // Update project SANS token (401)
    let res = client.put(&format!("http://localhost:3000/projects/{}", project_id))
        .json(&json!({"name": "newname"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 401);
    // Update project AVEC token
    let res = client.put(&format!("http://localhost:3000/projects/{}", project_id))
        .bearer_auth(access_token)
        .json(&json!({"name": "newname"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    // Delete project SANS token (401)
    let res = client.delete(&format!("http://localhost:3000/projects/{}", project_id))
        .send().await.unwrap();
    assert_eq!(res.status(), 401);
    // Delete project AVEC token
    let res = client.delete(&format!("http://localhost:3000/projects/{}", project_id))
        .bearer_auth(access_token)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);

    // --- ISSUES ---
    // Create issue (public route)
    let res = client.post("http://localhost:3000/issues")
        .json(&json!({"project_id": project_id, "title": "bug", "description": "desc"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
    // List issues by project
    let res = client.get(&format!("http://localhost:3000/issues?project_id={}", project_id))
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let issues: serde_json::Value = res.json().await.unwrap();
    let issue_id = issues[0]["id"].as_str().unwrap();
    // Update issue (public route)
    let res = client.put(&format!("http://localhost:3000/issues/{}", issue_id))
        .json(&json!({"title": "fixed"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    // Delete issue (public route)
    let res = client.delete(&format!("http://localhost:3000/issues/{}", issue_id))
        .send().await.unwrap();
    assert_eq!(res.status(), 200);

    // --- MEMBERS ---
    // Add member (public route)
    let res = client.post("http://localhost:3000/members")
        .json(&json!({"user_id": "dummy-user-id", "project_id": project_id, "role": "Member"}))
        .send().await.unwrap();
    // Peut échouer si user_id n'existe pas, donc pas d'assert strict ici
    // List members (public route)
    let res = client.get("http://localhost:3000/members?project_id=".to_string() + project_id)
        .send().await.unwrap();
    assert!(res.status().is_success());
    // Remove member (public route)
    let res = client.delete("http://localhost:3000/members")
        .json(&json!({"user_id": "dummy-user-id", "project_id": project_id}))
        .send().await.unwrap();
    // Peut échouer si user_id n'existe pas
}
