use amos_api::{create_app, AppState};
use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_health_endpoint() {
    let app = create_app(AppState::test());
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_auth_login() {
    let app = create_app(AppState::test());
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": "admin",
            "password": "amos123"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body = response.json::<serde_json::Value>();
    assert!(body.get("token").is_some());
}

#[tokio::test]
async fn test_unauthorized_access() {
    let app = create_app(AppState::test());
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/v1/agents").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_agent_crud_with_auth() {
    let app = create_app(AppState::test());
    let server = TestServer::new(app).unwrap();

    // Login first
    let login_response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": "admin",
            "password": "amos123"
        }))
        .await;
    
    let token = login_response.json::<serde_json::Value>()
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // List agents (should be empty)
    let response = server
        .get("/api/v1/agents")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let agents = response.json::<Vec<serde_json::Value>>();
    assert_eq!(agents.len(), 0);

    // Create an agent
    let create_response = server
        .post("/api/v1/agents")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "Test Agent",
            "agent_type": "traffic_seer",
            "shadow_mode": false
        }))
        .await;
    
    assert_eq!(create_response.status_code(), StatusCode::OK);
    let created_agent = create_response.json::<serde_json::Value>();
    let agent_id = created_agent.get("id").unwrap().as_str().unwrap();

    // Get the agent
    let get_response = server
        .get(&format!("/api/v1/agents/{}", agent_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    
    assert_eq!(get_response.status_code(), StatusCode::OK);

    // Delete the agent
    let delete_response = server
        .delete(&format!("/api/v1/agents/{}", agent_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    
    assert_eq!(delete_response.status_code(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_neural_state() {
    let app = create_app(AppState::test());
    let server = TestServer::new(app).unwrap();

    // Login first
    let login_response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": "admin",
            "password": "amos123"
        }))
        .await;
    
    let token = login_response.json::<serde_json::Value>()
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // Get neural state
    let response = server
        .get("/api/v1/neural/state")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let state = response.json::<serde_json::Value>();
    assert!(state.get("total_nodes").is_some());
    assert!(state.get("total_pathways").is_some());
    assert!(state.get("hormonal_levels").is_some());
}