use crate::NodeState;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

type SharedState = Arc<Mutex<NodeState>>;

/// API response for node info
#[derive(Serialize)]
struct NodeInfoResponse {
    address: String,
    public_key: String,
    dag_size: usize,
    dag_depth: u64,
    balance: u64,
    balance_rhz: f64,
    total_relays: u64,
    tips_count: usize,
}

/// API response for balance
#[derive(Serialize)]
struct BalanceResponse {
    address: String,
    balance: u64,
    balance_rhz: f64,
}

/// API request to send a transaction
#[derive(Deserialize)]
struct SendRequest {
    recipient_pubkey_hex: String,
    amount: u64,
}

/// API response for a transaction
#[derive(Serialize)]
struct TransactionResponse {
    id: String,
    status: String,
}

pub async fn run_api_server(state: SharedState, port: u16) {
    let app = Router::new()
        .route("/info", get(get_info))
        .route("/balance", get(get_balance))
        .route("/send", post(send_transaction))
        .route("/relay-reward", post(claim_relay_reward))
        .route("/dag/tips", get(get_tips))
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("🌐 API server listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn get_info(State(state): State<SharedState>) -> Json<NodeInfoResponse> {
    let state = state.lock().unwrap();
    let balance = state.balance();
    Json(NodeInfoResponse {
        address: state.address().to_string(),
        public_key: state.keypair.public_key.to_string(),
        dag_size: state.dag.len(),
        dag_depth: state.dag.depth(),
        balance,
        balance_rhz: balance as f64 / rhiza_core::UNITS_PER_RHZ as f64,
        total_relays: state.relay_tracker.total_relays(),
        tips_count: state.dag.tips().len(),
    })
}

async fn get_balance(State(state): State<SharedState>) -> Json<BalanceResponse> {
    let state = state.lock().unwrap();
    let balance = state.balance();
    Json(BalanceResponse {
        address: state.address().to_string(),
        balance,
        balance_rhz: balance as f64 / rhiza_core::UNITS_PER_RHZ as f64,
    })
}

async fn send_transaction(
    State(state): State<SharedState>,
    Json(req): Json<SendRequest>,
) -> Result<Json<TransactionResponse>, (StatusCode, String)> {
    let pubkey_bytes: [u8; 32] = hex::decode(&req.recipient_pubkey_hex)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid hex: {}", e)))?
        .try_into()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid public key length".to_string()))?;

    let recipient = rhiza_core::crypto::PublicKey::from_bytes(pubkey_bytes);

    let mut state = state.lock().unwrap();
    let tx = state
        .send(recipient, req.amount)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(TransactionResponse {
        id: tx.id.to_string(),
        status: "confirmed".to_string(),
    }))
}

async fn claim_relay_reward(
    State(state): State<SharedState>,
) -> Result<Json<TransactionResponse>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();
    let tx = state
        .claim_relay_reward()
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(TransactionResponse {
        id: tx.id.to_string(),
        status: "confirmed".to_string(),
    }))
}

async fn get_tips(State(state): State<SharedState>) -> Json<Vec<String>> {
    let state = state.lock().unwrap();
    let tips: Vec<String> = state.dag.tips().iter().map(|t| t.to_string()).collect();
    Json(tips)
}
