//! The launcher-facing HTTP API. Routing and a health check work; everything else returns the
//! base project's "not implemented" shape.

use axum::{routing::get, routing::post, Json, Router};
use serde_json::{json, Value};

use crate::{auth, mcp, moderation, og};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/auth/token", post(sign_in))
        .route("/mcp/profile/:account_id", get(query_profile))
        .route("/og/*path", get(og_placeholder))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn sign_in(Json(request): Json<auth::SignInRequest>) -> Json<Value> {
    match auth::sign_in(&request) {
        Ok(token) => Json(json!(token)),
        Err(error) => Json(json!({ "error": error })),
    }
}

async fn query_profile(
    axum::extract::Path(account_id): axum::extract::Path<String>,
) -> Json<Value> {
    if moderation::is_banned(&account_id) {
        return Json(json!({ "error": "banned" }));
    }
    match mcp::query_profile(&account_id) {
        Ok(profile) => Json(json!(profile)),
        Err(error) => Json(json!({ "error": error })),
    }
}

async fn og_placeholder() -> Json<Value> {
    Json(og::placeholder_response())
}
