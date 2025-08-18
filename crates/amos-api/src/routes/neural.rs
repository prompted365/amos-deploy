use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use crate::{
    models::neural::{NeuralState, PathwayUpdate, HormonalLevels, ImmuneStatus},
    ApiResult, AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/neural/state", get(get_neural_state))
        .route("/neural/pathways", post(update_neural_pathway))
}

#[utoipa::path(
    get,
    path = "/api/v1/neural/state",
    responses(
        (status = 200, description = "Get neural network state", body = NeuralState),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "neural",
)]
pub async fn get_neural_state(State(state): State<AppState>) -> ApiResult<Json<NeuralState>> {
    let neural_network = &state.neural_network;
    
    // Get node and pathway counts
    let total_nodes = neural_network.node_count().await;
    let total_pathways = neural_network.pathway_count().await;
    
    let neural_state = NeuralState {
        total_nodes,
        total_pathways,
        active_pathways: (total_pathways as f64 * 0.7) as usize, // Estimate active pathways
        average_strength: 0.65, // Default average strength
        hormonal_levels: HormonalLevels {
            dopamine: 0.7,
            serotonin: 0.6,
            cortisol: 0.3,
            oxytocin: 0.5,
        },
        immune_status: ImmuneStatus {
            health: 0.95,
            threats_detected: 0,
            patterns_remembered: 42,
        },
    };
    
    Ok(Json(neural_state))
}

#[utoipa::path(
    post,
    path = "/api/v1/neural/pathways",
    request_body = PathwayUpdate,
    responses(
        (status = 200, description = "Pathway updated"),
        (status = 400, description = "Invalid pathway update"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "neural",
)]
pub async fn update_neural_pathway(
    State(state): State<AppState>,
    Json(update): Json<PathwayUpdate>,
) -> ApiResult<Json<serde_json::Value>> {
    let neural_network = &state.neural_network;
    
    // Create a new pathway with the updated strength
    // In a real implementation, we would update the existing pathway
    let new_strength = 0.5 + update.strength_delta; // Base strength + delta
    let pathway_id = neural_network.create_pathway(
        update.from_node,
        update.to_node,
        new_strength,
    ).await;
    
    Ok(Json(serde_json::json!({
        "status": "updated",
        "pathway_id": pathway_id,
        "from_node": update.from_node,
        "to_node": update.to_node,
        "strength_delta": update.strength_delta,
        "new_strength": new_strength,
        "reason": update.reason,
    })))
}