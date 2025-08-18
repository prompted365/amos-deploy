use amos_api::{create_app, AppState};
use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;
use uuid::Uuid;

async fn setup_test_server() -> TestServer {
    let state = AppState::test();
    let app = create_app(state);
    TestServer::new(app).expect("Failed to create test server")
}

mod health_tests {
    use super::*;

    #[tokio::test]
    async fn test_health_endpoint() {
        let server = setup_test_server().await;
        
        let response = server.get("/health").await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let json: serde_json::Value = response.json();
        assert_eq!(json["status"], "healthy");
        assert!(json["version"].is_string());
        assert_eq!(json["agents_count"], 0);
        assert_eq!(json["swarms_count"], 0);
        assert_eq!(json["neural_network_active"], true);
    }
}

mod agent_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_list_agents() {
        let server = setup_test_server().await;
        
        // Create an agent
        let create_request = json!({
            "name": "Test Architect",
            "agent_type": "architect",
            "shadow_mode": false
        });
        
        let response = server
            .post("/api/v1/agents")
            .json(&create_request)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::CREATED);
        
        let created_agent: serde_json::Value = response.json();
        assert_eq!(created_agent["name"], "Test Architect");
        assert!(created_agent["id"].is_string());
        
        // List agents
        let response = server.get("/api/v1/agents").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let agents: Vec<serde_json::Value> = response.json();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0]["name"], "Test Architect");
    }

    #[tokio::test]
    async fn test_get_agent_by_id() {
        let server = setup_test_server().await;
        
        // Create an agent
        let create_request = json!({
            "name": "Test Builder",
            "agent_type": "builder",
            "shadow_mode": true
        });
        
        let response = server
            .post("/api/v1/agents")
            .json(&create_request)
            .await;
        
        let created_agent: serde_json::Value = response.json();
        let agent_id = created_agent["id"].as_str().unwrap();
        
        // Get agent by ID
        let response = server.get(&format!("/api/v1/agents/{}", agent_id)).await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let agent: serde_json::Value = response.json();
        assert_eq!(agent["id"], agent_id);
        assert_eq!(agent["name"], "Test Builder");
    }

    #[tokio::test]
    async fn test_delete_agent() {
        let server = setup_test_server().await;
        
        // Create an agent
        let create_request = json!({
            "name": "Test Critic",
            "agent_type": "critic",
            "shadow_mode": false
        });
        
        let response = server
            .post("/api/v1/agents")
            .json(&create_request)
            .await;
        
        let created_agent: serde_json::Value = response.json();
        let agent_id = created_agent["id"].as_str().unwrap();
        
        // Delete agent
        let response = server.delete(&format!("/api/v1/agents/{}", agent_id)).await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
        
        // Verify agent is deleted
        let response = server.get(&format!("/api/v1/agents/{}", agent_id)).await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_agent_command() {
        let server = setup_test_server().await;
        
        // Create an agent
        let create_request = json!({
            "name": "Test Guardian",
            "agent_type": "guardian",
            "shadow_mode": false
        });
        
        let response = server
            .post("/api/v1/agents")
            .json(&create_request)
            .await;
        
        let created_agent: serde_json::Value = response.json();
        let agent_id = created_agent["id"].as_str().unwrap();
        
        // Send command
        let command = json!({
            "command": "start",
            "parameters": null
        });
        
        let response = server
            .post(&format!("/api/v1/agents/{}/command", agent_id))
            .json(&command)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let result: serde_json::Value = response.json();
        assert_eq!(result["status"], "executed");
        assert_eq!(result["agent_id"], agent_id);
    }
}

mod neural_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_neural_state() {
        let server = setup_test_server().await;
        
        let response = server.get("/api/v1/neural/state").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let state: serde_json::Value = response.json();
        assert!(state["total_nodes"].is_number());
        assert!(state["total_pathways"].is_number());
        assert!(state["average_strength"].is_number());
        assert!(state["hormonal_levels"].is_object());
        assert!(state["immune_status"].is_object());
    }

    #[tokio::test]
    async fn test_update_neural_pathway() {
        let server = setup_test_server().await;
        
        let update = json!({
            "from_node": Uuid::new_v4(),
            "to_node": Uuid::new_v4(),
            "strength_delta": 0.1,
            "reason": "Test update"
        });
        
        let response = server
            .post("/api/v1/neural/pathways")
            .json(&update)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let result: serde_json::Value = response.json();
        assert_eq!(result["status"], "updated");
        assert_eq!(result["strength_delta"], 0.1);
        assert_eq!(result["reason"], "Test update");
    }
}

mod swarm_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_list_swarms() {
        let server = setup_test_server().await;
        
        // First create some agents
        let mut agent_ids = vec![];
        for i in 0..3 {
            let create_request = json!({
                "name": format!("Test Agent {}", i),
                "agent_type": "traffic_seer",
                "shadow_mode": false
            });
            
            let response = server
                .post("/api/v1/agents")
                .json(&create_request)
                .await;
            
            let agent: serde_json::Value = response.json();
            agent_ids.push(agent["id"].clone());
        }
        
        // Create a swarm
        let create_swarm = json!({
            "name": "Test Swarm",
            "agent_ids": agent_ids,
            "topology": "mesh"
        });
        
        let response = server
            .post("/api/v1/swarms")
            .json(&create_swarm)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::CREATED);
        
        let swarm: serde_json::Value = response.json();
        assert_eq!(swarm["name"], "Test Swarm");
        assert_eq!(swarm["agent_count"], 3);
        
        // List swarms
        let response = server.get("/api/v1/swarms").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let swarms: Vec<serde_json::Value> = response.json();
        assert_eq!(swarms.len(), 1);
        assert_eq!(swarms[0]["name"], "Test Swarm");
    }

    #[tokio::test]
    async fn test_orchestrate_task() {
        let server = setup_test_server().await;
        
        // Create agents and swarm
        let mut agent_ids = vec![];
        for i in 0..2 {
            let create_request = json!({
                "name": format!("Worker {}", i),
                "agent_type": "optimizer",
                "shadow_mode": false
            });
            
            let response = server
                .post("/api/v1/agents")
                .json(&create_request)
                .await;
            
            let agent: serde_json::Value = response.json();
            agent_ids.push(agent["id"].clone());
        }
        
        let create_swarm = json!({
            "name": "Worker Swarm",
            "agent_ids": agent_ids,
            "topology": "hierarchical"
        });
        
        let response = server
            .post("/api/v1/swarms")
            .json(&create_swarm)
            .await;
        
        let swarm: serde_json::Value = response.json();
        let swarm_id = swarm["id"].as_str().unwrap();
        
        // Orchestrate a task
        let task = json!({
            "task_description": "Test task execution",
            "strategy": "parallel",
            "timeout_seconds": 30,
            "priority": "medium"
        });
        
        let response = server
            .post(&format!("/api/v1/swarms/{}/orchestrate", swarm_id))
            .json(&task)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let result: serde_json::Value = response.json();
        assert!(result["task_id"].is_string());
        assert_eq!(result["status"], "completed");
        assert!(result["execution_time_ms"].is_number());
    }
}